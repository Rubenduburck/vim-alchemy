use crate::types::CliResult;
use crate::client::Client;
use crate::commands::SubCommand;
use crate::error::Error;
use clap::Args;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Side {
    #[clap(name = "left")]
    Left,
    #[clap(name = "right")]
    Right,
}

#[derive(Args)]
pub struct Pad {
    /// Padding size in bytes
    #[arg(short, long)]
    pub padding: u64,
    /// Side
    #[arg(short, long, default_value = "left", value_enum)]
    pub side: Side,
}

impl SubCommand for Pad {
    fn run(&self, _list_mode: bool, input: Option<&str>) -> CliResult {
        let input = match input {
            Some(i) => i,
            None => return Error::MissingArgs("input".to_string()).into(),
        };
        let client = Client::new();
        match self.side {
            Side::Left => client.pad_left(self.padding as usize, input),
            Side::Right => client.pad_right(self.padding as usize, input),
        }.into()
    }
}
