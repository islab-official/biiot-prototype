#[derive(Eq, PartialEq)]
pub enum RunError {
    NoError, InvalidJump, ExecutionReverted,
    InvalidOpCode, StackUnderflow, StackOverflow,
    WriteProtection,
    ReturnDataOutOfBounds
}