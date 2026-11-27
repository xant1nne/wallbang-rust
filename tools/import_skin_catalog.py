"""Generate the embedded econ catalog from a local Valve items_game.txt."""
import argparse
import hashlib
import json
import re
import struct
from pathlib import Path


def parse(text, root_name='items_game'):
    tokens = iter(re.findall(r'"((?:\\.|[^"\\])*)"|([{}])|(?://[^\n]*)', text))

    def block(nested=False):
        result = {}
        for value, brace in tokens:
            if brace == '}':
                if not nested:
                    raise ValueError('unexpected closing brace')
                return result
            if brace == '{':
                raise ValueError('expected key')
            if not value:
                continue
            key = value
            value, brace = next(tokens)
            value = block(True) if brace == '{' else value
            # Valve repeats top-level sections for subsequent content updates.
            # Merge sections; replacing one silently loses most of the catalog.
            if isinstance(value, dict) and isinstance(result.get(key), dict):
                result[key].update(value)
            else:
                result[key] = value
        if nested:
            raise ValueError('unterminated object')
        return result
    return block()[root_name]


def read_source(source, target='scripts/items/items_game.txt'):
    if source.suffix.lower() != '.vpk':
        return source.read_bytes()
    with source.open('rb') as file:
        magic, version, size = struct.unpack('<III', file.read(12))
        if magic != 0x55AA1234 or version not in (1, 2):
            raise ValueError('unsupported VPK header')
        header_size = 28 if version == 2 else 12
        file.seek(header_size)
        tree = file.read(size)
        cursor = 0

        def string():
            nonlocal cursor
            end = tree.index(b'\0', cursor)
            value = tree[cursor:end].decode('utf-8')
            cursor = end + 1
            return value

        while extension := string():
            while folder := string():
                while name := string():
                    _, preload, archive, offset, length, terminator = struct.unpack_from('<IHHIIH', tree, cursor)
                    cursor += 18
                    prefix = tree[cursor:cursor + preload]
                    cursor += preload
                    if terminator != 0xFFFF:
                        raise ValueError('invalid VPK entry')
                    if f'{folder}/{name}.{extension}' != target:
                        continue
                    archive_file = source if archive == 0x7FFF else source.with_name(source.stem[:-4] + f'_{archive:03}.vpk')
                    with archive_file.open('rb') as data_file:
                        data_file.seek(offset + (header_size + size if archive == 0x7FFF else 0))
                        return prefix + data_file.read(length)
    raise ValueError(f'{target} is missing from VPK')


def vpk_paths(source):
    with source.open('rb') as file:
        _, version, size = struct.unpack('<III', file.read(12))
        file.seek(28 if version == 2 else 12)
        tree = file.read(size)
    cursor = 0

    def string():
        nonlocal cursor
        end = tree.index(b'\0', cursor)
        value = tree[cursor:end].decode('utf-8')
        cursor = end + 1
        return value

    while extension := string():
        while folder := string():
            while name := string():
                preload = struct.unpack_from('<H', tree, cursor + 4)[0]
                cursor += 18 + preload
                yield f'{folder}/{name}.{extension}'


