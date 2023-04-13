
use std::fmt;

/// when an invalid operation is encountered
#[derive(Debug)]
pub enum OperationError {
    UnknownOperation,
}

/// a specific calculation operation, for example add: adds two numbers
#[derive(Debug)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    And,
    Or,
    Xor,
    Shl,
    Shr,
    Rol,
    Ror,
    Neg,
    Not,
    Reg, // register view
    End, // swap endianess
}

impl Operation {

    /// create an Operation from a string, check for errors 
    pub fn new_from_str(s: &str) -> Result<Operation, OperationError> {
        match s {
            "+" => Ok(Operation::Add),
            "-" => Ok(Operation::Sub),
            "*" => Ok(Operation::Mul),
            "&" => Ok(Operation::And),
            "|" => Ok(Operation::Or),
            "^" => Ok(Operation::Xor),
            "<<" => Ok(Operation::Shl),
            ">>" => Ok(Operation::Shr),
            "<<<" => Ok(Operation::Rol),
            ">>>" => Ok(Operation::Ror),
            "~" => Ok(Operation::Neg),
            "!" => Ok(Operation::Not),
            "r" => Ok(Operation::Reg),
            "e" => Ok(Operation::End),
            _ => Err(OperationError::UnknownOperation)
        }
    }

    pub fn show_operations() {
        println!("[+]   addition        example: 0x1u8 + 0x1u8");
        println!("[-]   subraction      example: 0x1u8 - 0x1u8");
        println!("[*]   multiplciation  example: 0x1u8 * 0x2u8");
        println!("[&]   bitwise AND     example: 0xdeadu16 & 0x03u16");
        println!("[|]   bitwise OR      example: 0x03u8 | 0x4u8 ");
        println!("[^]   bitwise XOR     example: 0xdeadbeefu32 ^ 0xbaadu32");
        println!("[<<]  shift left      example: 1u32 << 10u32");
        println!("[>>]  shift right     example: 10000u32 >> 10u32");
        println!("[<<<] rotate left     example: 1u8 <<< 16u8");
        println!("[>>>] rotate right    example: 1u8 >>> 1u8");
        println!("[~]   negate          example: ~ 0x1u8");
        println!("[!]   bitwise NOT     example: ! 0xdeadbeefu32");
        println!("[r]   regshow         example: r 0xdeadbeefc0cac01au64");
        println!("[e]   endian swap     example: e 0xdeadbeefc0cac01au64");
    }

    /// check if an operation is unary or not
    pub fn is_unary(&self) -> bool {
        matches!(self, Operation::Neg | Operation::Not
            | Operation::Reg | Operation::End)
    }

}


/// errors which can arise when converting integers
#[derive(Debug)]
pub enum IntTypeConversionError {
    InvalidInteger,
    InvalidBitwidth,
    UnsupportedBitwidth
}

impl fmt::Display for IntTypeConversionError {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {
            IntTypeConversionError::InvalidInteger => { 
                write!(f, "error: integer value is invalid")
            },
            IntTypeConversionError::InvalidBitwidth => { 
                write!(f, "error: integer bitwidth value is invalid")
            },
            IntTypeConversionError::UnsupportedBitwidth => { 
                write!(f, "error: integer bitwidth is not 8, 16, 32 or 64")
            },
        }

    } // fmt

} // impl Display

pub enum Overflow {
    NotPossible, // some operations can't overflow
    NoOverflow,  // didn't overflow
    Occured,     // overflow occured
}
impl fmt::Display for Overflow {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {
            Overflow::Occured => write!(f, "[overflow occured]"),
            Overflow::NoOverflow => write!(f, "[no overflow]"),
            Overflow::NotPossible => write!(f, ""),
        }

    } // fmt

} // impl Display

/// a type that describes different integer types used in this program
#[derive(Debug, PartialEq)]
pub enum IntType {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

impl IntType {

