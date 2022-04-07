/// instruction list
///
/// | instruction   | Description |
/// | ----------- | ----------- |
/// | Push (value) | Push the value into the top of stack |
/// | Pop | pop the value from the top of stack |
/// | Add | pop two values, and push their sum |
/// | Sub | pop two values, and push their differences |
/// | Mul | pop two values, and push their multiply |
/// | Div | pop two values, and push their division |
/// | Incr | the value at the top of the stack increment by one |
/// | Decr | the value at the top of the stack decrement by one |
/// | Get  |
pub enum Instruction {
    Push(isize),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Incr,
    Decr,
    // TODO: instruction list
}
