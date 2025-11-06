/// Iterator que extrae nibbles (4 bits) de un slice de bytes
/// 
/// Cada byte se divide en dos nibbles: el nibble alto (bits 7-4) y el nibble bajo (bits 3-0).
/// El nibble alto se devuelve primero, seguido del nibble bajo.
pub struct Nibbler<'a> {
    bytes: std::slice::Iter<'a, u8>,
    pending_nibble: Option<u8>,
}

impl<'a> Nibbler<'a> {
    /// Crea un nuevo iterator de nibbles desde un slice de bytes
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes: bytes.iter(),
            pending_nibble: None,
        }
    }
}

impl Iterator for Nibbler<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(nibble) = self.pending_nibble.take() {
            Some(nibble)
        } else if let Some(&byte) = self.bytes.next() {
            let high_nibble = (byte & 0xf0) >> 4;
            let low_nibble = byte & 0x0f;
            self.pending_nibble = Some(low_nibble);
            Some(high_nibble)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Nibbler;

    #[test]
    fn iterates_nibbles() {
        let bytes = [0x2a];
        let nibbles: Vec<u8> = Nibbler::new(&bytes).collect();
        assert_eq!(vec![0x02, 0x0a], nibbles);
    }

    #[test]
    fn handles_multiple_bytes() {
        let bytes = [0x2a, 0xbc];
        let nibbles: Vec<u8> = Nibbler::new(&bytes).collect();
        assert_eq!(vec![0x02, 0x0a, 0x0b, 0x0c], nibbles);
    }

    #[test]
    fn handles_empty_slice() {
        let bytes = [];
        let nibbles: Vec<u8> = Nibbler::new(&bytes).collect();
        assert!(nibbles.is_empty());
    }
}
