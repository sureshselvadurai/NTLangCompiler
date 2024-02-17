// parse.rs - parsing and parse tree construction

use crate::scan::{ScanTableSt,ScanToken};



#[derive(Clone, Debug)]
pub enum NodeType {
    Literal,
    UnaryOperation,
    BinaryOperation,
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

#[derive(Clone, Debug)]
pub struct ParseNode {
    pub node_type: NodeType,
    pub literal_value: Option<u32>,
    pub left: Option<Box<ParseNode>>,
    pub right: Option<Box<ParseNode>>,
}

// Define the trait
trait NodeTrait {
    fn display_info(&self);
}

// Implement the trait for ParseNode
impl NodeTrait for ParseNode {
    fn display_info(&self) {
        // Print information about the node
        println!("Node Type: {:?}", self.node_type);
        println!("Literal Value: {:?}", self.literal_value);
        println!("Left: {:?}", self.left);
        println!("Right: {:?}", self.right);
    }
}

fn convert_to_box_option(node_option: Option<ParseNode>) -> Option<Box<ParseNode>> {
    // Step 1: Unwrap the Option to get the ParseNode if it exists
    if let Some(node) = node_option {
        // Step 2: Box the ParseNode
        let boxed_node: Box<ParseNode> = Box::new(node);
        // Step 3: Create an Option containing the Boxed ParseNode
        let boxed_option = Some(boxed_node);
        // Step 4: Add a comment to clarify the purpose of the conversion
        // Convert ParseNode to Option<Box<ParseNode>> for dynamic dispatch
        // Step 5: Return the Option<Box<ParseNode>>
        return boxed_option;
    }
    // If the Option is None, return None
    None
}





impl ParseNode {
    fn new(node: Option<ParseNode>) -> Option<ParseNode> {
        node
    }
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
            parse_tree
        } else {
            println!("Expecting EOT");
            None
        }
    }

    fn new_node(&mut self, node_type: NodeType) -> Option<ParseNode> {
        let node = ParseNode {
            node_type,
            literal_value: None,
            left: None,
            right: None,
        };
        self.table.push(node.clone());
        self.len += 1;
        Some(node)
    }

    fn new_node_with_value(&mut self, node_type: NodeType, literal_value: Option<u32>) -> Option<ParseNode> {
        let node = ParseNode {
            node_type,
            literal_value,
            left: None,
            right: None,
        };
        self.table.push(node.clone());
        self.len += 1;
        Some(node)
    }


    fn parse_program(&mut self, scan_table: &mut ScanTableSt) -> Option<ParseNode> {
        self.parse_expression(scan_table)
    }

    fn parse_expression(&mut self, scan_table: &mut ScanTableSt) -> Option<ParseNode> {
        let mut node = self.parse_operand(scan_table);

        loop {
            let token = scan_table.get(0);
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
                let oper = match token.unwrap().id {
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
                scan_table.accept(ScanToken::Any);

                let mut binary_node = match self.new_node(NodeType::BinaryOperation) {
                    Some(node) => node,
                    None => return None,
                };

                let test = convert_to_box_option(node);
                // binary_node.left = convert_to_box_option(node);
                binary_node.right = convert_to_box_option(self.parse_operand(scan_table));

                node = Some(binary_node);
            } else {
                break;
            }
        }
        node
    }

    fn parse_operand(&mut self, scan_table: &mut ScanTableSt) -> Option<ParseNode> {
        let token_value;
        let token = {
            token_value = scan_table.get(0).unwrap().value.clone();
            scan_table.get(0)
        };

        if scan_table.accept(ScanToken::IntLit) {
            let value = token_value.parse().unwrap();
            let test = self.new_node_with_value(NodeType::Literal, Some(value));
            return self.new_node_with_value(NodeType::Literal, Some(value));
        } else if scan_table.accept(ScanToken::HexLit) {
            let value = u32::from_str_radix(&token_value[2..], 16).unwrap();
            return self.new_node_with_value(NodeType::Literal, Some(value));
        } else if scan_table.accept(ScanToken::BinLit) {
            let value = u32::from_str_radix(&token_value[2..], 2).unwrap();
            return self.new_node_with_value(NodeType::Literal, Some(value));
        } else if scan_table.accept(ScanToken::Minus) {
            let mut unary_node = match self.new_node(NodeType::UnaryOperation) {
                Some(node) => node,
                None => return None, // Return early if new_node returns None
            };
            unary_node.left = convert_to_box_option(self.parse_operand(scan_table));
            return Some(unary_node);
        } else if scan_table.accept(ScanToken::BitNot) {
            let mut unary_node = match self.new_node(NodeType::UnaryOperation) {
                Some(node) => node,
                None => return None, // Return early if new_node returns None
            };
            unary_node.left = convert_to_box_option(self.parse_operand(scan_table));
            return Some(unary_node);
        } else if scan_table.accept(ScanToken::LParen) {
            let expression_node = self.parse_expression(scan_table);
            if !scan_table.accept(ScanToken::RParen) {
                println!("Expecting ')'");
            }
            return expression_node;
        }
        // Return None when none of the conditions are met
        None
    }
}


impl ParseNode {
    fn with_literal_value(mut self, value: u32) -> Self {
        self.literal_value = Some(value);
        self
    }
}
//