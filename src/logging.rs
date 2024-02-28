use std::{fs::OpenOptions, sync::Arc};
use tracing_subscriber::{filter, prelude::*};

use crate::error::ConvertError;

pub fn setup_tracing() -> Result<(), ConvertError> {
    // get home dir from env
    let home_dir = std::env::var("HOME")?; 
    let log_file_name = format!("{}/.local/share/nvim/alchemy.log", home_dir);
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file_name)?;
    let debug_log = tracing_subscriber::fmt::layer().with_writer(Arc::new(file));

    let metrics_layer = /* ... */ filter::LevelFilter::INFO;

    tracing_subscriber::registry()
        .with(debug_log.with_filter(filter::filter_fn(|metadata| {
            !metadata.target().starts_with("metrics")
        })))
        .with(metrics_layer.with_filter(filter::filter_fn(|metadata| {
            metadata.target().starts_with("metrics")
        })))
        .init();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup_tracing() {
        match setup_tracing() {
            Ok(_) => {}
            Err(e) => {
                println!("Error setting up tracing: {}", e);
                panic!("Error setting up tracing");
            }
        }
    }
}
