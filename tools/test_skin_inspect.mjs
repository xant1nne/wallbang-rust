// Architecture regression; actual geometry/render/cache tests live in Rust
// and render/examples/weapon_scene.rs, using a real D3D11 device.
import {readFileSync} from 'node:fs';
import {strict as assert} from 'node:assert';
const read=p=>readFileSync(new URL('../'+p,import.meta.url),'utf8');
const request=read('render/src/skin_inspect.rs'),hooks=read('render/src/hooks.rs'),dx=read('render/src/dx11.rs'),scene=read('render/src/model/inspect.rs');
assert(!/MapItemPreviewPanel|run_script|Panorama::|\$\.Schedule/.test(request+scene),'owned inspect must not create an engine scene');
assert(!/capture_inspect|inspect_capture/.test(dx+hooks),'inspect must not copy the game backbuffer');
assert(hooks.indexOf('menu.frame(')<hooks.indexOf('renderer.render_weapon('),'menu closure must be applied before drawing inspect');
assert(scene.includes('ClearDepthStencilView')&&scene.includes('DrawIndexed'),'preview must draw real indexed geometry');
assert(/self\.last\s*==\s*Some\(cache\)/.test(scene),'static scene should reuse its rendered texture');
const ui=read('config/src/ui_state.rs');
const imported=JSON.parse(read('resources/assets/weapon_models.manifest.json'));
for(const match of ui.matchAll(/SkinItem \{ def_index: (\d+),[^\n]+category: [012],/g))for(const legacy of [false,true])assert(imported.some(m=>m.definition===+match[1]&&m.legacy===legacy),`missing model ${match[1]} / legacy=${legacy}`);
const finishes=JSON.parse(read('resources/assets/weapon_finishes.manifest.json'));
assert(finishes.some(k=>k.id===279),'AWP Asiimov native-UV finish must be imported');
console.log(`Owned DX11 inspect boundary passed: ${imported.length} model variants, ${finishes.length} native-UV finishes; no Panorama scene/backbuffer capture.`);
