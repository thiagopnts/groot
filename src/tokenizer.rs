use operations::{Op, ToOp};

pub struct Tokenizer<'a> {
    words: Vec<&'a str>,
    misses: u64,
    buf: String,
}

impl<'a> Tokenizer<'a> {
    pub fn new(words: Vec<&'a str>) -> Tokenizer<'a> {
        Tokenizer { words: words, misses: 0, buf: "".to_string() }
    }

    pub fn tokenize(&mut self) -> Vec<Op> {
        let mut ops = vec![];
        for word in self.words.clone().iter() {
            self.buf.push_str(word);
            match self.buf.clone().to_op() {
                Op::Increment => self.push_op(&mut ops, Op::Increment),
                Op::Decrement => self.push_op(&mut ops, Op::Decrement),
                Op::Output    => self.push_op(&mut ops, Op::Output),
                Op::Right     => self.push_op(&mut ops, Op::Right),
                Op::Left      => self.push_op(&mut ops, Op::Left),
                Op::Jump      => self.push_op(&mut ops, Op::Jump),
                Op::JumpBack  => self.push_op(&mut ops, Op::JumpBack),
                Op::Input  => self.push_op(&mut ops, Op::Input),
                Op::Unknown   => {
                    self.misses += 1;
                    if self.misses == 3 {
                        panic!("syntax error unknown token `{}`", self.buf);
                    }
                },
            }
        }
        ops
    }

    fn push_op(&mut self, ops: &mut Vec<Op>, op: Op) {
        ops.push(op);
        self.misses = 0;
        self.buf = "".to_string();
    }
}

