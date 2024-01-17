use crate::lexi_data::LexiData;

pub struct Builder {
    buf: Vec<u8>,
}

enum TypeByte {
    Array,
    Bulk,
    Int,
    Double,
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
        self.add_type_byte(TypeByte::Int);
        let s = int.to_string();
        for ch in s.bytes() {
            self.buf.push(ch);
        }
        self.add_end();
        self
    }

    pub fn add_double(mut self, dbl: f64) -> Self {
        self.add_type_byte(TypeByte::Double);
        let s = dbl.to_string();
        for ch in s.bytes() {
            self.buf.push(ch)
        }
        self.add_end();
        self
    }

    pub fn add_impl_lexi_data(self, value: impl Into<LexiData>) -> Self {
        match value.into() {
            LexiData::Bulk(s) => self.add_bulk(&s),
            LexiData::Int(i) => self.add_int(i),
            LexiData::Double(d) => self.add_double(d),
            _ => unreachable!(),
        }
    }

    fn add_type_byte(&mut self, type_byte: TypeByte) {
        match type_byte {
            TypeByte::Array => self.buf.push(b'*'),
            TypeByte::Bulk => self.buf.push(b'$'),
            TypeByte::Int => self.buf.push(b':'),
            TypeByte::Double => self.buf.push(b','),
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
        let t = vec![b':', b'4', b'2', b'0', b'6', b'9', b'\r', b'\n'];
        assert_eq!(t, buf);
    }

    #[test]
    fn builder_can_add_doubles() {
        let buf = Builder::new().add_double(1337.1337).out();
        let t = vec![
            b',', b'1', b'3', b'3', b'7', b'.', b'1', b'3', b'3', b'7', b'\r', b'\n',
        ];
        assert_eq!(buf, t);
    }
}
