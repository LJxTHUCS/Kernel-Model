mod error;
mod event;
mod kernel;
mod parse;
mod scheduler;
mod state;

use clap::Parser;
use error::Error;
use event::*;
use kernel::*;
use parse::*;
use scheduler::*;
use std::{fs::OpenOptions, io::Write};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of the kml file
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();
    let kml_file = OpenOptions::new()
        .read(true)
        .open(args.file)
        .expect("Failed to open file");
    let kml = std::io::read_to_string(kml_file).expect("Failed to read file");

    // parse
    let model = parse::lex_and_parse_kml(&kml).expect("Failed to parse kml");

    // build model
    // def events
    let mut events = Vec::new();
    for def in model.event_defs {
        events.push(Event::new(def));
    }
    // Config kernel
    let mut enabled_events = None;
    let mut scheduler = None;
    for config in &model.kernel_def.configs {
        if let KernelConfig::Events(names) = config {
            enabled_events = Some(
                events
                    .into_iter()
                    .filter(|event| names.contains(&Identifier(event.name().to_owned())))
                    .collect::<Vec<_>>(),
            );
            break;
        }
    }
    for config in &model.kernel_def.configs {
        if let KernelConfig::Scheduler(type_) = config {
            scheduler = Some(Scheduler::new(*type_));
            break;
        }
    }
    // Verification in parser ensures events and scheduler are not empty
    let mut kernel = Kernel::new(enabled_events.unwrap(), scheduler.unwrap());
    println!("Kernel Model Created!");
    kernel.print_config();

    // Run
    while kernel.shutdown_code().is_none() {
        kernel.print_state();
        print!("event>> ");
        std::io::stdout().flush().unwrap();
        let mut event = String::new();
        std::io::stdin().read_line(&mut event).unwrap();
        if let Err(e) = kernel.execute(event.trim()) {
            println!("Error: {:?}", e);
            if e == Error::NoReadyTask {
                kernel.shutdown(0).unwrap();
            }
        }
    }

    println!(
        "Kernel shutdown with code: {}",
        kernel.shutdown_code().unwrap()
    );
}
