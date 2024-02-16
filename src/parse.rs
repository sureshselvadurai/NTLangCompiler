// parse.rs - parsing and parse tree construction

use crate::scan::{ScanTableSt,ScanToken};



#[derive(Clone)]
pub enum NodeType {
    Expression,
    Literal,
    UnaryOperation(Operation),
    BinaryOperation(Operation),
}

#[derive(Clone)]
pub enum Operation {
    Plus,
    Minus,
    Multiply,
    Divide,
    ShiftRight,
    ShiftLeft,
    ArithShiftRight,
    BitAnd,
    BitOr,
    BitXor,
    BitNot,
}

pub struct ParseTable {
    table: Vec<ParseNode>,
    len: usize,
}

#[derive(Clone)]
pub struct ParseNode {
    pub node_type: NodeType,
    pub literal_value: Option<u32>,
    pub unary_operand: Option<Box<ParseNode>>,
    pub binary_operands: Option<(Box<ParseNode>, Box<ParseNode>)>,
}
impl ParseTable {
    pub fn new() -> Self {
        Self {
            table: Vec::new(),
            len: 0,
        }
    }

    pub fn parse(&mut self, scan_table: &mut ScanTableSt) -> Option<ParseNode> {
        let parse_tree = self.parse_program(scan_table);
        if scan_table.accept(ScanToken::EOT) {
            Some(parse_tree)
        } else {
            println!("Expecting EOT");
            None
        }
    }

    fn new_node(&mut self, node_type: NodeType) -> ParseNode {
        let node = ParseNode {
            node_type,
            literal_value: None,
            unary_operand: None,
            binary_operands: None,
        };
        self.table.push(node.clone());
        self.len += 1;
        node
    }

    fn parse_program(&mut self, scan_table: &mut ScanTableSt) -> ParseNode {
        self.parse_expression(scan_table)
    }

    fn parse_expression(&mut self, scan_table: &mut ScanTableSt) -> ParseNode {
        let mut node = self.parse_operand(scan_table);

        loop {
            let token = scan_table.get(0);
            let is_operator = matches!(
                token.id,
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
                let oper = match token.id {
                    ScanToken::Plus => Operation::Plus,
                    ScanToken::Minus => Operation::Minus,
                    ScanToken::Mult => Operation::Multiply,
                    ScanToken::Div => Operation::Divide,
                    ScanToken::ShiftRight => Operation::ShiftRight,
                    ScanToken::ShiftLeft => Operation::ShiftLeft,
                    ScanToken::ArithShiftRight => Operation::ArithShiftRight,
                    ScanToken::BitAnd => Operation::BitAnd,
                    ScanToken::BitOr => Operation::BitOr,
                    ScanToken::BitXor => Operation::BitXor,
                    _ => unreachable!(),
                };

                let mut binary_node = self.new_node(NodeType::BinaryOperation(oper));
                binary_node.binary_operands = Some((Box::new(node), Box::new(self.parse_operand(scan_table))));
                node = binary_node;
            } else {
                break;
            }
        }

        node
    }

    fn parse_operand(&mut self, scan_table: &mut ScanTableSt) -> ParseNode {
        let token = scan_table.get(0);
        match token.id {
            ScanToken::IntLit => {
                let value = token.value.parse().unwrap();
                self.new_node(NodeType::Literal).with_literal_value(value)
            }
            ScanToken::HexLit => {
                let value = u32::from_str_radix(&token.value[2..], 16).unwrap();
                self.new_node(NodeType::Literal).with_literal_value(value)
            }
            ScanToken::BinLit => {
                let value = u32::from_str_radix(&token.value[2..], 2).unwrap();
                self.new_node(NodeType::Literal).with_literal_value(value)
            }
            ScanToken::Minus => {
                let mut unary_node = self.new_node(NodeType::UnaryOperation(Operation::Minus));
                unary_node.unary_operand = Some(Box::new(self.parse_operand(scan_table)));
                unary_node
            }
            ScanToken::BitNot => {
                let mut unary_node = self.new_node(NodeType::UnaryOperation(Operation::BitNot));
                unary_node.unary_operand = Some(Box::new(self.parse_operand(scan_table)));
                unary_node
            }
            ScanToken::LParen => {
                let expression_node = self.parse_expression(scan_table);
                if !scan_table.accept(ScanToken::RParen) {
                    println!("Expecting ')'");
                }
                expression_node
            }
            _ => {
                println!("Bad operand");
                unreachable!()
            }
        }
    }
}

impl ParseNode {
    fn with_literal_value(mut self, value: u32) -> Self {
        self.literal_value = Some(value);
        self
    }
}
