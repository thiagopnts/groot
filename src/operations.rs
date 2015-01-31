
pub enum Op {
    Increment,
    Decrement,
    Output,
    Right,
    Left,
    Jump,
    JumpBack,
    Unknown,
}

pub trait ToOp {
    fn to_op(&self) -> Op;
}

impl ToOp for String {
    fn to_op(&self) -> Op {
        match self.as_slice() {
            "iamgroot" => Op::Increment,
            "IamGroot" => Op::Decrement,
            "IAMGROOOT" => Op::Output,
            "IAMGROOT" => Op::Right,
            "Iamgroot" => Op::Left,
            "I'mGroot" => Op::Jump,
            "WeareGroot" => Op::JumpBack,
            _ => Op::Unknown,
        }
    }
}

