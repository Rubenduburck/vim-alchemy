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
}

impl SubCommand for RotateCommand {
    fn run(&self, _list_mode: bool, input: Option<&str>) -> CliResult {
        match input {
            Some(input) => {
                let client = Client::new();
                client.rotate_array(input, self.rotation as isize).into()
            }
            None => Error::MissingArgs("No input provided".to_string()).into()
        }
    }
}