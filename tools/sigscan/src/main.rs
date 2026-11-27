// Config block: оффлайн-поиск сигнатур и vtable-слотов по локальному client.dll.
use std::fs;

struct Section {
    name: [u8; 8],
    va: u32,
    raw: u32,
    raw_size: u32,
}

struct Pe {
    image: Vec<u8>,
    image_base: u64,
    sections: Vec<Section>,
}

fn u16_at(b: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([b[o], b[o + 1]])
}

fn u32_at(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]])
}

fn u64_at(b: &[u8], o: usize) -> u64 {
    u64::from_le_bytes([
        b[o], b[o + 1], b[o + 2], b[o + 3], b[o + 4], b[o + 5], b[o + 6], b[o + 7],
    ])
}

impl Pe {
    fn load(path: &str) -> Self {
        let image = fs::read(path).expect("read dll");
        let pe = u32_at(&image, 0x3C) as usize;
        let count = u16_at(&image, pe + 6) as usize;
        let opt_size = u16_at(&image, pe + 20) as usize;
        let image_base = u64_at(&image, pe + 24 + 24);
        let mut sections = Vec::new();
        for i in 0..count {
            let o = pe + 24 + opt_size + i * 40;
            let mut name = [0u8; 8];
            name.copy_from_slice(&image[o..o + 8]);
            sections.push(Section {
                name,
                va: u32_at(&image, o + 12),
                raw_size: u32_at(&image, o + 16),
                raw: u32_at(&image, o + 20),
            });
        }
        Self { image, image_base, sections }
    }

    fn rva_to_raw(&self, rva: u32) -> Option<usize> {
        self.sections
            .iter()
            .find(|sec| rva >= sec.va && rva < sec.va + sec.raw_size)
            .map(|sec| sec.raw as usize + (rva - sec.va) as usize)
    }
}

fn parse_pattern(text: &str) -> (Vec<u8>, Vec<bool>) {
    let mut bytes = Vec::new();
    let mut mask = Vec::new();
    for part in text.split_whitespace() {
        if part == "?" || part == "??" {
            bytes.push(0);
            mask.push(false);
        } else {
            bytes.push(u8::from_str_radix(part, 16).expect("pattern byte"));
            mask.push(true);
        }
    }
    (bytes, mask)
}

fn scan_section(image: &[u8], sec: &Section, pat: &[u8], mask: &[bool]) -> Vec<u32> {
    let mut hits = Vec::new();
    let start = sec.raw as usize;
    let end = start + sec.raw_size as usize;
    let first = match mask.iter().position(|&m| m) {
        Some(i) => i,
        None => return hits,
    };
    let mut i = start;
    while i + pat.len() <= end {
        if image[i + first] == pat[first] {
            let mut ok = true;
            for j in 0..pat.len() {
                if mask[j] && image[i + j] != pat[j] {
                    ok = false;
                    break;
                }
            }
            if ok {
                hits.push(sec.va + (i - start) as u32);
            }
        }
        i += 1;
    }
    hits
}

fn scan_all(pe: &Pe, text: &str) -> Vec<u32> {
    let (pat, mask) = parse_pattern(text);
    let mut hits = Vec::new();
    for sec in &pe.sections {
        hits.extend(scan_section(&pe.image, sec, &pat, &mask));
    }
    hits
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        eprintln!("usage: sigscan pattern|vtable|rip <dll> ...");
        return;
    }
    let pe = Pe::load(&args[2]);
    match args[1].as_str() {
        "table" => {
            let rva = u32::from_str_radix(args[3].trim_start_matches("0x"), 16).unwrap();
            let raw = pe.rva_to_raw(rva).unwrap();
            for i in 0..80 { println!("slot {i}: 0x{:X}", u64_at(&pe.image, raw+i*8)); }
        }
        "pattern" => {
            for spec in &args[3..] {
                let (name, text) = spec.split_once('=').expect("name=pattern");
                let hits = scan_all(&pe, text);
                print!("{name} matches={}", hits.len());
                for hit in hits.iter().take(8) {
                    print!(" 0x{hit:X}");
                }
                println!();
            }
        }
        "rip" => {
            for spec in &args[3..] {
                let (name, rest) = spec.split_once('=').expect("name=pattern:disp_offset");
                let (text, disp_text) = rest.split_once(':').expect("need :disp_offset");
                let radix = if disp_text.starts_with("0x") { 16 } else { 10 };
                let clean = disp_text.trim_start_matches("0x");
                let disp_offset: u32 = u32::from_str_radix(clean, radix).expect("bad disp offset");
                let hits = scan_all(&pe, text);
                let Some(hit) = hits.first() else {
                    println!("{name} matches=0");
                    continue;
                };
                let Some(raw) = pe.rva_to_raw(*hit) else {
                    println!("{name} rva=0x{hit:X} no raw mapping");
                    continue;
                };
                let disp = u32_at(&pe.image, raw + disp_offset as usize) as i32;
                let next = hit + disp_offset + 4;
                let target = if disp >= 0 {
                    next.wrapping_add(disp as u32)
                } else {
                    next.wrapping_sub(disp.unsigned_abs())
                };
                println!("{name}: match=0x{hit:X} target=0x{target:X} disp={disp}");
            }
        }
        "vtable" => {
            let rva = u64::from_str_radix(args[3].trim_start_matches("0x"), 16).expect("rva");
            let target = pe.image_base + rva;
            println!("image_base=0x{:X} target=0x{:X}", pe.image_base, target);
            let max = pe.image_base + 0x4000000;
            for sec in &pe.sections {
                let start = sec.raw as usize;
                let end = start + sec.raw_size as usize;
                let mut o = start;
                while o + 8 <= end {
                    if u64_at(&pe.image, o) == target {
                        let mut begin = o;
                        while begin >= start + 8 {
                            let prev = u64_at(&pe.image, begin - 8);
                            if prev < pe.image_base || prev > max {
                                break;
                            }
                            begin -= 8;
                        }
                        let mut finish = o;
                        while finish + 16 <= end {
                            let next = u64_at(&pe.image, finish + 8);
                            if next < pe.image_base || next > max {
                                break;
                            }
                            finish += 8;
                        }
                        let index = (o - begin) / 8;
                        let slots = (finish - begin) / 8 + 1;
                        let name = String::from_utf8_lossy(&sec.name).trim_end_matches('\0').to_string();
                        println!(
                            "sec={name} index={index} vtable_rva=0x{:X} slots={slots}",
                            sec.va + (begin - start) as u32
                        );
                    }
                    o += 8;
                }
            }
        }
        other => eprintln!("unknown mode {other}"),
    }
}
