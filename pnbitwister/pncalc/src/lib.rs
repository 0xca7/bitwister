pub enum ParseError {
    ParseNumber,
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Xor,
    Shl,
    Shr,
    Rol,
    Ror,
    Neg,
}

impl Operator {
    fn from_str(input: &str) -> Option<Operator> {
        match input {
            "+" => Some(Operator::Add),
            "-" => Some(Operator::Sub),
            "*" => Some(Operator::Mul),
            "/" => Some(Operator::Div),
            "&" => Some(Operator::And),
            "|" => Some(Operator::Or),
            "^" => Some(Operator::Xor),
            "<<" => Some(Operator::Shl),
            ">>" => Some(Operator::Shr),
            "<<<" => Some(Operator::Rol),
            ">>>" => Some(Operator::Ror),
            "!" => Some(Operator::Neg),
            _   => None
        }
    }
    fn is_unary(&self) -> bool {
        matches!(self, Operator::Neg)
    }
}

#[derive(Debug)]
pub enum Token {
    Number(u64),
    Op(Operator)
}

pub struct Tokenizer;

impl Tokenizer {

    pub fn new() -> Self {
        Tokenizer {}
    }

    pub fn tokenize(&self, input: &str) -> Option<Vec<Token>> {
        
        if input.is_empty() {
            return None;
        }

        let mut tokens = vec![];

        let split: Vec<&str> = input
            .split(' ')
            .collect();
        
        for token in split {

            if Tokenizer::is_number(token) {
                match Tokenizer::parse_number(token) {
                    Ok(n) => tokens.push(Token::Number(n)),
                    Err(_) => {
                        eprintln!("error parsing number");
                        return None;
                    }
                };
            } else if Tokenizer::is_operand(token) {
                match Operator::from_str(token) {
                    Some(operator) => tokens.push(Token::Op(operator)),
                    None => {
                        eprintln!("[error] parsing operand {} failed", input);
                        return None;
                    },
                };
            } else {
                return None;
            }
        } // for
        
        Some(Tokenizer::prefix_to_postfix(&mut tokens))
    }

    fn prefix_to_postfix(expr: &mut Vec<Token>) -> Vec<Token> {

        let mut postfix = Vec::new();
        let mut stack = Vec::new();

        while !expr.is_empty() {
            
            // SAFETY: runs only if not empty
            let token = expr.pop().unwrap();

            match token {
                Token::Number(n) => stack.push(Token::Number(n)),
                Token::Op(op) => {
                    if let Some(a) = stack.pop() {
                        // turn these around, else reversing will lead 
                        // to - 0xdead 0xbeef == 0xbeef - 0xdead
                        if let Some(b) = stack.pop() {
                            postfix.push(b);
                        }
                        postfix.push(a);
                    }
                    postfix.push(Token::Op(op));
                },
            }

        }

        postfix.reverse(); // reverse because of Vec pop
        postfix
    }

    /// parse a number, it's either hex or a decimal
    fn parse_number(token: &str) -> Result<u64, ParseError>{
        if token.starts_with("0x") {
            if let Some(number) = Tokenizer::parse_hex(token) {
                Ok(number)
            } else {
                Err(ParseError::ParseNumber)
            }
        } else {
            match str::parse::<u64>(token) {
                Ok(number) => Ok(number),
                Err(_) => Err(ParseError::ParseNumber),
            }
        }

    }

    fn parse_hex(token: &str) -> Option<u64> {
        if token.starts_with("0x") {
            let num = token.strip_prefix("0x");
            if let Some(n) = num {
                match u64::from_str_radix(n, 16) {
                    Ok(value) => return Some(value),
                    Err(e) => {
                        eprintln!("[error] invalid hex value: {e}");
                        return None;
                    }
                }
            } else {
                return None;
            }
        }
        None
    }

