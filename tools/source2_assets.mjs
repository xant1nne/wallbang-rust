// Shared offline Source 2 archive/KV readers.
import fs from 'node:fs';
export function archiveIndex(file) {
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
export function keyValues(text) {
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

