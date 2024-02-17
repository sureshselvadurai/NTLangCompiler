use std::process;
use parse::{ParseOperator,ParseNodeType,ParseNode};
use Config;

pub fn eval_error(err: &str) {
    println!("eval_error: {}", err);
    process::exit(-1);
}

pub fn eval(pt: &Option<&ParseNode>) -> u32 {
    match pt {
        Some(pt) => {
            match pt.type_ {
                ParseNodeType::Literal => pt.value as u32,
                ParseNodeType::Oper1 => {
                    let v1 = eval(&pt.left.as_ref().map(|boxed| &**boxed));
                    match pt.oper {
                        ParseOperator::Plus => v1,
                        ParseOperator::Minus => -(v1 as i32) as u32,
                        ParseOperator::BitNot => !v1,
                        _ => {
                            eval_error("Invalid unary operator");
                            0
                        }
                    }
                }
                ParseNodeType::Oper2 => {
                    let v1 = eval(&pt.left.as_ref().map(|boxed| &**boxed));
                    let v2 = eval(&pt.right.as_ref().map(|boxed| &**boxed));
                    match pt.oper {
                        ParseOperator::Plus => v1 + v2,
                        ParseOperator::Minus => v1 - v2,
                        ParseOperator::Mult => v1 * v2,
                        ParseOperator::Div => {
                            if v2 == 0 {
                                eval_error("Division by zero");
                                0
                            } else {
                                v1 / v2
                            }
                        }
                        ParseOperator::ShiftRight => v1 >> v2,
                        ParseOperator::ShiftLeft => v1 << v2,
                        ParseOperator::ArithShiftRight => ((v1 as i32) >> v2) as u32,
                        ParseOperator::BitAnd => v1 & v2,
                        ParseOperator::BitOr => v1 | v2,
                        ParseOperator::BitXor => v1 ^ v2,
                        _ => {
                            eval_error("Invalid binary operator");
                            0
                        }
                    }
                }
                ParseNodeType::None => 0,
            }
        }
        None => 0,
    }
}


pub fn eval_print(cp: &Config, value: u32) {
    let mut str = String::new();
    let mut i = 0;

    let n_bit_value = mask_value(value, cp.width);
    let sign = is_negative(n_bit_value, cp.width, cp.unsigned_int);

    match cp.base {
        10 => convert_to_decimal(n_bit_value, &mut str, &mut i, sign, cp.width),
        2 => convert_to_binary(value, &mut str, &mut i, cp.width),
        16 => convert_to_hexadecimal(value, &mut str, &mut i, cp.width),
        _ => {}
    }
    println!("{}", str);
}

fn mask_value(value: u32, width: u32) -> u32 {
    if width != 32 {
        value & ((1 << width) - 1)
    } else {
        value
    }
}

fn is_negative(n_bit_value: u32, width: u32, unsigned_int: bool) -> bool {
    if unsigned_int {
        n_bit_value & (1 << (width - 1)) != 0
    } else {
        false
    }
}

use std::mem;

fn convert_to_decimal(n_bit_value: u32, str: &mut String, i: &mut usize, sign: bool, width: u32) {
    let mut n_bit_value = if sign {
        if width != 32 {
            let mask = (1 << width) - 1;
            n_bit_value ^ mask
        } else {
            !n_bit_value + 1
        }
    } else {
        n_bit_value
    };

    if n_bit_value == 0 {
        str.push('0');
        return;
    }

    while n_bit_value != 0 {
        let remainder = (n_bit_value % 10) as u8;
        str.push((remainder + b'0') as char);
        n_bit_value /= 10;
    }
    if sign {
        str.push('-');
    }

    let reversed_str: String = str.chars().rev().collect();
    mem::replace(str, reversed_str);
}

fn convert_to_binary(value: u32, str: &mut String, i: &mut usize, width: u32) {
    for j in 0..width {
        let mask = 1 << j;
        let bit = if (value & mask) != 0 { '1' } else { '0' };

        str.push(bit);
    }

    let reversed_str: String = str.chars().rev().collect();
    mem::replace(str, reversed_str);
}

fn convert_to_hexadecimal(value: u32, str: &mut String, i: &mut usize, width: u32) {
    let mut value = if width != 32 {
        let mask = (1 << width) - 1;
        value & mask
    } else {
        value
    };

    for j in (0..width).step_by(4) {
        let mask = 0xF << j;
        let hex_digit = (value & mask) >> j;

        let digit_char = match hex_digit {
            0..=9 => (hex_digit as u8 + b'0') as char,
            10..=15 => (hex_digit as u8 - 10 + b'A') as char,
            _ => panic!("Invalid hex digit"),
        };
        str.push(digit_char);
    }

    let reversed_str: String = str.chars().rev().collect();
    mem::replace(str, reversed_str);
}
