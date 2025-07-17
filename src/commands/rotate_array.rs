use crate::cli::Response;
use crate::client::Client;
use crate::commands::SubCommand;
use crate::error::Error;
use clap::Args;

#[derive(Args)]
pub struct RotateArrayCommand {
    /// Rotation amount (negative for left, positive for right)
    #[arg(short, long)]
    pub rotation: i64,
    /// The array to rotate
    pub input: String,
}

impl SubCommand for RotateArrayCommand {
    fn run(&self, _list_mode: bool) -> Result<Response, Error> {
        let client = Client::new();
        match client.rotate_array(&self.input, self.rotation as isize) {
            Ok(output) => Ok(Response::String(output)),
            Err(e) => Err(Error::from(e)),
        }
    }
}