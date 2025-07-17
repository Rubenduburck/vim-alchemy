use crate::types::CliResult;
use crate::client::Client;
use crate::commands::SubCommand;
use crate::error::Error;
use clap::Args;

#[derive(Args)]
pub struct RotateCommand {
    /// Rotation amount (negative for left, positive for right)
    #[arg(short, long)]
    pub rotation: i64,
    /// Input data to rotate
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    input: Vec<String>,
}

impl SubCommand for RotateCommand {
    fn run(&self, _list_mode: bool) -> CliResult {
        if self.input.is_empty() {
            return Error::MissingArgs("No input provided".to_string()).into();
        }
        let input = self.input.join(" ");
        let client = Client::new();
        client.rotate_array(&input, self.rotation as isize).into()
    }
}