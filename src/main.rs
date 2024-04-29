use std::process::exit;
use clap::Parser;
use env_logger::Env;
use log::error;
use kafka::run;
use kafka::Arguments;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let arguments = Arguments::parse();
    match run(arguments).await {
        Ok(()) => (),
        Err(err) =>  {
            error!("Exiting with error: {err}");
            exit(1);
        }
    }
}
