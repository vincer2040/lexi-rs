use crate::token::Token;

fn is_digit(ch: u8) -> bool {
    b'0' <= ch && ch <= b'9'
}

fn is_letter(ch: u8) -> bool {
    b'a' <= ch && ch <= b'z' || b'A' <= ch && ch <= b'Z'
}

pub struct Lexer {
    input: Vec<u8>,
    pos: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: Vec<u8>) -> Self {
        let mut l = Lexer { input, pos: 0, ch: 0 };
        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> Token {
        let res: Token;
        match self.ch {
            0 => res = Token::Eof,
            b'*' => res = Token::ArrayType,
            b'$' => res = Token::BulkType,
            b'\r' => res = Token::RetCar,
            b'\n' => res = Token::NewL,
            b':' => {
                let buf: Vec<u8>;
                self.read_char();
                buf = self.read_int();
                return Token::Int(buf.into());
            },
            _ => {
                if is_letter(self.ch) {
                    let str = self.read_string();
                    return Token::Bulk(str.into());
                } else if is_digit(self.ch) {
                    let len = self.read_len();
                    return Token::Len(len.into());
                } else {
                    return Token::Illegal;
                }
            }
        };

        self.read_char();

        res
    }

    fn read_len(&mut self) -> String {
        let mut res = String::new();
        while is_digit(self.ch) {
            res.push(self.ch as char);
            self.read_char();
        }
        res
    }

    fn read_string(&mut self) -> String {
        let mut res = String::new();
        while self.ch != b'\r' {
            res.push(self.ch as char);
            self.read_char();
        }
        res
    }

    fn read_int(&mut self) -> Vec<u8> {
        let mut res = Vec::<u8>::new();
        while self.ch != b'\r' {
            res.push(self.ch);
            self.read_char();
        }
        res
    }

    fn read_char(&mut self) {
        if self.pos >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.pos];
        }
        self.pos += 1;
    }
}

#[cfg(test)]
mod test {

    use crate::builder::Builder;
    use crate::lexer::Lexer;
    use crate::token::Token;

    #[test]
    fn it_can_lex_bulk_strings() {
        let buf = Builder::new()
            .add_bulk("vince")
            .out();
        let mut lexer = Lexer::new(buf);
        let exps = vec![
            Token::BulkType,
            Token::Len("5".into()),
            Token::RetCar,
            Token::NewL,
            Token::Bulk("vince".into()),
            Token::RetCar,
            Token::NewL,
            Token::Eof,
        ];
        for exp in exps.iter() {
            let tok = lexer.next_token();
            assert_eq!(tok, *exp);
        }
    }

    #[test]
    fn it_can_lex_arrays() {
        let buf = Builder::new()
            .add_arr(2)
            .add_bulk("vince")
            .add_bulk("is cool")
            .out();
        let mut lexer = Lexer::new(buf);
        let exps = vec![
            Token::ArrayType,
            Token::Len("2".into()),
            Token::RetCar,
            Token::NewL,
            Token::BulkType,
            Token::Len("5".into()),
            Token::RetCar,
            Token::NewL,
            Token::Bulk("vince".into()),
            Token::RetCar,
            Token::NewL,
            Token::BulkType,
            Token::Len("7".into()),
            Token::RetCar,
            Token::NewL,
            Token::Bulk("is cool".into()),
            Token::RetCar,
            Token::NewL,
            Token::Eof,
        ];
        for exp in exps.iter() {
            let tok = lexer.next_token();
            assert_eq!(tok, *exp);
        }
    }

    #[test]
    fn it_can_lex_integers() {
        let buf = Builder::new()
            .add_int(42069)
            .out();
        let mut l = Lexer::new(buf);
        let t = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa4, 0x55];
        let exps = [Token::Int(t.into()), Token::RetCar, Token::NewL, Token::Eof];
        for exp in exps.iter() {
            let tok = l.next_token();
            assert_eq!(tok , *exp);
        }
    }
}
