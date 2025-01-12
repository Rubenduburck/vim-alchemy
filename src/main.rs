use neovim_lib::{Neovim, Session};
use vim_alchemy::handler::Handler;
use vim_alchemy::logging::setup_tracing;

extern crate neovim_lib;

fn run() {
    tracing::debug!("Starting Neovim session");
    let mut nvim = match Session::new_parent() {
        Ok(session) => Neovim::new(session),
        Err(e) => {
            tracing::error!("Failed to create session: {}", e);
            return;
        }
    };
    tracing::debug!("Starting event loop handler");
    let handler = Handler::new();
    nvim.session.start_event_loop_handler(handler);
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    //tracing::info!("Exited event loop");
}

fn main() {
    match setup_tracing() {
        Ok(_) => tracing::info!("Tracing setup complete"),
        Err(e) => tracing::error!("Error setting up tracing: {}", e),
    }
    tracing::info!("Starting Alchemy");
    tracing::debug!("Tracing is active");
    run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
