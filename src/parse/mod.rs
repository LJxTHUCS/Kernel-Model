mod ast;
mod lexer;
mod parser;

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