    fn is_number(input: &str) -> bool {
        if input.starts_with("0x") {
            // SAFETY: prefix exists
            let input = input.strip_prefix("0x").unwrap(); 
            if input.is_empty() { return false; }
            input.chars().all(|c| c.is_ascii_hexdigit())
        } else {
            input.chars().all(|c| c.is_ascii_digit())
        }
    }

    pub fn is_operand(input: &str) -> bool {
        matches!(input, "+" | "-"| "*"| "/" | "&" | "|" | "^" |
                 "<<" | ">>" | "<<<" | ">>>" | "!")
    }

}

#[derive(Copy, Clone)]
pub enum Bits {
    U8,
    U16,
    U32,
    U64,
}

impl Bits {
    pub fn to_num(&self) -> usize {
        match self {
            Bits::U8  => 8,
            Bits::U16 => 16,
            Bits::U32 => 32,
            Bits::U64 => 64,
        }
    }
}

/// bitnum and overflow flag
pub struct Calculation(Bits, bool);

impl Calculation {

    pub fn new(bits: Bits) -> Self {
        Calculation(bits, false)
    }

    pub fn overflow(&self) -> bool {
        self.1
    }

    fn neg(&mut self, a: u64) -> u64 {
        match self.0 {
            Bits::U8  => !a as u64,
            Bits::U16 => !a as u64,
            Bits::U32 => !a as u64,
            Bits::U64 => !a as u64,
        }
    }

    fn add(&mut self, a: u64, b: u64) -> u64 {
        match self.0 {
            Bits::U8  => {
                let c = (a as u8).overflowing_add(b as u8);
                self.1 = c.1;
                c.0 as u64
            },
            Bits::U16  => {
                let c = (a as u16).overflowing_add(b as u16);
                self.1 = c.1;
                c.0 as u64
            },
            Bits::U32  => {
                let c = (a as u32).overflowing_add(b as u32);
                self.1 = c.1;
                c.0 as u64
            },
            Bits::U64  => {
                let c = (a as u64).overflowing_add(b as u64);
                self.1 = c.1;
                c.0
            },
        }
    }

    fn sub(&mut self, a: u64, b: u64) -> u64 {
        match self.0 {
            Bits::U8  => {
                let c = (a as u8).overflowing_sub(b as u8);
                self.1 = c.1;
                c.0 as u64
            },
            Bits::U16  => {
                let c = (a as u16).overflowing_sub(b as u16);
                self.1 = c.1;
                c.0 as u64
            },
            Bits::U32  => {
                let c = (a as u32).overflowing_sub(b as u32);
                self.1 = c.1;
                c.0 as u64
            },
            Bits::U64  => {
                let c = (a as u64).overflowing_sub(b as u64);
                self.1 = c.1;
                c.0
            },
        }
    }

    fn mul(&mut self, a: u64, b: u64) -> u64 {
        match self.0 {
            Bits::U8  => {
                let c = (a as u8).overflowing_mul(b as u8);
                self.1 = c.1;
                c.0 as u64
            },
            Bits::U16  => {
                let c = (a as u16).overflowing_mul(b as u16);
                self.1 = c.1;
                c.0 as u64
            },
            Bits::U32  => {
                let c = (a as u32).overflowing_mul(b as u32);
                self.1 = c.1;
                c.0 as u64
            },
            Bits::U64  => {
                let c = (a as u64).overflowing_mul(b as u64);
                self.1 = c.1;
                c.0
            },
        }
    }

    fn div(&mut self, a: u64, b: u64) -> u64 {
        match self.0 {
            Bits::U8  => {
                let c = (a as u8).overflowing_div(b as u8);
                self.1 = c.1;
                c.0 as u64
            },
            Bits::U16  => {
                let c = (a as u16).overflowing_div(b as u16);
                self.1 = c.1;
                c.0 as u64
            },
            Bits::U32  => {
                let c = (a as u32).overflowing_div(b as u32);
                self.1 = c.1;
                c.0 as u64
            },
            Bits::U64  => {
                let c = (a as u64).overflowing_div(b as u64);
                self.1 = c.1;
                c.0
            },
        }
    }

