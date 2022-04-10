use crate::instruction::Instruction;
use crate::Pointer;

type Program<'a> = &'a [Instruction];

struct Stack(Vec<isize>);

impl Stack {
    fn new() -> Self {
        Stack(vec![])
    }
    fn push(&mut self, value: isize) {
        self.0.push(value);
    }
    fn pop(&mut self) -> isize {
        self.0.pop().expect("pop value from the empty stack")
    }
    fn top(&self) -> &isize {
        self.0
            .last()
            .expect("get the value at the top of the empty stack")
    }
    fn top_mut(&mut self) -> &mut isize {
        self.0
            .last_mut()
            .expect("get the value at the top of the empty stack")
    }
    fn get(&self, pointer: Pointer) -> &isize {
        self.0.get(pointer).expect("get error value")
    }
    fn get_mut(&mut self, pointer: Pointer) -> &mut isize {
        self.0.get_mut(pointer).expect("get error value")
    }
}

pub fn interpret<'a>(program: Program<'a>) {
    use crate::instruction::Instruction::*;
    let mut stack = Stack::new();
    let mut pc = 0;
    while let Some(instruction) = program.get(pc) {
        match *instruction {
            Push(value) => stack.push(value),
            Pop => {
                stack.pop();
            }
            Add => {
                let (first, second) = (stack.pop(), stack.pop());
                stack.push(first + second)
            }
            Sub => {
                let (a, b) = (stack.pop(), stack.pop());
                stack.push(b - a);
            }
            Mul => {
                let (a, b) = (stack.pop(), stack.pop());
                stack.push(a * b);
            }
            Div => {
                let (a, b) = (stack.pop(), stack.pop());
                stack.push(b / a)
            }
            Incr => *stack.top_mut() += 1,
            Decr => *stack.top_mut() -= 1,
            Get(pointer) => {
                let value = *stack.get(pointer);
                stack.push(value);
            }
            Set(pointer) => {
                let top = *stack.top();
                let target = stack.get_mut(pointer);
                *target = top;
            }
            Noop => (),
            Print => print!("{}", *stack.top()),
            PrintC => print!("{}", *stack.top() as u8 as char),
            Jump(pointer) => pc = pointer,
            JNE(pointer) => {
                if *stack.top() != 0 {
                    stack.pop();
                    pc = pointer;
                }
            }
            JE(pointer) => {
                if *stack.top() == 0 {
                    stack.pop();
                    pc = pointer;
                }
            }
            JGT(pointer) => {
                if *stack.top() > 0 {
                    stack.pop();
                    pc = pointer;
                }
            }
            JLT(pointer) => {
                if *stack.top() < 0 {
                    stack.pop();
                    pc = pointer;
                }
            }
            JGE(pointer) => {
                if *stack.top() >= 0 {
                    stack.pop();
                    pc = pointer;
                }
            }
            JLE(pointer) => {
                if *stack.top() <= 0 {
                    stack.pop();
                    pc = pointer;
                }
            }
            Call(_) => todo!(),
            Ret => todo!(),
            PrintStack => todo!(),
            GetArg(_) => todo!(),
            SetArg(_) => todo!(),
        }
        pc += 1;
    }
}
