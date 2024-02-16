// // eval.rs - Evaluation of parse tree
//
// use parse::NodeType;
// use crate::parse::{ParseNode, Operation};
//
// pub fn evaluate(parse_tree: &ParseNode) -> u32 {
//     match &parse_tree.node_type {
//         NodeType::Literal => parse_tree.literal_value.unwrap(),
//         NodeType::UnaryOperation(operation) => {
//             let operand = evaluate(&parse_tree.unary_operand.as_ref().unwrap());
//             match operation {
//                 Operation::Minus => -operand,
//                 Operation::BitNot => !operand,
//                 _ => unreachable!(),
//             }
//         }
//         NodeType::BinaryOperation(operation) => {
//             let (left_operand, right_operand) = (
//                 evaluate(&parse_tree.binary_operands.as_ref().unwrap().0),
//                 evaluate(&parse_tree.binary_operands.as_ref().unwrap().1),
//             );
//             match operation {
//                 Operation::Plus => left_operand + right_operand,
//                 Operation::Minus => left_operand - right_operand,
//                 Operation::Multiply => left_operand * right_operand,
//                 Operation::Divide => left_operand / right_operand,
//                 Operation::ShiftRight => left_operand >> right_operand,
//                 Operation::ShiftLeft => left_operand << right_operand,
//                 Operation::ArithShiftRight => ((left_operand as i32) >> right_operand) as u32,
//                 Operation::BitAnd => left_operand & right_operand,
//                 Operation::BitOr => left_operand | right_operand,
//                 Operation::BitXor => left_operand ^ right_operand,
//                 _ => unreachable!(),
//             }
//         }
//         _ => unreachable!(),
//     }
// }
