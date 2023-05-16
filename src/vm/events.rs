pub enum VMEvent {
    IllegalOpCode,
    OutOfBound,
    StackOverflow,
    StackUnderflow,
    DivideByZero,
    ExitSyscall,
    AllGood,
}
