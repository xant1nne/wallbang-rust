// Read-only ABI audit against the installed client.dll. No process attachment.
import fs from 'node:fs';
const file=String.raw`C:\Program Files (x86)\Steam\steamapps\common\Counter-Strike Global Offensive\game\csgo\bin\win64\client.dll`;
const b=fs.readFileSync(file),pe=b.readUInt32LE(0x3c),opt=pe+24,base=Number(b.readBigUInt64LE(opt+24)),tab=opt+b.readUInt16LE(pe+20),sections=[];
for(let i=0;i<b.readUInt16LE(pe+6);i++){let p=tab+40*i;sections.push({name:b.toString('ascii',p,p+8).replace(/\0/g,''),rva:b.readUInt32LE(p+12),size:b.readUInt32LE(p+16),raw:b.readUInt32LE(p+20)});}
const rva=p=>{const s=sections.find(s=>p>=s.raw&&p<s.raw+s.size);return s?p-s.raw+s.rva:null;};
const code=sections.find(s=>s.name==='.text'),data=sections.find(s=>s.name==='.rdata');
const source=fs.readFileSync(new URL('../config/src/signatures.rs',import.meta.url),'utf8'),signatures={};
for(const name of ['SIG_ECON_DATA_CHANGED','SIG_BUILD_ECON_MATERIALS','SIG_ECON_MATERIAL_CONTEXT','SIG_REGENERATE_SKIN']){
 const pattern=source.match(new RegExp(`pub const ${name}: &str = "([^"]+)"`))?.[1];if(!pattern)throw Error(`Missing ${name}`);
 const bytes=pattern.split(' ').map(v=>v==='?'?null:parseInt(v,16)),matches=[];
 for(let p=code.raw;p<code.raw+code.size-bytes.length;p++){if(b[p]!==bytes[0])continue;let ok=true;for(let i=1;i<bytes.length;i++)if(bytes[i]!==null&&bytes[i]!==b[p+i]){ok=false;break;}if(ok)matches.push(rva(p));}
 if(matches.length!==1)throw Error(`${name}: ${matches.length} matches`);signatures[name]='0x'+matches[0].toString(16);
}
const tables=[];
for(const name of ['C_EconEntity','C_CSWeaponBase','C_CSWeaponBaseGun','C_AK47','C_WeaponAWP','C_WeaponSSG08','C_WeaponSCAR20','C_Knife','C_CS2HudModelWeapon']){
 const idx=b.indexOf(Buffer.from('.?AV'+name+'@@\0'));if(idx<0)continue;const type=rva(idx-16);
 for(let p=data.raw;p<data.raw+data.size-24;p+=4){if(b.readUInt32LE(p)!==1||b.readUInt32LE(p+12)!==type||b.readUInt32LE(p+20)!==rva(p))continue;
  const offset=b.readUInt32LE(p+4);if(offset!==0&&offset!==0x610)continue;const needle=Buffer.alloc(8);needle.writeBigUInt64LE(BigInt(base+rva(p)));let v=-1;
  while((v=b.indexOf(needle,v+1))>=0){const slot=offset===0?11:3;tables.push({name,offset:'0x'+offset.toString(16),slot,address:'0x'+(Number(b.readBigUInt64LE(v+8+slot*8))-base).toString(16)});}
 }
}
const result={file,signatures,tables};fs.mkdirSync('target/tmp',{recursive:true});fs.writeFileSync('target/tmp/skin-backend-native.json',JSON.stringify(result,null,2));console.log(JSON.stringify(result,null,2));