def generate(source, destination):
    data = read_source(source)
    root = parse(data.decode('utf-8-sig'))
    tokens = {}
    if source.suffix.lower() == '.vpk':
        try:
            english = read_source(source, 'resource/csgo_english.txt')
            english = english.decode('utf-16' if english.startswith((b'\xff\xfe', b'\xfe\xff')) else 'utf-8-sig')
            tokens = {key.lower(): value for key, value in parse(english, 'lang')['Tokens'].items()}
        except (ValueError, KeyError):
            pass
    prefabs = root.get('prefabs', {})

    def inherited(item, depth=0):
        if depth > 20:
            raise ValueError('prefab cycle')
        values = {}
        for parent in item.get('prefab', '').split():
            values.update(inherited(prefabs.get(parent, {}), depth + 1))
        values.update(item)
        return values

    kits = []
    for key, kit in root['paint_kits'].items():
        if not key.isdecimal() or not isinstance(kit, dict):
            continue
        kits.append((int(key), kit.get('name', key), kit.get('use_legacy_model', '0') == '1'))
    kit_ids = {name: ident for ident, name, _ in kits}
    item_ids = {item.get('name'): int(key) for key, item in root['items'].items()
                if key.isdecimal() and isinstance(item, dict)}
    pairs = set()
    for paint, weapon in re.findall(r'\[([^\]]+)\]([A-Za-z0-9_]+)', data.decode('utf-8-sig')):
        if paint in kit_ids and weapon in item_ids and item_ids[weapon] <= 32767:
            pairs.add((item_ids[weapon], kit_ids[paint]))
    if source.suffix.lower() == '.vpk':
        weapons = sorted((name for name in item_ids if name), key=len, reverse=True)
        for path in vpk_paths(source):
            if '/econ/default_generated/' not in path or not path.endswith('_light_png.vtex_c'):
                continue
            name = path.rsplit('/', 1)[1][:-len('_light_png.vtex_c')]
            for weapon in weapons:
                if name.startswith(weapon + '_'):
                    paint = name[len(weapon) + 1:]
                    if paint in kit_ids and item_ids[weapon] <= 32767:
                        pairs.add((item_ids[weapon], kit_ids[paint]))
                    break
    models = []
    for key, item in root['items'].items():
        if not key.isdecimal() or not isinstance(item, dict):
            continue
        item = inherited(item)
        model = item.get('model_player', '')
        if model:
            models.append((int(key), model))
    quote = lambda value: json.dumps(value, ensure_ascii=False)
    lines = ['// Config block: generated by tools/import_skin_catalog.py; do not hand-edit.',
             '// Source SHA256: ' + hashlib.sha256(data).hexdigest(),
             '#[derive(Clone, Copy, Debug)]',
             'pub struct PaintKit { pub id: u32, pub name: &\'static str, pub legacy_model: bool }',
             'pub const PAINT_CATALOG: &[PaintKit] = &[']
    for ident, name, legacy in sorted(kits):
        tag = root['paint_kits'][str(ident)].get('description_tag', '').lstrip('#').lower()
        display = tokens.get(tag, name)
        lines.append(f'    PaintKit {{ id: {ident}, name: {quote(display)}, legacy_model: {str(legacy).lower()} }},')
    lines.extend(['];', 'pub const ITEM_MODELS: &[(i16, &str)] = &['])
    for ident, model in sorted(models):
        if ident <= 32767:
            lines.append(f'    ({ident}, {quote(model)}),')
    lines.extend(['];', 'pub const ITEM_PAINTS: &[(i16, u32)] = &['])
    for definition, paint in sorted(pairs):
        lines.append(f'    ({definition}, {paint}),')
    lines.extend(['];', '',
        'pub fn paint_kit(id: u32) -> Option<&\'static PaintKit> {',
        '    PAINT_CATALOG.binary_search_by_key(&id, |kit| kit.id).ok().map(|i| &PAINT_CATALOG[i])', '}',
        'pub fn model(definition: i16) -> Option<&\'static str> {',
        '    ITEM_MODELS.binary_search_by_key(&definition, |item| item.0).ok().map(|i| ITEM_MODELS[i].1)', '}',
        'pub fn compatible(definition: i16, paint: u32) -> bool {',
        '    paint == 0 || ITEM_PAINTS.binary_search(&(definition, paint)).is_ok()', '}', ''])
    destination.write_text('\n'.join(lines), encoding='utf-8')
    print(f'{len(kits)} paint kits; {len(models)} item models; {len(pairs)} item/paint pairs -> {destination}')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('source', type=Path)
    parser.add_argument('destination', type=Path)
    args = parser.parse_args()
    generate(args.source, args.destination)
