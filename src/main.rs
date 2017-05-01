#[allow(dead_code)]

mod library;

use std::io;
use std::io::Write;

use library::lexer;
use library::parser;

fn main() {


    let help_doc = "crust-repl is interactive transpiler tool, powered by crust.
	USAGE :
		clear | :c > Clears console
		help  | :h > display help message
		exit  | :e > close crust-repl
		";
    let mut input: String = " ".to_string();
    println!("Enter help | :h for help");
    loop {
        print!(">");
        input.clear();
        io::stdout().flush().ok().expect("FATAL : Buffer flush failed");
        io::stdin().read_line(&mut input).expect("Sorry...I couldn't read that");
        let mut input: String = input.trim().to_string();
        match input.as_ref() {
            "clear" | ":c" => {
                println!("Clear...");
            	continue;
            }
            "exit" | ":e" => {
                println!("Closing crust-repl");
                break;

            }
            "help" | ":h" => {
                println!("{}", help_doc);
            	continue;
	    }
            _ => {}
        }
        input += ";";
        let mut tok = lexer::Tokenizer::new(&input);

        let tokens = tok.tokenize();
        let strict = false;
        let rust_lexeme = parser::init_parser(&tokens, strict).unwrap();

        //regenerate the code from lexemes
        let mut o: String = String::new();
        for i in rust_lexeme {
            o = o + " ";
            o = o + &i[..];
        }
        println!(">>> {} ", o);
    }
}
