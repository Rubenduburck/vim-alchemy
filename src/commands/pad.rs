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
    /// Input data to pad
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    input: Vec<String>,
}

impl SubCommand for Pad {
    fn run(&self, _list_mode: bool) -> CliResult {
        if self.input.is_empty() {
            return Error::MissingArgs("input".to_string()).into();
        }
        let input = self.input.join(" ");
        let client = Client::new();
        match self.side {
            Side::Left => client.pad_left(self.padding as usize, &input),
            Side::Right => client.pad_right(self.padding as usize, &input),
        }.into()
    }
}
