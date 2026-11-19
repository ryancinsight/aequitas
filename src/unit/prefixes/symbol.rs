/// UTF-8 bytes from two static symbol tokens, joined during const evaluation.
/// The vocabulary is sealed. 16 bytes accommodate the longest pair of tokens
/// in the catalog; overflow is a const error if a future token grows past it.
pub(super) struct Symbol {
    bytes: [u8; 16],
    len: usize,
}

impl Symbol {
    pub(super) const fn new(prefix: &str, root: &str) -> Self {
        let mut symbol = Self {
            bytes: [0; 16],
            len: 0,
        };
        symbol.push(prefix);
        symbol.push(root);
        symbol
    }

    const fn push(&mut self, token: &str) {
        let token = token.as_bytes();
        let mut index = 0;
        while index < token.len() {
            self.bytes[self.len] = token[index];
            self.len += 1;
            index += 1;
        }
    }

    pub(super) const fn as_str(&self) -> &str {
        // The used bytes are a concatenation of two valid UTF-8 strings.
        match core::str::from_utf8(self.bytes.split_at(self.len).0) {
            Ok(symbol) => symbol,
            Err(_) => unreachable!(),
        }
    }
}