    fn and(&mut self, a: u64, b: u64) -> u64 {
        match self.0 {
            Bits::U8  => {
                let c = (a as u8) & (b as u8);
                c as u64
            },
            Bits::U16  => {
                let c = (a as u16) & (b as u16);
                c as u64
            },
            Bits::U32  => {
                let c = (a as u32) & (b as u32);
                c as u64
            },
            Bits::U64  => {
                let c = (a as u64) & (b as u64);
                c
            },
        }
    }

    fn or(&mut self, a: u64, b: u64) -> u64 {
        match self.0 {
            Bits::U8  => {
                let c = (a as u8) | (b as u8);
                c as u64
            },
            Bits::U16  => {
                let c = (a as u16) | (b as u16);
                c as u64
            },
            Bits::U32  => {
                let c = (a as u32) | (b as u32);
                c as u64
            },
            Bits::U64  => {
                let c = (a as u64) | (b as u64);
                c
            },
        }
    }

    fn xor(&mut self, a: u64, b: u64) -> u64 {
        match self.0 {
            Bits::U8  => {
                let c = (a as u8) ^ (b as u8);
                c as u64
            },
            Bits::U16  => {
                let c = (a as u16) ^ (b as u16);
                c as u64
            },
            Bits::U32  => {
                let c = (a as u32) ^ (b as u32);
                c as u64
            },
            Bits::U64  => {
                let c = (a as u64) ^ (b as u64);
                c
            },
        }
    }

    fn shl(&mut self, a: u64, b: u64) -> u64 {
        match self.0 {
            Bits::U8  => {
                let c = (a as u8) << (b as u8);
                c as u64
            },
            Bits::U16  => {
                let c = (a as u16) << (b as u16);
                c as u64
            },
            Bits::U32  => {
                let c = (a as u32) << (b as u32);
                c as u64
            },
            Bits::U64  => {
                let c = (a as u64) << (b as u64);
                c
            },
        }
    }

    fn shr(&mut self, a: u64, b: u64) -> u64 {
        match self.0 {
            Bits::U8  => {
                let c = (a as u8) >> (b as u8);
                c as u64
            },
            Bits::U16  => {
                let c = (a as u16) >> (b as u16);
                c as u64
            },
            Bits::U32  => {
                let c = (a as u32) >> (b as u32);
                c as u64
            },
            Bits::U64  => {
                let c = (a as u64) >> (b as u64);
                c
            },
        }
    }

    fn rol(&mut self, a: u64, b: u64) -> u64 {
        match self.0 {
            Bits::U8  => {
                let c = (a as u8).rotate_left(b as u32);
                c as u64
            },
            Bits::U16  => {
                let c =(a as u16).rotate_left(b as u32);
                c as u64
            },
            Bits::U32  => {
                let c =(a as u32).rotate_left(b as u32);
                c as u64
            },
            Bits::U64  => {
                let c =(a as u64).rotate_left(b as u32);
                c
            },
        }
    }

    fn ror(&mut self, a: u64, b: u64) -> u64 {
        match self.0 {
            Bits::U8  => {
                let c = (a as u8).rotate_right(b as u32);
                c as u64
            },
            Bits::U16  => {
                let c =(a as u16).rotate_right(b as u32);
                c as u64
            },
            Bits::U32  => {
                let c =(a as u32).rotate_right(b as u32);
                c as u64
            },
            Bits::U64  => {
                let c =(a as u64).rotate_right(b as u32);
                c
            },
        }
    }

