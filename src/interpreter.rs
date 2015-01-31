use operations::Op;

pub struct Interpreter {
    memory: Vec<u8>,
    pointer: i64,
    ops: Vec<Op>,
}

impl Interpreter {
    pub fn new(ops: Vec<Op>) -> Interpreter {
        let m = (0 .. 30000).map(|_| 0).collect();
        Interpreter { memory: m, pointer: 0, ops: ops }
    }

    pub fn run(&mut self) {
        let mut program_counter = 0;
        while program_counter < self.ops.len() {
            match self.ops[program_counter] {
                Op::Increment => self.increment(),
                Op::Decrement => self.decrement(),
                Op::Output    => self.output(),
                Op::Right     => self.right(),
                Op::Left      => self.left(),
                Op::Jump      => self.jump(&mut program_counter),
                Op::JumpBack  => self.jump_back(&mut program_counter),
                _ => panic!("boom"),
            }
            program_counter += 1;
        }
        println!("");
    }

    fn left(&mut self) {
        self.pointer -= 1;
    }

    fn right(&mut self) {
        self.pointer += 1;
    }

    fn increment(&mut self) {
        self.memory[self.pointer as usize] += 1;
    }

    fn decrement(&mut self) {
        self.memory[self.pointer as usize] -= 1;
    }

    fn output(&self) {
        print!("{}", (self.memory[self.pointer as usize]) as char);
    }

    fn jump(&mut self, program_counter: &mut usize) {
        let mut bal = 1i32;
        if self.memory[self.pointer as usize] == 0u8 {
            loop {
                *program_counter += 1;
                if self.ops[*program_counter] == Op::Jump {
                    bal += 1;
                } else if self.ops[*program_counter] == Op::JumpBack {
                    bal -= 1;
                }
                if bal == 0 {
                    break;
                }
            }
        }
    }

    fn jump_back(&mut self, program_counter: &mut usize) {
        let mut bal = 0i32;
        loop {
            if self.ops[*program_counter] == Op::Jump {
                bal += 1;
            } else if self.ops[*program_counter] == Op::JumpBack {
                bal -= 1;
            }
            *program_counter -= 1;
            if bal == 0 {
                break;
            }
        }
    }
}


