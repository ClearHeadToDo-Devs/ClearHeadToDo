use std::io::{self, Write};

fn main() {
    println!("starting program");

    loop {
        print!("> ");
        io::stdout().flush().expect("failed to flush");
        
        let mut inp = String::new();
        io::stdin().read_line(&mut inp).expect("failed to read line");
        match parse_input(&inp) {
            Ok(op) => {
                println!("success!, {}", op);
            },
            Err(err) => {
                println!("error: {}", err);
            }
        } //end 'match parse_input()'
        
    } //end 'loop'
} //end main

pub fn parse_input(inp: &str) -> Result<String, String> {
    match inp.to_ascii_lowercase().trim() {
        "foo" => Ok("test_foo".to_string()),
        //"" => Ok("");
        _ => Err(format!("invalid input")),
    }
}
