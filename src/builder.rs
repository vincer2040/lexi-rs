pub struct Builder {
    buf: Vec<u8>,
}

enum TypeByte {
    Array,
    Bulk,
    Int,
    Simple,
}

impl Builder {
    pub fn new() -> Self {
        let buf = Vec::new();
        Builder { buf }
    }

    pub fn add_ping(mut self) -> Self {
        self.add_type_byte(TypeByte::Simple);
        self.buf.push(b'P');
        self.buf.push(b'I');
        self.buf.push(b'N');
        self.buf.push(b'G');
        self.add_end();
        self
    }

    pub fn add_arr(mut self, len: usize) -> Self {
        self.add_type_byte(TypeByte::Array);
        self.add_len(len);
        self.add_end();
        self
    }

    pub fn add_bulk(mut self, bulk: &str) -> Self {
        let len = bulk.len();
        self.add_type_byte(TypeByte::Bulk);
        self.add_len(len);
        self.add_end();
        self.add_string(bulk);
        self.add_end();
        self
    }

    pub fn add_int(mut self, int: i64) -> Self {
        let mut shift = 56;
        let end = 8;
        self.add_type_byte(TypeByte::Int);
        for _ in 0..end {
            let byte = (int >> shift) as u8;
            self.buf.push(byte);
            shift -= 8;
        }
        self.add_end();
        self
    }

    pub fn reset(mut self) -> Self {
        self.buf = Vec::new();
        self
    }

    fn add_type_byte(&mut self, type_byte: TypeByte) {
        match type_byte {
            TypeByte::Array => self.buf.push(b'*'),
            TypeByte::Bulk => self.buf.push(b'$'),
            TypeByte::Int => self.buf.push(b':'),
            TypeByte::Simple => self.buf.push(b'+'),
        }
    }

    fn add_len(&mut self, len: usize) {
        let len_str = len.to_string();
        for ch in len_str.chars() {
            self.buf.push(ch as u8);
        }
    }

    fn add_string(&mut self, str: &str) {
        for ch in str.chars() {
            self.buf.push(ch as u8);
        }
    }

    fn add_end(&mut self) {
        self.buf.push(b'\r');
        self.buf.push(b'\n');
    }

    pub fn out(self) -> Vec<u8> {
        self.buf
    }
}

#[cfg(test)]
mod test {

    use super::Builder;

    #[test]
    fn builder_can_add_strings() {
        let buf = Builder::new().add_bulk("vince").out();
        let buf_str = String::from_utf8(buf).unwrap();
        assert_eq!(buf_str, "$5\r\nvince\r\n");
    }

    #[test]
    fn bulilder_can_add_arrays() {
        let buf = Builder::new()
            .add_arr(2)
            .add_bulk("vince")
            .add_bulk("is cool")
            .out();
        let buf_str = String::from_utf8(buf).unwrap();
        assert_eq!(buf_str, "*2\r\n$5\r\nvince\r\n$7\r\nis cool\r\n");
    }

    #[test]
    fn builder_can_add_integers() {
        let buf = Builder::new().add_int(42069).out();
        let t = vec![
            0x3a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa4, 0x55, 0x0d, 0x0a,
        ];
        assert_eq!(t, buf);
    }
}
