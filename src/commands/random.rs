use crate::types::CliResult;
use crate::client::Client;
use crate::commands::SubCommand;
use clap::Args;

#[derive(Args)]
pub struct Random {
    /// Encoding type
    #[arg(short, long)]
    pub encoding: String,
    /// Number of bytes
    #[arg(short, long)]
    pub bytes: u64,
}

impl SubCommand for Random {
    fn run(&self, _list_mode: bool, _input: Option<&str>) -> CliResult {
        let client = Client::new();
        client.random(&self.encoding, self.bytes as usize).into()
    }
}
