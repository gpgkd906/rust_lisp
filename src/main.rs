mod environment;
mod parser;
mod evaluator;
mod exception;
mod expression;
mod operator;

use crate::environment::Environment;
use crate::parser::Parser;
use crate::evaluator::Evaluator;
use std::io::{self, Write};

struct Lisp;

impl Lisp {
    fn initialize() -> Environment {
        Environment::initialize()
    }

    fn repl(env: &mut Environment) {
        let mut input_accumulated = String::new();
        let mut open_parens = 0;

        loop {
            let prompt = if open_parens > 0 { "> " } else { "lisp:> " };
            match Lisp::readline(prompt) {
                Ok(input) => {
                    if input.trim() == "exit" {
                        break;
                    }
                    input_accumulated.push_str(&input);
                    open_parens += input.chars().filter(|&ch| ch == '(').count();
                    open_parens -= input.chars().filter(|&ch| ch == ')').count();

                    if open_parens == 0 {
                        match Parser::read(&input_accumulated) {
                            Ok(ast) => {
                                let result = Evaluator::eval(&ast, env);
                                match result {
                                    Ok(value) => Lisp::write(format!("{}\n", value)),
                                    Err(err) => eprintln!("Error: {}", err),
                                }
                            }
                            Err(err) => eprintln!("Parse Error: {}", err),
                        }
                        input_accumulated.clear();
                    } else if open_parens < 0 {
                        eprintln!("Parse Error: Mismatched parentheses");
                        input_accumulated.clear();
                        open_parens = 0;
                    }
                }
                Err(err) => eprintln!("Readline Error: {}", err),
            }
        }
    }

    #[allow(dead_code)]
    fn interpreter(file: &str, env: &mut Environment) {
        match std::fs::read_to_string(file) {
            Ok(contents) => {
                let mut input_accumulated = String::new();
                let mut open_parens = 0;

                for line in contents.lines() {
                    input_accumulated.push_str(line);
                    open_parens += line.chars().filter(|&ch| ch == '(').count();
                    open_parens -= line.chars().filter(|&ch| ch == ')').count();

                    if open_parens == 0 {
                        match Parser::read(&input_accumulated) {
                            Ok(ast) => {
                                let result = Evaluator::eval(&ast, env);
                                match result {
                                    Ok(value) => Lisp::write(format!("{}\n", value)),
                                    Err(err) => eprintln!("Error: {}", err),
                                }
                            }
                            Err(err) => eprintln!("Parse Error: {}", err),
                        }
                        input_accumulated.clear();
                    } else if open_parens < 0 {
                        eprintln!("Parse Error: Mismatched parentheses");
                        input_accumulated.clear();
                        open_parens = 0;
                    }
                }
            }
            Err(err) => eprintln!("File Error: {}", err),
        }
    }

    fn readline(prompt: &str) -> Result<String, io::Error> {
        print!("{}", prompt);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input)
    }

    fn write(output: String) {
        print!("{}", output);
    }
}

fn main() {
    let mut env = Lisp::initialize();
    Lisp::repl(&mut env);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_environment() {
        let env = Lisp::initialize();
        assert!(env.get_symbol("T").is_some());
        assert_eq!(env.get_symbol("NIL"), Some(&crate::expression::Expr::List(vec![])));
    }
}