    /// take a string and parse it to an IntType, if the parsing fails, an
    /// error is returned
    pub fn new_from_str(s: &str) -> Result<IntType, IntTypeConversionError> {

        // check if decimal or hex
        let (s, is_hex) = if s.starts_with("0x") {
            (s.trim_start_matches("0x"), true)
        } else {
            (s, false)   
        };

        // we need each value entered to be of a specific type
        if !s.contains('u') {
            return Err(IntTypeConversionError::InvalidInteger);
        }

        // if we have something like 1u8 we need to separate the two terms
        // into 1 and 8
        let vs: Vec<&str> = s.split('u')
            .collect();

        // an integer in this program always consists of two parts after split
        if vs.len() != 2 {
            return Err(IntTypeConversionError::InvalidInteger);
        }

        // get the width of the integer
        let bits = vs[1].parse::<usize>();

        // check conversion result
        let bits = match bits {
            Ok(v) => v,
            Err(e) => {
                eprintln!("error: {e}");
                return Err(IntTypeConversionError::InvalidBitwidth);
            }
        };

        let res = match bits {
                8 => match u8::from_str_radix(vs[0], 
                        if is_hex { 16 } else { 10 }) {
                            Ok(num) => Ok(IntType::U8(num)),
                            Err(e) => {
                                eprintln!("ParseIntError: {e}");
                                Err(IntTypeConversionError::InvalidInteger)
                            }
                        },

                16 => match u16::from_str_radix(vs[0], 
                        if is_hex { 16 } else { 10 }) {
                            Ok(num) => Ok(IntType::U16(num)),
                            Err(e) => {
                                eprintln!("ParseIntError: {e}");
                                Err(IntTypeConversionError::InvalidInteger)
                            }
                        },

                32 => match u32::from_str_radix(vs[0], 
                        if is_hex { 16 } else { 10 }) {
                            Ok(num) => Ok(IntType::U32(num)),
                            Err(e) => {
                                eprintln!("ParseIntError: {e}");
                                Err(IntTypeConversionError::InvalidInteger)
                            }
                        },

                64 => match u64::from_str_radix(vs[0], 
                        if is_hex { 16 } else { 10 }) {
                            Ok(num) => Ok(IntType::U64(num)),
                            Err(e) => {
                                eprintln!("ParseIntError: {e}");
                                Err(IntTypeConversionError::InvalidInteger)
                            }
                        },
                _ => Err(IntTypeConversionError::UnsupportedBitwidth)
            };

        res 
    }

