use std::str;

const SCAN_INPUT_LEN: usize = 1024;

#[derive(Debug, PartialEq)]
enum ScanTokenEnum {
    TK_INTLIT,
    TK_HEXLIT,
    TK_BINLIT,
    TK_PLUS,
    TK_MINUS,
    TK_MULT,
    TK_DIV,
    TK_SHIFT_RIGHT,
    TK_ARITH_SHIFT_RIGHT,
    TK_SHIFT_LEFT,
    TK_BIT_AND,
    TK_BIT_OR,
    TK_BIT_XOR,
    TK_BIT_NOT,
    TK_LPAREN,
    TK_RPAREN,
    TK_EOT,
    TK_ANY
}

#[derive(Debug)]
struct ScanTokenSt {
    id: ScanTokenEnum,
    value: String,
}

#[derive(Debug)]
pub(crate) struct ScanTableSt {
    table: Vec<ScanTokenSt>,
    len: usize,
    cur: usize,
}

impl ScanTableSt {
    pub(crate) fn new() -> Self {
        ScanTableSt {
            table: Vec::new(),
            len: 0,
            cur: 0,
        }
    }

    fn init(&mut self) {
        self.len = 0;
        self.cur = 0;
    }

    pub(crate) fn print(&self) {
        for token in &self.table {
            println!("{:?}", token);
        }
    }

    fn new_token(&mut self) -> &mut ScanTokenSt {
        let tp = ScanTokenSt {
            id: ScanTokenEnum::TK_ANY,
            value: String::new(),
        };
        self.table.push(tp);
        self.len += 1;
        self.table.last_mut().unwrap()
    }

    fn get(&self, i: usize) -> &ScanTokenSt {
        &self.table[self.cur + i]
    }

    fn accept(&mut self, tk_expected: ScanTokenEnum) -> bool {
        if tk_expected == ScanTokenEnum::TK_ANY {
            self.cur += 1;
            return true;
        }

        let tp = self.get(0);
        if tp.id == tk_expected {
            self.cur += 1;
            return true;
        }

        false
    }
}

fn is_whitespace(ch: char) -> bool {
    ch == ' ' || ch == '\t'
}

fn whitespace(mut p: usize, end: usize, input: &[u8]) -> usize {
    while p < end && is_whitespace(input[p] as char) {
        p += 1;
    }
    p
}

fn is_digit(ch: char) -> bool {
    ch >= '0' && ch <= '9'
}

fn intlit(mut p: usize, end: usize, input: &[u8], tp: &mut ScanTokenSt) -> usize {
let mut i = 0;

while p < end && is_digit(input[p] as char) {
tp.value.push(input[p] as char);
p += 1;
i += 1;
}
tp.id = ScanTokenEnum::TK_INTLIT;
p
}

fn hexlit(mut p: usize, end: usize, input: &[u8], tp: &mut ScanTokenSt) -> usize {
    let mut i = 0;
    p += 2;
    while p < end
        && (is_digit(input[p] as char)
        || (input[p] >= b'A' && input[p] <= b'F')
        || (input[p] >= b'a' && input[p] <= b'f'))
    {
        tp.value.push(input[p] as char);
        p += 1;
        i += 1;
    }
    tp.id = ScanTokenEnum::TK_HEXLIT;
    p
}

fn binlit(mut p: usize, end: usize, input: &[u8], tp: &mut ScanTokenSt) -> usize {
    let mut i = 0;
    p += 2; // Skip '0b' or '0B'
    while p < end && (input[p] == b'0' || input[p] == b'1') {
        tp.value.push(input[p] as char);
        p += 1;
        i += 1;
    }
    tp.id = ScanTokenEnum::TK_BINLIT;
    p
}

fn token_helper(
    tp: &mut ScanTokenSt,
    mut p: usize,
    len: usize,
    id: ScanTokenEnum,
    input: &[u8],
) -> usize {
    tp.id = id;
    for _ in 0..len {
        tp.value.push(input[p] as char);
        p += 1;
    }
    p
}

fn token(
    mut p: usize,
    end: usize,
    input: &[u8],
    tp: &mut ScanTokenSt,
) -> usize {
    if p == end {
        tp.value.push('\0');
        tp.id = ScanTokenEnum::TK_EOT;
    } else if is_whitespace(input[p] as char) {
        p = whitespace(p, end, input);
        p = token(p, end, input, tp);
    } else if input[p] == b'0'
        && (input[p + 1] == b'x' || input[p + 1] == b'X')
    {
        p = hexlit(p, end, input, tp);
    } else if input[p] == b'0'
        && (input[p + 1] == b'b' || input[p + 1] == b'B')
    {
        p = binlit(p, end, input, tp);
    } else if is_digit(input[p] as char) {
        p = intlit(p, end, input, tp);
    } else if input[p] == b'+' {
        p = token_helper(tp, p, 1, ScanTokenEnum::TK_PLUS, input);
    } else if input[p] == b'-' {
        p = token_helper(tp, p, 1, ScanTokenEnum::TK_MINUS, input);
    } else if input[p] == b'*' {
        p = token_helper(tp, p, 1, ScanTokenEnum::TK_MULT, input);
    } else if input[p] == b'/' {
        p = token_helper(tp, p, 1, ScanTokenEnum::TK_DIV, input);
    } else if input[p] == b'>' {
        if input[p + 1] == b'>' {
            p = token_helper(tp, p, 2, ScanTokenEnum::TK_SHIFT_RIGHT, input);
        } else if input[p + 1] == b'-' {
            p = token_helper(tp, p, 2, ScanTokenEnum::TK_ARITH_SHIFT_RIGHT, input);
        }
    } else if input[p] == b'<' {
        if input[p + 1] == b'<' {
            p = token_helper(tp, p, 2, ScanTokenEnum::TK_SHIFT_LEFT, input);
        }
    } else if input[p] == b'&' {
        p = token_helper(tp, p, 1, ScanTokenEnum::TK_BIT_AND, input);
    } else if input[p] == b'|' {
        p = token_helper(tp, p, 1, ScanTokenEnum::TK_BIT_OR, input);
    } else if input[p] == b'^' {
        p = token_helper(tp, p, 1, ScanTokenEnum::TK_BIT_XOR, input);
    } else if input[p] == b'~' {
        p = token_helper(tp, p, 1, ScanTokenEnum::TK_BIT_NOT, input);
    } else if input[p] == b'(' {
        p = token_helper(tp, p, 1, ScanTokenEnum::TK_LPAREN, input);
    } else if input[p] == b')' {
        p = token_helper(tp, p, 1, ScanTokenEnum::TK_RPAREN, input);
    } else {
        // Instead of returning an error code, we will usually
        // exit on failure.
        println!("scan error: invalid char: {}", input[p] as char);
        std::process::exit(-1);
    }
    p
}

pub(crate) fn scan_table_scan(st: &mut ScanTableSt, input: &[u8]) {
    let mut p = 0;
    let end = input.len();
    let mut len;
    let mut tp;

    loop {
        tp = st.new_token();
        len = end;
        p = token(p, end, input, tp);
        if tp.id == ScanTokenEnum::TK_EOT {
            break;
        }
    }
}