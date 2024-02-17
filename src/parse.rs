use std::process;
use scan::{ScanTableSt, ScanToken};

#[derive(Debug, Clone)]
pub enum ParseOperator {
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
}

#[derive(Debug,Clone)]
pub enum ParseNodeType{
    Literal,
    Oper1,
    Oper2,
    None
}
#[derive(Debug,Clone)]
pub struct ParseNode {
    pub(crate) type_: ParseNodeType,
    pub(crate) value: i32,
    pub(crate) oper : ParseOperator,
    pub(crate) left: Option<Box<ParseNode>>,
    pub(crate) right: Option<Box<ParseNode>>,
}
impl ParseNode {
    fn new(value: i32) -> Self {
        ParseNode {
            type_: ParseNodeType::Literal,
            value,
            oper: ParseOperator::Plus,
            left: None,
            right: None,
        }
    }


}

pub struct ParseTableSt {
    pub table: Vec<ParseNode>, // Adjust the size as needed
    pub len: usize,
}

impl ParseTableSt {
    pub fn new() -> Self {
        ParseTableSt {
            table: Vec::new(),
            len: 0
        }
    }

    pub fn parse_node_new(&mut self) -> ParseNode {
        let node = ParseNode {
            type_: ParseNodeType::None,
            value: 0,
            oper: ParseOperator::Plus,
            left: None,
            right: None,
        };
        self.table.push(node.clone());
        self.len += 1;
        node
    }

    pub fn parse_program(&mut self, st: &mut ScanTableSt) -> Option<ParseNode> {
        let np1 = self.parse_expression(st)?;

        if !st.accept(ScanToken::EOT) {
            parse_error("Expecting EOT");
        }

        Some(np1)
    }

    pub fn parse_expression(&mut self, st: &mut ScanTableSt) -> Option<ParseNode> {
        let mut np1 = self.parse_operand(st)?;

        loop {
            let token = st.get(0);
            let is_operator = matches!(
                token.unwrap().id,
                ScanToken::Plus
                    | ScanToken::Minus
                    | ScanToken::Mult
                    | ScanToken::Div
                    | ScanToken::ShiftRight
                    | ScanToken::ShiftLeft
                    | ScanToken::ArithShiftRight
                    | ScanToken::BitAnd
                    | ScanToken::BitOr
                    | ScanToken::BitXor
            );

            if is_operator {
                let operator = match token.unwrap().id {
                    ScanToken::Plus => Some(ParseOperator::Plus),
                    ScanToken::Minus => Some(ParseOperator::Minus),
                    ScanToken::Mult => Some(ParseOperator::Mult),
                    ScanToken::Div => Some(ParseOperator::Div),
                    ScanToken::ShiftRight => Some(ParseOperator::ShiftRight),
                    ScanToken::ShiftLeft => Some(ParseOperator::ShiftLeft),
                    ScanToken::ArithShiftRight => Some(ParseOperator::ArithShiftRight),
                    ScanToken::BitAnd => Some(ParseOperator::BitAnd),
                    ScanToken::BitOr => Some(ParseOperator::BitOr),
                    ScanToken::BitXor => Some(ParseOperator::BitXor),
                    _ => unreachable!(),
                };
                st.accept(ScanToken::Any); // Consume the operator token
                let mut np2 = self.parse_node_new();

                np2.type_= ParseNodeType::Oper2;
                np2.oper = operator.unwrap();
                np2.left = Some(Box::new(np1));
                np2.right = Some(Box::new(self.parse_operand(st)?));
                np1 = np2;
            } else {
                break;
            }
        }
        Some(np1)
    }



    fn get_operator(&self, token: ScanToken) -> Option<ParseOperator> {
        match token {
            ScanToken::Plus => Some(ParseOperator::Plus),
            ScanToken::Minus => Some(ParseOperator::Minus),
            ScanToken::Mult => Some(ParseOperator::Mult),
            ScanToken::Div => Some(ParseOperator::Div),
            ScanToken::ShiftRight => Some(ParseOperator::ShiftRight),
            ScanToken::ShiftLeft => Some(ParseOperator::ShiftLeft),
            ScanToken::ArithShiftRight => Some(ParseOperator::ArithShiftRight),
            ScanToken::BitAnd => Some(ParseOperator::BitAnd),
            ScanToken::BitOr => Some(ParseOperator::BitOr),
            ScanToken::BitXor => Some(ParseOperator::BitXor),
            _ => None,
        }
    }

    pub fn parse_operand(&mut self, st: &mut ScanTableSt) -> Option<ParseNode> {
        if st.accept(ScanToken::IntLit) {
            Some(self.parse_literal_value(st, 10))
        } else if st.accept(ScanToken::HexLit) {
            Some(self.parse_literal_value(st, 16))
        } else if st.accept(ScanToken::BinLit) {
            Some(self.parse_literal_value(st, 2))
        } else if st.accept(ScanToken::Minus) {
            let mut np1 = self.parse_node_new();

            np1.type_= ParseNodeType::Oper1;
            np1.oper = self.get_operator(ScanToken::Minus).unwrap();
            np1.left = Some(Box::new(self.parse_operand(st)?));
            Some(np1)
        } else if st.accept(ScanToken::BitNot) {
            let mut np1 = self.parse_node_new();

            np1.type_= ParseNodeType::Oper1;
            np1.oper = self.get_operator(ScanToken::BitNot).unwrap();
            np1.left = Some(Box::new(self.parse_operand(st)?));
            Some(np1)
        } else if st.accept(ScanToken::LParen) {
            let np1 = self.parse_expression(st)?;
            if !st.accept(ScanToken::RParen) {
                parse_error("Expecting ')'");
            }
            Some(np1)
        } else {
            parse_error("Bad operand");
            None
        }
    }

    fn parse_literal_value(&mut self, st: &mut ScanTableSt, base: u32) -> ParseNode {
        let token = st.getLast(1).unwrap(); // Get the last scanned token
        let value = match base {
            10 => parse_literal_value_base_10(token.value.as_str()),
            16 => parse_literal_value_base_16(token.value.as_str()),
            2 => parse_literal_value_base_2(token.value.as_str()),
            _ => panic!("Unsupported base"),
        };
        let mut np1 = self.parse_node_new();

        np1.type_= ParseNodeType::Literal;
        np1.value = value as i32;
        np1
    }
}

fn parse_literal_value_base_10(value: &str) -> u32 {
    value.parse().unwrap() // Parse the string as u32
}

fn parse_literal_value_base_16(value: &str) -> u32 {
    u32::from_str_radix(value.trim_start_matches("0x"), 16).unwrap() // Parse hexadecimal
}

fn parse_literal_value_base_2(value: &str) -> u32 {
    u32::from_str_radix(value.trim_start_matches("0b"), 2).unwrap() // Parse binary
}

fn parse_error(err: &str) {
    println!("{}", err);
    process::exit(-1);
}

pub fn print_parse_tree(node: &ParseNode) {
    match node.type_ {
        ParseNodeType::Literal => println!("Literal: {}", node.value),
        ParseNodeType::Oper1 => {
            println!("Operator: {:?}", node.oper);
            if let Some(ref left) = node.left {
                print_parse_tree(left);
            }
        }
        ParseNodeType::Oper2 => {
            println!("Operator: {:?}", node.oper);
            if let Some(ref left) = node.left {
                print_parse_tree(left);
            }
            if let Some(ref right) = node.right {
                print_parse_tree(right);
            }
        }
        ParseNodeType::None => println!("Empty Node"),
    }
}