    pub fn calculate(&mut self, tokens: &mut Vec<Token>) -> Option<CalculationResult> {

        let mut stack = Vec::new();

        while !tokens.is_empty() {
            // can't be empty, tested above
            let current = tokens.pop().unwrap();

            match current {
                Token::Number(n) => stack.push(n),
                Token::Op(op) => {

                    if op.is_unary() {
                        // if it's an unary operation, the single 
                        // parameter is on the stack 
                        let a = if let Some(val) = stack.pop() {
                            val
                        } else {
                            println!("[error] no operand on stack");
                            return None;
                        };

                        // we get can now perform the calculation
                        let res = match op {
                            Operator::Neg => self.neg(a),
                            _ => unimplemented!(),
                        };
                        stack.push(res);

                    } else {
                        // if it's a binary operation, the two parameters
                        // must be on the stack
                        let a = if let Some(val) = stack.pop() {
                            val
                        } else {
                            println!("[error] no operand on stack");
                            return None;
                        };

                        let b = if let Some(val) = stack.pop() {
                            val
                        } else {
                            println!("[error] no operand on stack");
                            return None;
                        };

                        // choose the calculation here.
                        let res = match op {
                            Operator::Add => self.add(a, b),
                            Operator::Sub => self.sub(a, b),
                            Operator::Mul => self.mul(a, b),
                            Operator::Div => self.div(a, b),
                            Operator::And => self.and(a, b),
                            Operator::Or  => self.or(a, b),
                            Operator::Xor => self.xor(a, b),
                            Operator::Shl => self.shl(a, b),
                            Operator::Shr => self.shr(a, b),
                            Operator::Rol => self.rol(a, b),
                            Operator::Ror => self.ror(a, b),
                            _ => unimplemented!(),
                        };

                        stack.push(res);
                    } // else
                }
            }
        }
        
        match stack.pop() {
            Some(value) => Some(CalculationResult(value, self.0, self.1)),
            None => None
        }
    }

}

pub struct CalculationResult(u64, Bits, bool);

impl CalculationResult {
    fn to_binary_string(&self) -> String {
        let mut s = String::new();
        for i in (0..self.1.to_num()).rev() {
            s.push(((self.0 >> i & 0x1) + 0x30) as u8 as char);
        }
        s
    }
    pub fn inner(&self) -> u64 {
        self.0
    }
}

impl std::fmt::Display for CalculationResult {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\n[evaluation] ({} bit calculation)", self.1.to_num())?;
        if self.2 {
            writeln!(f,"    [overflow occured]")?;
        }
        match self.1 {
            Bits::U8  => write!(f, "[hex] {:02x}\n", self.0)?,
            Bits::U16 => write!(f, "[hex] {:04x}\n", self.0)?,
            Bits::U32 => write!(f, "[hex] {:08x}\n", self.0)?,
            Bits::U64 => write!(f, "[hex] {:016x}\n", self.0)?,
        }
        write!(f, "[dec] {}\n", self.0)?;
        write!(f, "[bin] {}\n", self.to_binary_string())?;
        write!(f, "[reg]\n{}\n", regprint(self.0, self.1.to_num()))?;
        write!(f, "\n")
    }
}

