// Offline import only. Runtime reads the embedded PNG pack, never a VPK.
import fs from 'node:fs';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';

const root=path.resolve(import.meta.dirname,'..');
const args=process.argv.slice(2);
const option=(name,fallback)=>{const i=args.indexOf(name);return i<0?fallback:args[i+1];};
const game=option('--game',String.raw`C:\Program Files (x86)\Steam\steamapps\common\Counter-Strike Global Offensive\game\csgo`);
const vrf=option('--vrf',String.raw`C:\wb_toolchain\tools\vendor\vrf-21.1\Source2Viewer-CLI.exe`);
const source=path.join(game,'pak01_dir.vpk');
const work=path.join(root,'target/tmp/skin-icon-import');
const rawDir=path.join(work,'raw'),pngDir=path.join(work,'png');
fs.mkdirSync(rawDir,{recursive:true});fs.mkdirSync(pngDir,{recursive:true});

function archiveIndex(file) {
    const fd=fs.openSync(file,'r'),header=Buffer.alloc(28);
    fs.readSync(fd,header);const version=header.readUInt32LE(4);
    if(header.readUInt32LE(0)!==0x55aa1234 || ![1,2].includes(version))throw Error('Unsupported VPK');
    const headerSize=version===2?28:12,tree=Buffer.alloc(header.readUInt32LE(8));
    fs.readSync(fd,tree,0,tree.length,headerSize);fs.closeSync(fd);
    let cursor=0;const entries=new Map();
    const string=()=>{const end=tree.indexOf(0,cursor);if(end<0)throw Error('Invalid VPK tree');const value=tree.toString('utf8',cursor,end);cursor=end+1;return value;};
    for(let ext;(ext=string());)for(let folder;(folder=string());)for(let name;(name=string());){
        const preload=tree.readUInt16LE(cursor+4),archive=tree.readUInt16LE(cursor+6),offset=tree.readUInt32LE(cursor+8),length=tree.readUInt32LE(cursor+12);
        if(tree.readUInt16LE(cursor+16)!==0xffff)throw Error('Invalid VPK terminator');
        cursor+=18;const prefix=tree.subarray(cursor,cursor+preload);cursor+=preload;
        entries.set(`${folder}/${name}.${ext}`.toLowerCase(),{archive,offset,length,prefix});
    }
    const handles=new Map();
    const read=(name)=>{
        const entry=entries.get(name.toLowerCase());if(!entry)return null;
        const archiveFile=entry.archive===0x7fff?file:file.replace(/_dir\.vpk$/i,`_${String(entry.archive).padStart(3,'0')}.vpk`);
        if(!handles.has(archiveFile))handles.set(archiveFile,fs.openSync(archiveFile,'r'));
        const bytes=Buffer.alloc(entry.length);let read=0;
        const start=entry.offset+(entry.archive===0x7fff?headerSize+tree.length:0);
        while(read<bytes.length){const n=fs.readSync(handles.get(archiveFile),bytes,read,bytes.length-read,start+read);if(n===0)throw Error(`Truncated VPK: ${name}`);read+=n;}
        return Buffer.concat([entry.prefix,bytes]);
    };
    return {entries,read,close:()=>{for(const fd of handles.values())fs.closeSync(fd);}};
}
function keyValues(text) {
    const tokens=[...text.matchAll(/"((?:\\.|[^"\\])*)"|([{}])|\/\/[^\n]*/g)].filter(m=>m[1]!==undefined||m[2]).map(m=>m[2]||m[1].replace(/\\"/g,'"').replace(/\\\\/g,'\\'));
    let i=0;
    function object(nested=false){
        const result=Object.create(null);
        while(i<tokens.length){
            const key=tokens[i++];if(key==='}'){if(!nested)throw Error('Unexpected KV close');return result;}
            if(key==='{')throw Error('Unexpected KV open');
            let value=tokens[i++];if(value==='{')value=object(true);
            if(value===undefined)throw Error('Truncated KV');
            if(typeof value==='object'&&typeof result[key]==='object')Object.assign(result[key],value);else result[key]=value;
        }
        if(nested)throw Error('Unclosed KV object');return result;
    }
    return object().items_game;
}