    // calculation of a binary operation, given two inttypes and an operation
    pub fn calculate_binary(self, other: IntType, op: Operation) -> Option<(IntType, Overflow)> {

        // return the result and if an overflow occured
        let res = match op {

            Operation::Add => {
                match (self, other) {
                    (Self::U8(v), Self::U8(u)) => {
                        let (val, overflow) = v.overflowing_add(u);
                        ( IntType::U8( val ), Some(overflow) )
                    },
                    (Self::U16(v), Self::U16(u)) => {
                        let (val, overflow) = v.overflowing_add(u);
                        ( IntType::U16( val ), Some(overflow) )
                    },
                    (Self::U32(v), Self::U32(u)) => {
                        let (val, overflow) = v.overflowing_add(u);
                        ( IntType::U32( val ), Some(overflow))
                    },
                    (Self::U64(v), Self::U64(u)) => {
                        let (val, overflow) = v.overflowing_add(u);
                        ( IntType::U64( val ), Some(overflow))
                    },
                    _ => panic!("error"),
                }
            },

            Operation::Sub => {
                match (self, other) {
                    (Self::U8(v), Self::U8(u)) => {
                        let (val, overflow) = v.overflowing_sub(u);
                        ( IntType::U8( val ), Some(overflow))
                    },
                    (Self::U16(v), Self::U16(u)) => {
                        let (val, overflow) = v.overflowing_sub(u);
                        ( IntType::U16( val ), Some(overflow))
                    },
                    (Self::U32(v), Self::U32(u)) => {
                        let (val, overflow) = v.overflowing_sub(u);
                        ( IntType::U32( val ), Some(overflow))
                    },
                    (Self::U64(v), Self::U64(u)) => {
                        let (val, overflow) = v.overflowing_sub(u);
                        ( IntType::U64( val ), Some(overflow))
                    },
                    _ => panic!("error"),
                }
            },

            Operation::Mul => {
                match (self, other) {
                    (Self::U8(v), Self::U8(u)) => {
                        let (val, overflow) = v.overflowing_mul(u);
                        ( IntType::U8( val ), Some(overflow))
                    },
                    (Self::U16(v), Self::U16(u)) => {
                        let (val, overflow) = v.overflowing_mul(u);
                        ( IntType::U16( val ), Some(overflow))
                    },
                    (Self::U32(v), Self::U32(u)) => {
                        let (val, overflow) = v.overflowing_mul(u);
                        ( IntType::U32( val ), Some(overflow))
                    },
                    (Self::U64(v), Self::U64(u)) => {
                        let (val, overflow) = v.overflowing_mul(u);
                        ( IntType::U64( val ), Some(overflow))
                    },
                    _ => panic!("error"),
                }
            },

            Operation::And => {
                match (self, other) {
                    (Self::U8(v), Self::U8(u)) => {
                        (IntType::U8( v & u ), None)
                    },
                    (Self::U16(v), Self::U16(u)) => {
                        (IntType::U16( v & u ), None)
                    },
                    (Self::U32(v), Self::U32(u)) => {
                        (IntType::U32( v & u ), None)
                    },
                    (Self::U64(v), Self::U64(u)) => {
                        (IntType::U64( v & u ), None)
                    },
                    _ => panic!("error"),
                }
            },

            Operation::Or => {
                match (self, other) {
                    (Self::U8(v), Self::U8(u)) => {
                        (IntType::U8( v | u ), None)
                    },
                    (Self::U16(v), Self::U16(u)) => {
                        (IntType::U16( v | u ), None)
                    },
                    (Self::U32(v), Self::U32(u)) => {
                        (IntType::U32( v | u ), None)
                    },
                    (Self::U64(v), Self::U64(u)) => {
                        (IntType::U64( v | u ), None)
                    },
                    _ => panic!("error"),
                }
            },

            Operation::Xor => {
                match (self, other) {
                    (Self::U8(v), Self::U8(u)) => {
                        (IntType::U8( v ^ u ), None)
                    },
                    (Self::U16(v), Self::U16(u)) => {
                        (IntType::U16( v ^ u ), None)
                    },
                    (Self::U32(v), Self::U32(u)) => {
                        (IntType::U32( v ^ u ), None)
                    },
                    (Self::U64(v), Self::U64(u)) => {
                        (IntType::U64( v ^ u ), None)
                    },
                    _ => panic!("error"),
                }
            },

            Operation::Shl => {
                match (self, other) {
                    (Self::U8(v), Self::U8(u)) => {
                        (IntType::U8( v.checked_shl(u as u32).unwrap_or(0) ), None)
                    },
                    (Self::U16(v), Self::U16(u)) => {
                        (IntType::U16( v.checked_shl(u as u32).unwrap_or(0) ), None)
                    },
                    (Self::U32(v), Self::U32(u)) => {
                        (IntType::U32( v.checked_shl(u).unwrap_or(0) ), None)
                    },
                    (Self::U64(v), Self::U64(u)) => {
                        (IntType::U64( v.checked_shl(u as u32).unwrap_or(0) ), None)
                    },
                    _ => panic!("error"),
                }
            },

            Operation::Shr => {
                match (self, other) {
                    (Self::U8(v), Self::U8(u)) => {
                        (IntType::U8( v.checked_shr(u as u32).unwrap_or(0) ), None)
                    },
                    (Self::U16(v), Self::U16(u)) => {
                        (IntType::U16( v.checked_shr(u as u32).unwrap_or(0) ), None)
                    },
                    (Self::U32(v), Self::U32(u)) => {
                        (IntType::U32( v.checked_shr(u).unwrap_or(0) ), None)
                    },
                    (Self::U64(v), Self::U64(u)) => {
                        (IntType::U64( v.checked_shr(u as u32).unwrap_or(0) ), None)
                    },
                    _ => panic!("error"),
                }
            },

            Operation::Rol => {
                match (self, other) {
                    (Self::U8(v), Self::U8(u)) => {
                        (IntType::U8( v.rotate_left(u as u32) ), None)
                    },
                    (Self::U16(v), Self::U16(u)) => {
                        (IntType::U16( v.rotate_left(u as u32) ), None)
                    },
                    (Self::U32(v), Self::U32(u)) => {
                        (IntType::U32( v.rotate_left(u) ), None)
                    },
                    (Self::U64(v), Self::U64(u)) => {
                        (IntType::U64( v.rotate_left(u as u32) ), None)
                    },
                    _ => panic!("error"),
                }
            },

            Operation::Ror => {
                match (self, other) {
                    (Self::U8(v), Self::U8(u)) => {
                        (IntType::U8( v.rotate_right(u as u32) ), None)
                    },
                    (Self::U16(v), Self::U16(u)) => {
                        (IntType::U16( v.rotate_right(u as u32) ), None)
                    },
                    (Self::U32(v), Self::U32(u)) => {
                        (IntType::U32( v.rotate_right(u) ), None)
                    },
                    (Self::U64(v), Self::U64(u)) => {
                        (IntType::U64( v.rotate_right(u as u32) ), None)
                    },
                    _ => panic!("error"),
                }
            },

            _ => panic!("error")

        };

        let overflow = if let Some(overflow) = res.1 {
            // overflows are possible, check if occured or not
            if overflow {
                Overflow::Occured
            } else {
                Overflow::NoOverflow
            }
        } else {
            Overflow::NotPossible
        };

        Some((res.0, overflow))
    }

