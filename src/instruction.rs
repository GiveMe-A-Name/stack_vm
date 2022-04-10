use crate::{
    label::{self, Labels},
    procedure::Procedures,
    Pointer,
};

/// instruction list
///
/// | instruction   | Description |
/// | ----------- | ----------- |
/// | Push (isize) | Push the value into the top of stack |
/// | Pop | pop the value from the top of stack |
/// | Add | pop two values, and push their sum |
/// | Sub | pop two values, and push their differences |
/// | Mul | pop two values, and push their multiply |
/// | Div | pop two values, and push their division |
/// | Incr | the value at the top of the stack increment by one |
/// | Decr | the value at the top of the stack decrement by one |
/// | Get (usize) | get index of the statck and copy it to the top |
/// | Set (usize) | copy value at the top of stack to the index |
/// | GetArg (usize) | gets nth argument from top of callstack stack offset, used for procedures |
/// | SetArg (usize) | sets nth argument from top fo callstack stack offset, used for precedures |
/// | Noop | Do nothing |
/// | Print | print the value at the top of the stack as an integer |
/// | PrintC | pint the value at the top of the stack as a char |
/// | Jump (label) | set the pc to the label |
/// | JNE (label) | jump if the top of the stack is not zero |
/// | JE (label) | jump if the top of the stack is zero |
/// | JGT (label) | Jumps if the top of the stack is greater than zero |
/// | JLT (label) | Jumps if the top of the stack is less than zero |
/// | JGE (label) | Jumps if the top of the stack is greater than or equal to zero |
/// | JLE (label) | Jumps if the top of the stack is less than or equal to zero |
/// | Call (procedure) | Calls a procedure |
/// | Ret | return the procedure's value |
/// | PrintStack | Prints the whole stack, used mostly for debugging |
pub enum Instruction {
    Push(isize),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Incr,
    Decr,
    Get(Pointer),
    Set(Pointer),
    GetArg(Pointer),
    SetArg(Pointer),
    Noop,
    Print,
    PrintC,
    Jump(Pointer),
    JNE(Pointer),
    JE(Pointer),
    JGT(Pointer),
    JLT(Pointer),
    JLE(Pointer),
    Call(Pointer),
    Ret,
    PrintStack,
}

pub type Instructions = Vec<Instruction>;

pub fn parse_instruction(line: &[&str], labels: &Labels, procedures: &Procedures) -> Instruction {
    use Instruction::*;

    match line {
        ["Push", x] => Push(x.parse().unwrap()),
        ["Pop"] => Pop,
        ["Add"] => Add,
        ["Sub"] => Sub,
        ["Mul"] => Mul,
        ["Div"] => Div,
        ["Incr"] => Incr,
        ["Decr"] => Decr,
        ["Get", x] => Get(x.parse().unwrap()),
        ["Set", x] => Set(x.parse().unwrap()),
        ["GetArg", x] => GetArg(x.parse().unwrap()),
        ["SetArg", x] => SetArg(x.parse().unwrap()),
        ["Noop"] => Noop,
        ["Print"] => Print,
        ["PinrtC"] => PrintC,
        ["PrintStack"] => PrintStack,
        ["Ret"] => Ret,
        ["Call", proce_name] => {
            let range = procedures.get(proce_name).unwrap();
            Call(range.0)
        }
        ["Jump", label] => Jump(*labels.get(label).unwrap()),
        ["JNE", label] => JNE(*labels.get(label).unwrap()),
        ["JE", label] => JE(*labels.get(label).unwrap()),
        ["JGE", label] => JGT(*labels.get(label).unwrap()),
        ["JLT", label] => JLT(*labels.get(label).unwrap()),
        ["JLE", label] => JLE(*labels.get(label).unwrap()),
        _ => panic!("err instrucitons"),
    }
}
