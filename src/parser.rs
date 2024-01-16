use crate::lexi_data::{LexiData, SimpleString};

pub struct Parser<'a> {
    input: &'a [u8],
    pos: usize,
    ch: u8,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        let mut p = Self {
            input,
            pos: 0,
            ch: 0,
        };
        p.read_byte();
        return p;
    }

    pub fn parse(&mut self) -> anyhow::Result<LexiData> {
        match self.ch {
            b'$' => self.parse_string(),
            b'+' => self.parse_simple(),
            b':' => self.parse_int(),
            b',' => self.parse_double(),
            b'-' => self.parse_error(),
            _ => todo!(),
        }
    }

    fn parse_string(&mut self) -> anyhow::Result<LexiData> {
        let mut string = String::new();
        if !self.expect_peek_to_be_num() {
            return Err(anyhow::anyhow!("expected length"));
        }
        let length = self.parse_length();

        if !self.cur_byte_is(b'\r') {
            return Err(anyhow::anyhow!("expected retcar"));
        }
        if !self.expect_peek(b'\n') {
            return Err(anyhow::anyhow!("expected newline"));
        }

        self.read_byte();

        for _ in 0..length {
            string.push(self.ch as char);
            self.read_byte();
        }

        if !self.cur_byte_is(b'\r') {
            return Err(anyhow::anyhow!("expected retcar"));
        }
        if !self.expect_peek(b'\n') {
            return Err(anyhow::anyhow!("expected newline"));
        }

        self.read_byte();
        return Ok(LexiData::Bulk(string));
    }

    fn parse_int(&mut self) -> anyhow::Result<LexiData> {
        self.read_byte();
        let mut s = String::new();
        while self.ch != b'\r' && self.ch != 0 {
            s.push(self.ch as char);
            self.read_byte();
        }
        if !self.cur_byte_is(b'\r') {
            return Err(anyhow::anyhow!("expected retcar"));
        }
        if !self.expect_peek(b'\n') {
            return Err(anyhow::anyhow!("expected newline"));
        }

        let res: i64 = s.parse()?;

        self.read_byte();
        return Ok(LexiData::Int(res));
    }

    fn parse_double(&mut self) -> anyhow::Result<LexiData> {
        self.read_byte();
        let mut s = String::new();
        while self.ch != b'\r' && self.ch != 0 {
            s.push(self.ch as char);
            self.read_byte();
        }
        if !self.cur_byte_is(b'\r') {
            return Err(anyhow::anyhow!("expected retcar"));
        }
        if !self.expect_peek(b'\n') {
            return Err(anyhow::anyhow!("expected newline"));
        }

        let res: f64 = s.parse()?;

        self.read_byte();
        return Ok(LexiData::Double(res));
    }

    fn parse_simple(&mut self) -> anyhow::Result<LexiData> {
        let mut string = String::new();
        self.read_byte();
        while self.ch != b'\r' && self.ch != 0 {
            string.push(self.ch as char);
            self.read_byte();
        }

        let simple_string = match string.as_str() {
            "OK" => SimpleString::Ok,
            "PONG" => SimpleString::Pong,
            "NONE" => SimpleString::None,
            _ => return Err(anyhow::anyhow!("unkown simple string")),
        };

        if !self.cur_byte_is(b'\r') {
            return Err(anyhow::anyhow!("expected retcar"));
        }
        if !self.expect_peek(b'\n') {
            return Err(anyhow::anyhow!("expected newline"));
        }
        self.read_byte();
        return Ok(LexiData::Simple(simple_string));
    }

    fn parse_error(&mut self) -> anyhow::Result<LexiData> {
        let mut string = String::new();
        self.read_byte();
        while self.ch != b'\r' && self.ch != 0 {
            string.push(self.ch as char);
            self.read_byte();
        }

        if !self.cur_byte_is(b'\r') {
            return Err(anyhow::anyhow!("expected retcar"));
        }
        if !self.expect_peek(b'\n') {
            return Err(anyhow::anyhow!("expected newline"));
        }
        self.read_byte();
        return Ok(LexiData::Error(string));
    }

    fn parse_length(&mut self) -> usize {
        let mut res: usize = 0;
        while Parser::is_digit(self.ch) {
            res = (res * 10) + ((self.ch - b'0') as usize);
            self.read_byte();
        }
        return res;
    }

    fn peek_byte(&self) -> u8 {
        if self.pos >= self.input.len() {
            return 0;
        }
        return self.input[self.pos];
    }

    fn cur_byte_is(&self, byte: u8) -> bool {
        return self.ch == byte;
    }

    fn peek_byte_is(&self, byte: u8) -> bool {
        return self.peek_byte() == byte;
    }

    fn expect_peek(&mut self, byte: u8) -> bool {
        if self.peek_byte_is(byte) {
            self.read_byte();
            return true;
        }
        return false;
    }

    fn expect_peek_to_be_num(&mut self) -> bool {
        if Parser::is_digit(self.peek_byte()) {
            self.read_byte();
            return true;
        }
        return false;
    }

    fn is_digit(ch: u8) -> bool {
        return b'0' <= ch && ch <= b'9';
    }

    fn read_byte(&mut self) {
        if self.pos >= self.input.len() {
            self.ch = 0;
            return;
        }
        self.ch = self.input[self.pos];
        self.pos += 1;
    }
}

