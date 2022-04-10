use crate::{label::Labels, procedure::Procedures, Pointer};

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

pub fn parse_instructions(
    line_slice: &[Vec<&str>],
    labels: &Labels,
    procedures: &Procedures,
) -> Instructions {
    todo!()
}
