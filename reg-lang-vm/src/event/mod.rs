pub enum VMEvent {
    IllegalOpCode,
    OutOfBound,
    HaltEncountered,
    InvalidRegister
}