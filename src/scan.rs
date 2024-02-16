// scan.rs
use std::process;

#[derive(Debug, PartialEq)]
pub enum ScanToken {
    IntLit,
    HexLit,
    BinLit,
    Plus,
    Minus,
    Mult,
    Div,
    ShiftRight,
    ShiftLeft,
    ArithShiftRight,
    BitAnd,
    BitOr,
    BitXor,
    BitNot,
    LParen,
    RParen,
    EOT,
    Any,
}

pub struct ScanTokenSt {
    pub id: ScanToken,
    pub value: String,
}

pub struct ScanTableSt {
    pub table: Vec<ScanTokenSt>,
    pub len: usize,
    pub cur: usize,
}

impl ScanTableSt {
    pub fn new() -> Self {
        ScanTableSt {
            table: Vec::new(),
            len: 0,
            cur: 0,
        }
    }

    pub fn init(&mut self) {
        self.len = 0;
        self.cur = 0;
    }

    pub fn new_token(&mut self) -> &mut ScanTokenSt {
        let token = ScanTokenSt {
            id: ScanToken::Any,
            value: String::new(),
        };
        self.table.push(token);
        self.len += 1;
        self.table.last_mut().unwrap()
    }

    pub fn scan(&mut self, input: &str) {
        let mut iter = input.chars().peekable();
        while let Some(c) = iter.next() {
            if c == '+' {
                self.add_token(ScanToken::Plus, c);
            } else if c == '-' {
                self.add_token(ScanToken::Minus, c);
            } else if c == '*' {
                self.add_token(ScanToken::Mult, c);
            } else if c == '/' {
                self.add_token(ScanToken::Div, c);
            } else if c == '>' {
                if iter.peek() == Some(&'>') {
                    self.add_token(ScanToken::ShiftRight, c);
                } else if iter.peek() == Some(&'-') {
                    self.add_token(ScanToken::ArithShiftRight, c);
                } else {
                    self.add_token(ScanToken::Any, c);
                }
            } else if c == '<' {
                if iter.peek() == Some(&'<') {
                    self.add_token(ScanToken::ShiftLeft, c);
                } else {
                    self.add_token(ScanToken::Any, c);
                }
            } else if c == '&' {
                self.add_token(ScanToken::BitAnd, c);
            } else if c == '|' {
                self.add_token(ScanToken::BitOr, c);
            } else if c == '^' {
                self.add_token(ScanToken::BitXor, c);
            } else if c == '~' {
                self.add_token(ScanToken::BitNot, c);
            } else if c == '(' {
                self.add_token(ScanToken::LParen, c);
            } else if c == ')' {
                self.add_token(ScanToken::RParen, c);
            } else if c.is_digit(10) {
                self.scan_intlit(&mut iter,c);
            } else if c == '0' {
                if iter.peek() == Some(&'x') || iter.peek() == Some(&'X') {
                    self.scan_hexlit(&mut iter);
                } else if iter.peek() == Some(&'b') || iter.peek() == Some(&'B') {
                    self.scan_binlit(&mut iter);
                } else {
                    self.add_token(ScanToken::Any, c);
                }
            } else if c.is_whitespace() {
                continue;
            } else {
                println!("scan error: invalid char: {}", c);
                process::exit(-1);
            }
        }
        self.add_token(ScanToken::EOT, '\0');
    }


    fn add_token(&mut self, id: ScanToken, value: char) {
        let token = self.new_token();
        token.id = id;
        token.value.push(value);
    }

    fn scan_intlit(&mut self, iter: &mut std::iter::Peekable<std::str::Chars>, c: char) {
        let token = self.new_token();
        token.id = ScanToken::IntLit;
        token.value.push(c);
        while let Some(&c) = iter.peek() {
            if c.is_digit(10) {
                token.value.push(iter.next().unwrap());
            } else {
                break;
            }
        }
    }

    fn scan_hexlit(&mut self, iter: &mut std::iter::Peekable<std::str::Chars>) {
        let token = self.new_token();
        token.id = ScanToken::HexLit;
        token.value.push('0');
        token.value.push(iter.next().unwrap());
        while let Some(&c) = iter.peek() {
            if c.is_digit(16) {
                token.value.push(iter.next().unwrap());
            } else {
                break;
            }
        }
    }

    fn scan_binlit(&mut self, iter: &mut std::iter::Peekable<std::str::Chars>) {
        let token = self.new_token();
        token.id = ScanToken::BinLit;
        token.value.push('0');
        token.value.push(iter.next().unwrap());
        while let Some(&c) = iter.peek() {
            if c == '0' || c == '1' {
                token.value.push(iter.next().unwrap());
            } else {
                break;
            }
        }
    }
}

impl ScanTableSt {
    pub fn accept(&mut self, tk_expected: ScanToken) -> bool {
        if tk_expected == ScanToken::Any {
            self.cur += 1;
            return true;
        }

        if let Some(token) = self.table.get(self.cur) {
            if token.id == tk_expected {
                self.cur += 1;
                return true;
            }
        }

        false
    }
}