/// print a `value` as if it were a value in an `iter_max`-bit register.
fn regprint(value: u64, iter_max: usize) -> String {

    let mut s = String::new();

    if iter_max == 64 {
        for i in (32..iter_max).rev() {
            s.push_str(&format!("{i} "));
        }
        s.push_str(&format!("\n"));
        // print value
        for i in (32..iter_max).rev() {
            let bit = (value >> i) & 0x01;
            if i >= 10 {
                s.push_str(&format!("{bit}  "));
            } else {
                s.push_str(&format!("{bit} "));
            }
        }
        s.push_str(&format!("\n"));
        for i in (0..32).rev() {
            s.push_str(&format!("{i} "));
        }
        s.push_str(&format!("\n"));
        // print value
        for i in (0..32).rev() {
            let bit = (value >> i) & 0x01;
            if i >= 10 {
                s.push_str(&format!("{bit}  "));
            } else {
                s.push_str(&format!("{bit} "));
            }
        }
    } else {
        for i in (0..iter_max).rev() {
            s.push_str(&format!("{i} "));
        }
        s.push_str(&format!("\n"));
        // print value
        for i in (0..iter_max).rev() {
            let bit = (value >> i) & 0x01;
            if i >= 10 {
                s.push_str(&format!("{bit}  "));
            } else {
                s.push_str(&format!("{bit} "));
            }
        }
    }
    s.push_str(&format!("\n"));
    s
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        assert!(Tokenizer::parse_number("0xdeadbeef").is_ok());
        assert!(Tokenizer::parse_number("1234").is_ok());
        assert!(Tokenizer::parse_number("abcd").is_err());
        assert!(Tokenizer::parse_number("0xgggg").is_err());
        assert!(Tokenizer::parse_number("1a30").is_err());
    }

    #[test]
    fn test_is_number() {
        assert!(Tokenizer::is_number("0xdeadbeef"));
        assert!(Tokenizer::is_number("1234"));
        assert!(!Tokenizer::is_number("1ddddddddd"));
        assert!(!Tokenizer::is_number("0x"));
    }

    #[test]
    fn test_operator() {
        assert!(Operator::from_str("+").is_some());
        assert!(Operator::from_str("!").is_some());
        assert!(Operator::from_str("x").is_none());
    }

    #[test]
    fn test_tokenize() {
        let t = Tokenizer::new();
        let tokens = t.tokenize("+ 2 + 0xdead 0xbeef");
        println!("tokens: {:?}", tokens);
    }

    #[test]
    fn test_prefix_to_postfix() {
        let mut calc = Calculation(Bits::U64, false);
        let t = Tokenizer::new();
        let tokens = t.tokenize("+ 2 + 0xdead 0xbeef");
    
        let res = calc.calculate(&mut tokens.unwrap());
        assert!(res.is_some());

        let mut calc = Calculation(Bits::U64, false);
        let t = Tokenizer::new();
        let tokens = t.tokenize("! + 0xdead 0xbeef");
        let res = calc.calculate(&mut tokens.unwrap());
        assert!(res.is_some());
    }

    #[test]
    fn test_calculations() {
        let mut calc = Calculation(Bits::U8, false);
        let t = Tokenizer::new();
        let tokens = t.tokenize("+ 0xde 1");
        let res = calc.calculate(&mut tokens.unwrap());
        assert!(res.is_some());

        let mut calc = Calculation(Bits::U8, false);
        let t = Tokenizer::new();
        let tokens = t.tokenize("+ 1 0xff");
        let res = calc.calculate(&mut tokens.unwrap());
        assert!(res.is_some());

        let mut calc = Calculation(Bits::U16, false);
        let t = Tokenizer::new();
        let tokens = t.tokenize("+ 0xdead 2");
        let res = calc.calculate(&mut tokens.unwrap());
        assert!(res.is_some());

        let mut calc = Calculation(Bits::U16, false);
        let t = Tokenizer::new();
        let tokens = t.tokenize("+ 1 0xffff");
        let res = calc.calculate(&mut tokens.unwrap());
        assert!(res.is_some());

        let mut calc = Calculation(Bits::U32, false);
        let t = Tokenizer::new();
        let tokens = t.tokenize("+ 0xdeadca11 1");
        let res = calc.calculate(&mut tokens.unwrap());
        assert!(res.is_some());

        let mut calc = Calculation(Bits::U32, false);
        let t = Tokenizer::new();
        let tokens = t.tokenize("+ 1 0xffffffff");
        let res = calc.calculate(&mut tokens.unwrap());
        assert!(res.is_some());

        let mut calc = Calculation(Bits::U64, false);
        let t = Tokenizer::new();
        let tokens = t.tokenize("+ 0xdeaddeaddeaddead 1");
        let res = calc.calculate(&mut tokens.unwrap());
        assert!(res.is_some());

        let mut calc = Calculation(Bits::U64, false);
        let t = Tokenizer::new();
        let tokens = t.tokenize("+ 1 0xffffffffffffffff");
        let res = calc.calculate(&mut tokens.unwrap());
        assert!(res.is_some());
    }

}
