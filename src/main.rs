use tracing::{error, info};
use vim_alchemy::handler::EventHandler;
use vim_alchemy::logging::setup_tracing;

extern crate neovim_lib;

fn main() {
    match setup_tracing() {
        Ok(_) => info!("Tracing setup complete"),
        Err(e) => error!("Error setting up tracing: {}", e),
    }
    info!("Starting up");
    let mut event_handler = EventHandler::new();
    info!("Event handler created");
    event_handler.recv();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}


