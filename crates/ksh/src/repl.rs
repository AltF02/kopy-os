pub trait Repl {
    type ReplResult;
    type ReplError;

    fn read() -> Self;
    fn evaluate(&self) -> Result<Self::ReplResult, Self::ReplError>;
    fn print(output: Result<Self::ReplResult, Self::ReplError>);
    fn loop_interactive();
}