const archive=archiveIndex(source);
try {
    const itemsBytes=archive.read('scripts/items/items_game.txt');if(!itemsBytes)throw Error('items_game missing');
    const data=keyValues(itemsBytes.toString('utf8'));
    function inherited(item,depth=0){
        if(depth>32)throw Error('Prefab cycle');let result={};
        for(const parent of String(item.prefab||'').split(/\s+/).filter(Boolean))Object.assign(result,inherited(data.prefabs[parent]||{},depth+1));
        return Object.assign(result,item);
    }
    const catalog=fs.readFileSync(path.join(root,'config/src/skin_catalog.rs'),'utf8');
    const pairs=[...catalog.slice(catalog.indexOf('pub const ITEM_PAINTS'),catalog.indexOf('pub fn paint_kit')).matchAll(/\((\d+),\s*(\d+)\)/g)].map(m=>[Number(m[1]),Number(m[2])]);
    const baseDefinitions=new Set(pairs.map(pair=>pair[0]));
    const uiCatalog=fs.readFileSync(path.join(root,'config/src/ui_state.rs'),'utf8');
    for(const match of uiCatalog.matchAll(/SkinItem \{ def_index: (\d+),[^\n]+category: [012],/g))baseDefinitions.add(Number(match[1]));
    baseDefinitions.add(42);baseDefinitions.add(59);
    for(const definition of baseDefinitions)pairs.push([definition,0]);
    const chosen=new Map(),missing=[];
    for(const [definition,paint] of pairs){
        const sourceItem=data.items[String(definition)];if(!sourceItem)continue;
        const item=inherited(sourceItem),kit=data.paint_kits[String(paint)];
        const image=paint===0?item.image_inventory:null;
        const filename=paint===0
            ? image?`panorama/images/${image}_png.vtex_c`:`panorama/images/econ/weapons/base_weapons/${item.name}_png.vtex_c`
            : kit?`panorama/images/econ/default_generated/${item.name}_${kit.name}_light_png.vtex_c`:null;
        // Import wearable/weapon inventory assets only; skip stickers/music/keys.
        if(paint===0 && !/weapon_|glove|agents\//i.test(String(image)+' '+item.name))continue;
        if(!filename||!archive.entries.has(filename.toLowerCase())){if(paint!==0)missing.push([definition,paint]);continue;}
        chosen.set(`${definition}:${paint}`,{definition,paint,filename});
    }
    if(chosen.size<1000)throw Error(`Incomplete icon selection: ${chosen.size}`);
    const entries=[...chosen.values()].sort((a,b)=>a.definition-b.definition||a.paint-b.paint);
    if(!args.includes('--reuse-decoded')) {
        for(const entry of entries)fs.writeFileSync(path.join(rawDir,`${entry.definition}_${entry.paint}.vtex_c`),archive.read(entry.filename));
        console.log(`Decoding ${entries.length} selected inventory textures offline; ${missing.length} absent pairs.`);
        const result=spawnSync(vrf,['-i',rawDir,'-o',pngDir,'--recursive','-d','--threads','4','-q'],{encoding:'utf8',maxBuffer:16*1024*1024,windowsHide:true});
        fs.writeFileSync(path.join(work,'decode.log'),`${result.stdout||''}\n${result.stderr||''}`);
        if(result.error||result.status!==0)throw result.error||Error(`VRF exit ${result.status}`);
        const optimize=spawnSync('cargo',['run','--manifest-path',path.join(root,'tools/iconpack/Cargo.toml'),'--release','--offline','--',pngDir],{encoding:'utf8',maxBuffer:16*1024*1024,windowsHide:true});
        fs.writeFileSync(path.join(work,'optimize.log'),`${optimize.stdout||''}\n${optimize.stderr||''}`);
        if(optimize.error||optimize.status!==0)throw optimize.error||Error(`Icon optimization exit ${optimize.status}`);
    }
    const chunks=[],index=[];let offset=0;const dimensions={};
    for(const entry of entries){
        const png=fs.readFileSync(path.join(pngDir,`${entry.definition}_${entry.paint}.png`));
        if(!png.subarray(0,8).equals(Buffer.from([137,80,78,71,13,10,26,10])))throw Error('Invalid PNG output');
        const width=png.readUInt32BE(16),height=png.readUInt32BE(20);dimensions[`${width}x${height}`]=(dimensions[`${width}x${height}`]||0)+1;
        if(width===0||height===0||width>2048||height>2048)throw Error(`Invalid cropped icon ${entry.filename}: ${width}x${height}`);
        if(offset+png.length>0xffffffff)throw Error('Icon pack exceeds index range');
        index.push([entry.definition,entry.paint,offset,png.length]);chunks.push(png);offset+=png.length;
    }
    const destination=path.join(root,'resources/assets');fs.mkdirSync(destination,{recursive:true});
    const pack=Buffer.concat(chunks);fs.writeFileSync(path.join(destination,'skin_icons.bin'),pack);
    fs.writeFileSync(path.join(root,'resources/src/skin_icons.rs'),`// Generated by tools/import_skin_icons.mjs; all PNGs are embedded at build time.\nconst PACK: &[u8] = include_bytes!("../assets/skin_icons.bin");\nconst INDEX: &[(i16,u32,u32,u32)] = &[\n${index.filter(e=>e[0]<=32767).map(e=>'    ('+e.join(',')+'),').join('\n')}\n];\npub fn png(definition:i16,paint:u32)->Option<&'static [u8]> {\n    let slot=INDEX.binary_search_by_key(&(definition,paint),|e|(e.0,e.1)).ok()?;\n    let entry=INDEX[slot];let start=entry.2 as usize;PACK.get(start..start.checked_add(entry.3 as usize)?)\n}\npub fn count()->usize {INDEX.len()}\n`);
    const manifest={source,vpk_index_sha256:createHash('sha256').update(fs.readFileSync(source)).digest('hex'),alpha_cropped:true,items_sha256:createHash('sha256').update(itemsBytes).digest('hex'),pack_sha256:createHash('sha256').update(pack).digest('hex'),count:index.length,bytes:pack.length,dimensions,missing};
    fs.writeFileSync(path.join(destination,'skin_icons.manifest.json'),JSON.stringify(manifest,null,2));
    console.log(JSON.stringify({count:manifest.count,bytes:manifest.bytes,dimension_variants:Object.keys(dimensions).length,missing:missing.length}));
} finally { archive.close(); }
