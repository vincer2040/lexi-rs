use crate::{lexer::Lexer, lexi_type::LexiType, token::Token};
use anyhow::{anyhow, Result};

pub struct Parser {
    l: Lexer,
    cur: Option<Token>,
    peek: Option<Token>,
}

impl Parser {
    pub fn new(l: Lexer) -> Self {
        let mut p = Parser {
            l,
            cur: None,
            peek: None,
        };

        p.next_token();
        p.next_token();

        p
    }

    pub fn parse(&mut self) -> Result<LexiType> {
        match &self.cur {
            Some(Token::Simple(v)) => {
                let str: String = v.to_string();
                if !self.expect_peek(Token::RetCar) {
                    return Err(anyhow!("peek err"));
                }
                if !self.expect_peek(Token::NewL) {
                    return Err(anyhow!("peek err"));
                }
                self.next_token();
                Ok(LexiType::Simple(str))
            }
            Some(Token::BulkType) => {
                let str: String;
                self.next_token();
                match self.cur {
                    Some(Token::Len(_)) => {}
                    _ => return Err(anyhow!("peek err")),
                };
                if !self.expect_peek(Token::RetCar) {
                    return Err(anyhow!("peek err"));
                }
                if !self.expect_peek(Token::NewL) {
                    return Err(anyhow!("peek err"));
                }
                self.next_token();
                match &self.cur {
                    Some(Token::Bulk(v)) => {
                        str = v.to_string();
                    }
                    _ => return Err(anyhow!("expected bulk")),
                };
                if !self.expect_peek(Token::RetCar) {
                    return Err(anyhow!("peek err"));
                }
                if !self.expect_peek(Token::NewL) {
                    return Err(anyhow!("peek err"));
                }
                self.next_token();
                Ok(LexiType::BulkString(str))
            }
            Some(Token::ArrayType) => {
                let mut vec: Vec<LexiType> = Vec::new();
                self.next_token();
                match &self.cur {
                    Some(Token::Len(l)) => {
                        let len: usize = l.parse()?;
                        if !self.expect_peek(Token::RetCar) {
                            return Err(anyhow!("peek err"));
                        }
                        if !self.expect_peek(Token::NewL) {
                            return Err(anyhow!("peek err"));
                        }
                        self.next_token();
                        for _ in 0..len {
                            let val = self.parse()?;
                            vec.push(val);
                        }
                    }
                    _ => return Err(anyhow!("peek err")),
                };
                Ok(LexiType::Array(vec))
            }
            Some(Token::Int(buf)) => {
                let res: i64;
                let mut temp: u64 = 0;
                let mut shift = 56;
                for b in buf.iter() {
                    let btmp = *b as u64;
                    temp |= btmp << shift;
                    shift -= 8;
                }
                // hack for checking if value should be negative
                if temp <= 0x7fffffffffffffff {
                    res = temp as i64;
                } else {
                    res = -1 - ((0xffffffffffffffff - temp) as i64);
                }
                if !self.expect_peek(Token::RetCar) {
                    return Err(anyhow!("peek err"));
                }
                if !self.expect_peek(Token::NewL) {
                    return Err(anyhow!("peek err"));
                }
                self.next_token();
                Ok(LexiType::Int(res))
            }
            Some(Token::Err(e)) => {
                let s = e.to_string();
                if !self.expect_peek(Token::RetCar) {
                    return Err(anyhow!("peek err"));
                }
                if !self.expect_peek(Token::NewL) {
                    return Err(anyhow!("peek err"));
                }
                self.next_token();
                Ok(LexiType::Error(s))
            }
            _ => Err(anyhow!("illegal")),
        }
    }

    fn expect_peek(&mut self, tok: Token) -> bool {
        if let Some(t) = &self.peek {
            if tok == *t {
                self.next_token();
                return true;
            }
            return false;
        }
        false
    }

    fn next_token(&mut self) {
        self.cur = self.peek.clone();
        let n = self.l.next_token();
        match n {
            Token::Eof => self.peek = None,
            _ => self.peek = Some(n),
        }
    }
}

#[cfg(test)]
mod test {

    use crate::builder::Builder;
    use crate::lexer::Lexer;
    use crate::lexi_type::LexiType;
    use crate::parser::Parser;

    #[test]
    fn it_can_parse_bulk_strings() {
        let buf = Builder::new().add_bulk("vince").out();
        let l = Lexer::new(buf);
        let mut p = Parser::new(l);
        let val = p.parse().unwrap();
        assert_eq!(val, LexiType::BulkString("vince".to_owned()));
    }

    #[test]
    fn it_can_parse_arrays() {
        let buf = Builder::new()
            .add_arr(2)
            .add_bulk("vince")
            .add_bulk("is cool")
            .out();
        let lexer = Lexer::new(buf);
        let mut p = Parser::new(lexer);
        let val = p.parse().unwrap();
        let exps = vec![
            LexiType::BulkString("vince".to_owned()),
            LexiType::BulkString("is cool".to_owned()),
        ];
        assert_eq!(val, LexiType::Array(exps));
    }

    #[test]
    fn it_can_parse_integers() {
        let buf = Builder::new().add_int(42069).out();
        let l = Lexer::new(buf);
        let mut p = Parser::new(l);
        let val = p.parse().unwrap();
        assert_eq!(val, LexiType::Int(42069));
    }

    #[test]
    fn it_can_parse_negative_ints() {
        let buf = Builder::new().add_int(-42069).out();
        let l = Lexer::new(buf);
        let mut p = Parser::new(l);
        let val = p.parse().unwrap();
        assert_eq!(val, LexiType::Int(-42069));
    }

    #[test]
    fn it_can_parse_errors() {
        let buf = vec![b'-', b'E', b'r', b'r', b'\r', b'\n'];
        let l = Lexer::new(buf);
        let mut p = Parser::new(l);
        let val = p.parse().unwrap();
        assert_eq!(val, LexiType::Error("Err".to_string()));
    }
}
