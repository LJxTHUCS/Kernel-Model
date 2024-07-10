mod ast;
mod lexer;
mod parser;

use crate::error::Error;
use lexer::{tokenize_kml, Tokens};
use parser::parse_kml;

pub use ast::*;

pub fn lex_and_parse_kml(kml: &str) -> Result<Model, Error> {
    let tokens = tokenize_kml(kml)?;
    let tokens = Tokens::from(tokens.as_ref());
    let model = parse_kml(tokens)?;
    // Verify that the model is valid
    verify(&model)?;
    Ok(model)
}

fn verify(model: &Model) -> Result<(), Error> {
    // Events config and scheduler config must be present and only present once
    let mut events_config = None;
    let mut scheduler_config = None;
    for config in &model.kernel_def.configs {
        match config {
            KernelConfig::Events(event) => {
                if events_config.is_some() {
                    return Err(Error::BadKernelConfig);
                }
                events_config = Some(event);
            }
            KernelConfig::Scheduler(scheduler) => {
                if scheduler_config.is_some() {
                    return Err(Error::BadKernelConfig);
                }
                scheduler_config = Some(scheduler);
            }
        }
    }
    if events_config.is_none() || scheduler_config.is_none() {
        return Err(Error::BadKernelConfig);
    }
    // Check if events are defined
    for event in events_config.unwrap() {
        if model.event_defs.iter().find(|e| e.name == *event).is_none() {
            return Err(Error::BadKernelConfig);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::lexer::*;
    use super::parser::*;
    use std::fs::OpenOptions;

    #[test]
    fn parse_test() {
        let kml_file = OpenOptions::new()
            .read(true)
            .open("src/parse/demo.kml")
            .unwrap();
        let kml = std::io::read_to_string(kml_file).unwrap();
        let tokens = tokenize_kml(&kml).unwrap();
        // println!("{:#?}", tokens);
        let tokens = Tokens::from(tokens.as_ref());
        let result = parse_kml(tokens);
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}