#[cfg(test)]
mod test {
    use crate::lexi_data::{LexiData, SimpleString};

    use super::Parser;

    struct ParserTest<'a, T> {
        input: &'a [u8],
        exp: T,
    }

    #[test]
    fn it_can_parse_strings() -> anyhow::Result<()> {
        let tests = [
            ParserTest {
                input: b"$3\r\nfoo\r\n",
                exp: "foo",
            },
            ParserTest {
                input: b"$7\r\nfoo\nbar\r\n",
                exp: "foo\nbar",
            },
        ];

        for test in tests {
            let mut p = Parser::new(test.input);
            let data = p.parse()?;
            assert!(matches!(data, LexiData::Bulk(_)));
            match data {
                LexiData::Bulk(s) => assert_eq!(test.exp, s),
                _ => unreachable!(),
            }
        }

        Ok(())
    }

    #[test]
    fn it_can_parse_simple_strings() -> anyhow::Result<()> {
        let tests = [
            ParserTest {
                input: b"+OK\r\n",
                exp: SimpleString::Ok,
            },
            ParserTest {
                input: b"+PONG\r\n",
                exp: SimpleString::Pong,
            },
            ParserTest {
                input: b"+NONE\r\n",
                exp: SimpleString::None,
            },
        ];

        for test in tests {
            let mut p = Parser::new(test.input);
            let data = p.parse()?;
            assert!(matches!(data, LexiData::Simple(_)));
            match data {
                LexiData::Simple(s) => assert_eq!(test.exp, s),
                _ => unreachable!(),
            }
        }
        Ok(())
    }

    #[test]
    fn it_can_parse_integers() -> anyhow::Result<()> {
        let tests = [
            ParserTest {
                input: b":12345\r\n",
                exp: 12345,
            },
            ParserTest {
                input: b":-12345\r\n",
                exp: -12345,
            },
        ];

        for test in tests {
            let mut p = Parser::new(test.input);
            let data = p.parse()?;
            assert!(matches!(data, LexiData::Int(_)));
            match data {
                LexiData::Int(i) => assert_eq!(test.exp, i),
                _ => unreachable!(),
            }
        }
        Ok(())
    }

    #[test]
    fn parse_error() -> anyhow::Result<()> {
        let tests = [
            ParserTest {
                input: b"-Invalid command\r\n",
                exp: "Invalid command",
            },
            ParserTest {
                input: b"-failed to set\r\n",
                exp: "failed to set",
            },
        ];

        for test in tests {
            let mut p = Parser::new(test.input);
            let data = p.parse()?;
            assert!(matches!(data, LexiData::Error(_)));
            match data {
                LexiData::Error(e) => assert_eq!(test.exp, e),
                _ => unreachable!(),
            }
        }
        Ok(())
    }

    #[test]
    fn parse_double() -> anyhow::Result<()> {
        let tests = [
            ParserTest {
                input: b",1337.1337\r\n",
                exp: 1337.1337 as f64,
            },
            ParserTest {
                input: b",1337.0\r\n",
                exp: 1337.0 as f64,
            },
            ParserTest {
                input: b",1337\r\n",
                exp: 1337.0 as f64,
            },
        ];

        for test in tests {
            let mut p = Parser::new(test.input);
            let data = p.parse()?;
            assert!(matches!(data, LexiData::Double(_)));
            match data {
                LexiData::Double(d) => assert_eq!(test.exp, d),
                _ => assert!(false),
            }
        }

        Ok(())
    }
}