    /// calculate an unary operation given an IntType and an operation
    pub fn calculate_unary(self, op: Operation) -> Option<(IntType, Overflow)> {

        let res = match op {
            Operation::Neg => {
                match self {
                    IntType::U8(v) => IntType::U8(v.wrapping_neg()),
                    IntType::U16(v) => IntType::U16(v.wrapping_neg()),
                    IntType::U32(v) => IntType::U32(v.wrapping_neg()),
                    IntType::U64(v) => IntType::U64(v.wrapping_neg()),
                }
                
            },
            Operation::Not => {
                match self {
                    IntType::U8(v) => IntType::U8(!v),
                    IntType::U16(v) => IntType::U16(!v),
                    IntType::U32(v) => IntType::U32(!v),
                    IntType::U64(v) => IntType::U64(!v),
                }
            },
            Operation::Reg => {
                match self {
                    IntType::U8(v) => {
                        regprint(v as u64, 8);
                        IntType::U8(v)
                    },
                    IntType::U16(v) => {
                        regprint(v as u64, 16);
                        IntType::U16(v)
                    },
                    IntType::U32(v) => {
                        regprint(v as u64, 32);
                        IntType::U32(v)
                    },
                    IntType::U64(v) => {
                        regprint(v, 64);
                        IntType::U64(v)
                    },
                }
            },
            Operation::End => {
                match self {
                    IntType::U8(v) => {
                        IntType::U8(v.swap_bytes())
                    },
                    IntType::U16(v) => {
                        IntType::U16(v.swap_bytes())
                    },
                    IntType::U32(v) => {
                        IntType::U32(v.swap_bytes())
                    },
                    IntType::U64(v) => {
                        IntType::U64(v.swap_bytes())
                    },
                }
            }
            _ => panic!("error"),
        };

        Some((res, Overflow::NotPossible))
    }

}

impl fmt::Display for IntType {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {
            IntType::U8(v) => {
                write!(f, "hex: 0x{:02x} bin: b{:b} dec: {}", v, v, v)
            },
            IntType::U16(v) => {
                write!(f, "hex: 0x{:04x} bin: b{:b} dec: {}", v, v, v)
            },
            IntType::U32(v) => {
                write!(f, "hex: 0x{:08x} bin: b{:b} dec: {}", v, v, v)
            },
            IntType::U64(v) => {
                write!(f, "hex: 0x{:16x} bin: b{:b} dec: {}", v, v, v)
            },
        }

    } // fmt

} // impl Display

/// print a `value` as if it were a value in an `iter_max`-bit register.
fn regprint(value: u64, iter_max: usize) {

    if iter_max == 64 {
        for i in (32..iter_max).rev() {
            print!("{i} ");
        }
        println!();
        // print value
        for i in (32..iter_max).rev() {
            let bit = (value >> i) & 0x01;
            if i >= 10 {
                print!("{bit}  ");
            } else {
                print!("{bit} ");
            }
        }
        println!();
        for i in (0..32).rev() {
            print!("{i} ");
        }
        println!();
        // print value
        for i in (0..32).rev() {
            let bit = (value >> i) & 0x01;
            if i >= 10 {
                print!("{bit}  ");
            } else {
                print!("{bit} ");
            }
        }
    } else {
        for i in (0..iter_max).rev() {
            print!("{i} ");
        }
        println!();
        // print value
        for i in (0..iter_max).rev() {
            let bit = (value >> i) & 0x01;
            if i >= 10 {
                print!("{bit}  ");
            } else {
                print!("{bit} ");
            }
        }
    }
    println!();
}

