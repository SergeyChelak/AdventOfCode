pub type Register = usize;
pub type Value = i64;
pub enum OpValue {
    Val(Value),
    Reg(Register),
}

pub enum Op {
    Snd(OpValue),
    Set(Register, OpValue),
    Add(Register, OpValue),
    Sub(Register, OpValue),
    Mul(Register, OpValue),
    Mod(Register, OpValue),
    Rcv(Register),
    Jgz(OpValue, OpValue),
    Jnz(OpValue, OpValue),
}

impl Op {
    pub fn from_str(s: &str) -> Self {
        let tokens = s.split(' ').collect::<Vec<&str>>();
        match tokens[0] {
            "snd" => Self::Snd(Self::parse_op_value(tokens[1])),
            "set" => Self::Set(Self::parse_reg(tokens[1]), Self::parse_op_value(tokens[2])),
            "add" => Self::Add(Self::parse_reg(tokens[1]), Self::parse_op_value(tokens[2])),
            "sub" => Self::Sub(Self::parse_reg(tokens[1]), Self::parse_op_value(tokens[2])),
            "mul" => Self::Mul(Self::parse_reg(tokens[1]), Self::parse_op_value(tokens[2])),
            "mod" => Self::Mod(Self::parse_reg(tokens[1]), Self::parse_op_value(tokens[2])),
            "rcv" => Self::Rcv(Self::parse_reg(tokens[1])),
            "jgz" => Self::Jgz(
                Self::parse_op_value(tokens[1]),
                Self::parse_op_value(tokens[2]),
            ),
            "jnz" => Self::Jnz(
                Self::parse_op_value(tokens[1]),
                Self::parse_op_value(tokens[2]),
            ),
            _ => panic!("Unexpected command {}", tokens[0]),
        }
    }

    fn parse_op_value(s: &str) -> OpValue {
        if let Ok(val) = s.parse::<Value>() {
            OpValue::Val(val)
        } else {
            OpValue::Reg(Self::parse_reg(s))
        }
    }

    fn parse_reg(s: &str) -> Register {
        let ch = s.parse::<char>().expect("Register name should be char");
        assert!(ch.is_alphabetic(), "{} isn't alphabetic character", ch);
        (ch as u8 - b'a') as usize
    }
}
