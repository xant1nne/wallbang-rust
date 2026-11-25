// Config block: MAX_SIG_BYTES = 128; скан идёт по SizeOfImage модуля, остановка на первом совпадении.
use crate::modules;

pub const MAX_SIG_BYTES: usize = 128;

#[derive(Clone, Debug, Default)]
pub struct Pattern {
    bytes: Vec<u8>,
    mask: Vec<bool>,
}

impl Pattern {
    /// Разбирает строку вида "48 8B C4 ? 55".
    pub fn parse(text: &str) -> Option<Self> {
        let mut bytes = Vec::new();
        let mut mask = Vec::new();
        for part in text.split_whitespace() {
            if bytes.len() >= MAX_SIG_BYTES {
                return None;
            }
            if part == "?" || part == "??" {
                bytes.push(0);
                mask.push(false);
            } else {
                bytes.push(u8::from_str_radix(part, 16).ok()?);
                mask.push(true);
            }
        }
        if bytes.len() < 4 {
            return None;
        }
        Some(Self { bytes, mask })
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Скан в [start, start+len). Возвращает абсолютный адрес первого совпадения.
    pub fn scan(&self, start: usize, len: usize) -> Option<usize> {
        if start == 0 || len < self.bytes.len() || self.bytes.is_empty() {
            return None;
        }
        // Первый не-wildcard байт — быстрый фильтр.
        let anchor = self.mask.iter().position(|&m| m)?;
        let end = start.checked_add(len - self.bytes.len())?;
        let mut address = start;
        while address <= end {
            if unsafe { *((address + anchor) as *const u8) } != self.bytes[anchor] {
                address += 1;
                continue;
            }
            let mut ok = true;
            for i in 0..self.bytes.len() {
                if self.mask[i] && unsafe { *((address + i) as *const u8) } != self.bytes[i] {
                    ok = false;
                    break;
                }
            }
            if ok {
                return Some(address);
            }
            address += 1;
        }
        None
    }

    /// Проверяет, что по адресу лежит ровно эта сигнатура.
    pub fn matches(&self, address: usize) -> bool {
        if address == 0 {
            return false;
        }
        self.mask.iter().enumerate().all(|(i, &m)| {
            !m || unsafe { *((address + i) as *const u8) } == self.bytes[i]
        })
    }
}

/// Сканирует образ загруженного модуля (например "client.dll").
pub fn scan_module(module: &str, pattern: &str) -> Option<usize> {
    let pat = Pattern::parse(pattern)?;
    let base = modules::module_base(module)?;
    let size = modules::module_size(module)?;
    pat.scan(base, size)
}
