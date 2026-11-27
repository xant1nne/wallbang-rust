// Offline, native-UV custom finishes only. Procedural finishes require their
// own bake and are deliberately not represented by a random pattern/tint.
import fs from 'node:fs';
import path from 'node:path';
import {spawnSync} from 'node:child_process';
import {archiveIndex,keyValues} from './source2_assets.mjs';
const root=path.resolve(import.meta.dirname,'..');
const game=String.raw`C:\Program Files (x86)\Steam\steamapps\common\Counter-Strike Global Offensive\game\csgo`;
const vrf=String.raw`C:\wb_toolchain\tools\vendor\vrf-21.1\Source2Viewer-CLI.exe`;
const work=path.join(root,'target/tmp/weapon-finish-import');for(const dir of ['raw','decoded','jpg'])fs.mkdirSync(path.join(work,dir),{recursive:true});
const archive=archiveIndex(path.join(game,'pak01_dir.vpk'));
const kv=keyValues(archive.read('scripts/items/items_game.txt').toString('utf8'));
const kits=Object.entries(kv.paint_kits).filter(([id,v])=>+v.style===7&&archive.entries.has(`materials/models/weapons/customization/paints/vmats/${v.name}.vmat_c`)).map(([id,v])=>({id:+id,name:v.name,material:`materials/models/weapons/customization/paints/vmats/${v.name}.vmat_c`}));
function run(exe,args,options={}){const r=spawnSync(exe,args,{cwd:root,encoding:'utf8',maxBuffer:128*1024*1024,...options});if(r.status!==0)throw Error(`${exe}: ${r.stderr||r.stdout}`);return r.stdout;}
const dataFile=path.join(work,'materials.txt');if(!process.argv.includes('--reuse'))fs.writeFileSync(dataFile,run(vrf,['-i',path.join(game,'pak01_dir.vpk'),'-f',kits.map(k=>k.material).join(','),'-b','DATA']));
const data=fs.readFileSync(dataFile,'utf8'),selected=[];
for(const kit of kits){const begin=data.indexOf(`m_materialName = "${kit.material.slice(0,-2)}"`);if(begin<0)continue;const end=data.indexOf('--- Data for block',begin+1);const block=data.slice(begin,end<0?data.length:end);
 const param=(name,field)=>block.match(new RegExp(`m_name = "${name}"\\s+${field} = ([^\\r\\n]+)`))?.[1];
 const pattern=param('g_tPattern','m_pValue')?.match(/resource:"([^"]+)"/)?.[1];
 if(!pattern||+param('F_PAINT_STYLE','m_nValue')!==6)continue;
 if(param('g_flPatternTexCoordScale','m_flValue')&&+param('g_flPatternTexCoordScale','m_flValue')!==1)continue;
 if(param('g_flPatternTexCoordRotation','m_flValue')&&+param('g_flPatternTexCoordRotation','m_flValue')!==0)continue;
 const bytes=archive.read(pattern+'_c');if(!bytes)continue;
 fs.writeFileSync(path.join(work,'raw',`${kit.id}.vtex_c`),bytes);selected.push({...kit,pattern});
}
archive.close();
console.log(`Native-UV finish textures selected: ${selected.length}/${kits.length}`);
if(!process.argv.includes('--reuse'))console.log(run(vrf,['-i',path.join(work,'raw'),'--recursive','-d','-o',path.join(work,'decoded'),'--threads','4','-q']).trim());
const jobs=selected.map(k=>({input:path.join(work,'decoded',`${k.id}.png`),output:path.join(work,'jpg',`${k.id}.jpg`)}));const jobsFile=path.join(work,'jobs.json');fs.writeFileSync(jobsFile,JSON.stringify(jobs));
run('cargo',['build','--release','--offline','--manifest-path','tools/weaponpack/Cargo.toml','--target-dir','target/weaponpack'],{stdio:'inherit'});
run(path.join(root,'target/weaponpack/release/wallbang_weaponpack.exe'),['--textures',jobsFile],{stdio:'inherit'});
let offset=0;const parts=[],index=[];for(const k of selected.sort((a,b)=>a.id-b.id)){const bytes=fs.readFileSync(path.join(work,'jpg',`${k.id}.jpg`));parts.push(bytes);index.push(`    (${k.id}, ${offset}, ${bytes.length}),`);offset+=bytes.length;}
fs.writeFileSync(path.join(root,'resources/assets/weapon_finishes.bin'),Buffer.concat(parts));fs.writeFileSync(path.join(root,'resources/assets/weapon_finishes.manifest.json'),JSON.stringify(selected,null,2));
fs.writeFileSync(path.join(root,'resources/src/weapon_finishes.rs'),`// Generated native-UV custom finish textures; procedural styles are not baked.\nconst PACK: &[u8] = include_bytes!("../assets/weapon_finishes.bin");\npub const INDEX: &[(u32,usize,usize)] = &[\n${index.join('\n')}\n];\npub fn texture(paint:u32)->Option<&'static [u8]> {let i=INDEX.binary_search_by_key(&paint,|v|v.0).ok()?;let (_,offset,length)=INDEX[i];PACK.get(offset..offset+length)}\n`);
console.log(`Embedded ${selected.length} finish textures, ${offset} bytes`);
