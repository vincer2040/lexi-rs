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

    struct StringTest<'a> {
        input: &'a [u8],
        exp: &'static str,
    }

    struct SimpleTest<'a> {
        input: &'a [u8],
        exp: SimpleString,
    }

    #[test]
    fn it_can_parse_strings() -> anyhow::Result<()> {
        let tests = [
            StringTest {
                input: b"$3\r\nfoo\r\n",
                exp: "foo",
            },
            StringTest {
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
            SimpleTest {
                input: b"+OK\r\n",
                exp: SimpleString::Ok,
            },
            SimpleTest {
                input: b"+PONG\r\n",
                exp: SimpleString::Pong,
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
}
