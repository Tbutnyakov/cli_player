use std::process::exit;
use cli_player::{run_app};
use colored::Colorize;

fn main()-> Result<(), Box<dyn std::error::Error>> {

    ctrlc::set_handler(|| {
        println!("\n{}: Exiting...", "Info".blue());
        exit(0);
    })?;


    let _ = run_app();

    Ok(())
}