pub fn evaluate(s: &str) -> Option<(IntType, Overflow)> {
    
    let vs: Vec<&str> = s
        .trim_end()
        .split(' ')
        .collect();

    match vs.len() {
        2 => {
            // unary operation
            let op = Operation::new_from_str(vs[0]);
            let v0 = IntType::new_from_str(vs[1]);

            if v0.is_err() || op.is_err() {
                return None;
            }

            let op = op.unwrap();

            if !op.is_unary() {
                return None;
            } else {
                return v0.unwrap().calculate_unary(op);
            }
        },
        3 => {
            // binary operation
            let v0 = IntType::new_from_str(vs[0]);
            let op = Operation::new_from_str(vs[1]);
            let v1 = IntType::new_from_str(vs[2]);

            if v0.is_err() || op.is_err() || v1.is_err() {
                return None;
            }

            // SAFETY: checked above
            let op = op.unwrap();

            if op.is_unary() {
                return None;
            } else {
                // SAFETY: checked above            
                return v0.unwrap().calculate_binary(v1.unwrap(), op);
            }
        },
        _ => {
            //eprintln!("[bt]> error operand count {} invalid", vs.len());
        }
    };

    None
}

pub fn show_help() {
    println!("\n\nHELP:\n");
    println!("use [-h] to list this help\n");
    println!("use [-i] to enter interactive mode\n");
    println!("available operations: ");
    Operation::show_operations();
    println!("\n\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inttype_new_from_str() {

        assert!(IntType::new_from_str("1u8").is_ok());
        assert!(IntType::new_from_str("1u16").is_ok());
        assert!(IntType::new_from_str("1u32").is_ok());
        assert!(IntType::new_from_str("1u64").is_ok());

        assert!(IntType::new_from_str("1").is_err());
        assert!(IntType::new_from_str("a").is_err());
        assert!(IntType::new_from_str("au8").is_err());
        assert!(IntType::new_from_str("1u").is_err());
        assert!(IntType::new_from_str("1u33").is_err());

    }

    #[test]
    fn operation_new_from_str() {

        assert!(Operation::new_from_str("+").is_ok());
        assert!(Operation::new_from_str("-").is_ok());
        assert!(Operation::new_from_str("*").is_ok());
        assert!(Operation::new_from_str("&").is_ok());
        assert!(Operation::new_from_str("|").is_ok());
        assert!(Operation::new_from_str("^").is_ok());
        assert!(Operation::new_from_str("<<").is_ok());
        assert!(Operation::new_from_str(">>").is_ok());
        assert!(Operation::new_from_str("<<<").is_ok());
        assert!(Operation::new_from_str(">>>").is_ok());
        assert!(Operation::new_from_str("~").is_ok());
        assert!(Operation::new_from_str("!").is_ok());
        assert!(Operation::new_from_str("r").is_ok());

        assert!(Operation::new_from_str("x").is_err());

    }

    #[test]
    fn operation_is_unary() {

        assert!(!Operation::new_from_str("+").unwrap().is_unary());
        assert!(!Operation::new_from_str("-").unwrap().is_unary());
        assert!(!Operation::new_from_str("*").unwrap().is_unary());
        assert!(!Operation::new_from_str("&").unwrap().is_unary());
        assert!(!Operation::new_from_str("|").unwrap().is_unary());
        assert!(!Operation::new_from_str("^").unwrap().is_unary());
        assert!(!Operation::new_from_str("<<").unwrap().is_unary());
        assert!(!Operation::new_from_str(">>").unwrap().is_unary());
        assert!(!Operation::new_from_str("<<<").unwrap().is_unary());
        assert!(!Operation::new_from_str(">>>").unwrap().is_unary());
        assert!(Operation::new_from_str("~").unwrap().is_unary());
        assert!(Operation::new_from_str("!").unwrap().is_unary());
        assert!(Operation::new_from_str("r").unwrap().is_unary());

        assert!(Operation::new_from_str("x").is_err());

    }

    #[test]
    fn evaluate_test() {
        assert!(evaluate("1u8 + 1u8").is_some());
        assert!(evaluate("1u8 - 1u8").is_some());
        assert!(evaluate("1u8 * 1u8").is_some());
        assert!(evaluate("1u8 & 1u8").is_some());
        assert!(evaluate("1u8 | 1u8").is_some());
        assert!(evaluate("1u8 ^ 1u8").is_some());
        assert!(evaluate("1u8 << 1u8").is_some());
        assert!(evaluate("1u8 >> 1u8").is_some());
        assert!(evaluate("1u8 <<< 1u8").is_some());
        assert!(evaluate("1u8 >>> 1u8").is_some());
        assert!(evaluate("! 1u8").is_some());
        assert!(evaluate("~ 1u8").is_some());
        assert!(evaluate("r 1u8").is_some());
        assert!(evaluate("e 1u8").is_some());

        assert!(evaluate("1u16 + 1u16").is_some());
        assert!(evaluate("1u16 - 1u16").is_some());
        assert!(evaluate("1u16 * 1u16").is_some());
        assert!(evaluate("1u16 & 1u16").is_some());
        assert!(evaluate("1u16 | 1u16").is_some());
        assert!(evaluate("1u16 ^ 1u16").is_some());
        assert!(evaluate("1u16 << 1u16").is_some());
        assert!(evaluate("1u16 >> 1u16").is_some());
        assert!(evaluate("1u16 <<< 1u16").is_some());
        assert!(evaluate("1u16 >>> 1u16").is_some());
        assert!(evaluate("! 1u16").is_some());
        assert!(evaluate("~ 1u16").is_some());
        assert!(evaluate("r 1u16").is_some());
        assert!(evaluate("e 1u16").is_some());

        assert!(evaluate("1u32 + 1u32").is_some());
        assert!(evaluate("1u32 - 1u32").is_some());
        assert!(evaluate("1u32 * 1u32").is_some());
        assert!(evaluate("1u32 & 1u32").is_some());
        assert!(evaluate("1u32 | 1u32").is_some());
        assert!(evaluate("1u32 ^ 1u32").is_some());
        assert!(evaluate("1u32 << 1u32").is_some());
        assert!(evaluate("1u32 >> 1u32").is_some());
        assert!(evaluate("1u32 <<< 1u32").is_some());
        assert!(evaluate("1u32 >>> 1u32").is_some());
        assert!(evaluate("! 1u32").is_some());
        assert!(evaluate("~ 1u32").is_some());
        assert!(evaluate("r 1u32").is_some());
        assert!(evaluate("e 1u32").is_some());

        assert!(evaluate("1u64 + 1u64").is_some());
        assert!(evaluate("1u64 - 1u64").is_some());
        assert!(evaluate("1u64 * 1u64").is_some());
        assert!(evaluate("1u64 & 1u64").is_some());
        assert!(evaluate("1u64 | 1u64").is_some());
        assert!(evaluate("1u64 ^ 1u64").is_some());
        assert!(evaluate("1u64 << 1u64").is_some());
        assert!(evaluate("1u64 >> 1u64").is_some());
        assert!(evaluate("1u64 <<< 1u64").is_some());
        assert!(evaluate("1u64 >>> 1u64").is_some());
        assert!(evaluate("! 1u64").is_some());
        assert!(evaluate("~ 1u64").is_some());
        assert!(evaluate("r 1u64").is_some());
        assert!(evaluate("e 1u64").is_some());

        assert!(evaluate("1u64 ++ 1u64").is_none());
        assert!(evaluate("1u64 - u64").is_none());
        assert!(evaluate("u64 * 1u64").is_none());
        assert!(evaluate("1u64 x 1u64").is_none());
        assert!(evaluate("1au64 | 1u64").is_none());
        assert!(evaluate("! 1u6").is_none());
        assert!(evaluate("~ -1u64").is_none());
        assert!(evaluate("rr 1u64").is_none());
    }
}
