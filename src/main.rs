// main.rs
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

// 定义一个IO trait用于后续测试时模拟输入和输出操作
trait IO {
    fn readline(&mut self, prompt: &str) -> io::Result<String>;
    fn write(&mut self, output: String) -> io::Result<()>;
}

// 实现标准输入输出
pub struct StdIO;

#[cfg(not(tarpaulin_include))]
impl IO for StdIO {
    fn readline(&mut self, prompt: &str) -> io::Result<String> {
        print!("{}", prompt);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input)
    }

    fn write(&mut self, output: String) -> io::Result<()> {
        print!("{}", output);
        io::stdout().flush()?;
        Ok(())
    }
}

struct Lisp;

impl Lisp {
    fn initialize() -> Environment {
        Environment::initialize()
    }

    fn repl<T: IO>(env: &mut Environment, io: &mut T) {
        let mut input_accumulated = String::new();
        let mut open_parens = 0;

        loop {
            let prompt = if open_parens > 0 { "> " } else { "lisp:> " };
            match io.readline(prompt) {
                Ok(input) => {
                    if input.trim() == "exit" {
                        break;
                    }
                    input_accumulated.push_str(&input);
                    
                    open_parens += input.chars().filter(|&ch| ch == '(').count();
                    open_parens = open_parens.saturating_sub(input.chars().filter(|&ch| ch == ')').count());

                    if open_parens == 0 {
                        match Parser::read(&input_accumulated) {
                            Ok(ast) => {
                                let result = Evaluator::eval(&ast, env);
                                match result {
                                    Ok(value) => io.write(format!("{}\n", value)).unwrap(),
                                    Err(err) => io.write(format!("Error: {}\n", err)).unwrap(),
                                }
                            }
                            Err(err) => io.write(format!("Parse Error: {}\n", err)).unwrap(),
                        }
                        input_accumulated.clear(); // 每次完整表达式处理后清空输入
                    }
                }
                Err(err) => io.write(format!("Readline Error: {}\n", err)).unwrap(),
            }
        }
    }


    #[allow(dead_code)]
    fn interpreter<T: IO>(file: &str, env: &mut Environment, io: &mut T) {
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
                                    Ok(value) => io.write(format!("{}\n", value)).unwrap(),
                                    Err(err) => io.write(format!("Error: {}\n", err)).unwrap(),
                                }
                            }
                            Err(err) => io.write(format!("Parse Error: {}\n", err)).unwrap(),
                        }
                        input_accumulated.clear();
                    }
                }
            }
            Err(err) => eprintln!("File Error: {}", err),
        }
    }    
}

#[cfg(not(tarpaulin_include))]
fn main() {
    let mut env = Lisp::initialize();
    let mut stdio = StdIO;
    Lisp::repl(&mut env, &mut stdio);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exception::LispError;
    use std::io::{self, Write, Cursor, BufReader, BufRead};

    struct MockIO<'a> {
        input: Cursor<&'a [u8]>,
        output: Vec<u8>,
    }
    
    impl<'a> MockIO<'a> {
        fn new(input: &'a [u8]) -> Self {
            MockIO {
                input: Cursor::new(input),
                output: Vec::new(),
            }
        }
    
        fn get_output(&self) -> String {
            String::from_utf8(self.output.clone()).unwrap()
        }
    }
    
    impl<'a> IO for MockIO<'a> {
        fn readline(&mut self, prompt: &str) -> io::Result<String> {
            self.write(prompt.to_string())?;
            let mut line = String::new();
            let bytes_read = BufReader::new(&mut self.input).read_line(&mut line)?;
    
            if bytes_read == 0 {
                return Ok("exit".to_string()); // 在EOF时返回"exit"以退出REPL
            }
    
            Ok(line)
        }
    
        fn write(&mut self, output: String) -> io::Result<()> {
            self.output.write_all(output.as_bytes())?;
            Ok(())
        }
    }

    #[test]
    fn test_initialize_environment() {
        let env = Lisp::initialize();
        assert!(env.get_symbol("T").is_some());
        assert_eq!(env.get_symbol("NIL"), Some(&crate::expression::Expr::List(vec![])));
    }

    #[test]
    fn test_repl_addition() {
        let input = b"(+ 1 2 3)\nexit\n"; // 确保每个表达式后都有换行符，并以exit结束
        let mut env = Lisp::initialize();
        let mut io = MockIO::new(input);

        Lisp::repl(&mut env, &mut io);
        let output = io.get_output();
        assert!(output.contains("6\n")); // 检查输出是否包含期望的结果并以换行符结束
    }

    #[test]
    fn test_repl_subtraction() {
        let input = b"(- 10 4 2)\nexit\n";
        let mut env = Lisp::initialize();
        let mut io = MockIO::new(input);

        Lisp::repl(&mut env, &mut io);
        let output = io.get_output();
        assert!(output.contains("4"));
    }

    #[test]
    fn test_repl_multiplication() {
        let input = b"(* 3 5 2)\nexit\n";
        let mut env = Lisp::initialize();
        let mut io = MockIO::new(input);

        Lisp::repl(&mut env, &mut io);
        let output = io.get_output();
        assert!(output.contains("30"));
    }

    #[test]
    fn test_repl_division() {
        let input = b"(/ 20 4)\nexit\n";
        let mut env = Lisp::initialize();
        let mut io = MockIO::new(input);

        Lisp::repl(&mut env, &mut io);
        let output = io.get_output();
        assert!(output.contains("5"));
    }

    #[test]
    fn test_repl_division_by_zero() {
        let input = b"(/ 20 0)\nexit\n";
        let mut env = Lisp::initialize();
        let mut io = MockIO::new(input);

        Lisp::repl(&mut env, &mut io);
        let output = io.get_output();
        assert!(output.contains("Error"));
    }

    #[test]
    fn test_repl_invalid_expression() {
        let input = b"(+ 1 a)\nexit\n"; // 输入中存在未定义符号a
        let mut env = Lisp::initialize();
        let mut io = MockIO::new(input);

        Lisp::repl(&mut env, &mut io);
        let output = io.get_output();
        assert!(output.contains("Undefined symbol: a"));
    }

    #[test]
    fn test_repl_quotes() {
        let input = b"'(1 2 3)\nexit\n";
        let mut env = Lisp::initialize();
        let mut io = MockIO::new(input);

        Lisp::repl(&mut env, &mut io);
        let output = io.get_output();
        assert!(output.contains("(1 2 3)"));
    }

    #[test]
    fn test_repl_conditional() {
        let input = b"(cond ((> 3 2) 1) (t 0))\nexit\n";
        let mut env = Lisp::initialize();
        let mut io = MockIO::new(input);

        Lisp::repl(&mut env, &mut io);
        let output = io.get_output();
        assert!(output.contains("1"));
    }

    #[test]
    fn test_invalid_function_call() {
        let input = b"(invalid-func 1 2 3)\nexit\n"; // 调用未定义的函数
        let mut env = Lisp::initialize();
        let mut io = MockIO::new(input);
    
        Lisp::repl(&mut env, &mut io);
        let output = io.get_output();
        assert!(output.contains("Undefined function: invalid-func"));
    }

    #[test]
    fn test_file_interpreter_valid_file() {
        // 测试解释器从文件读取并执行内容
        let file_content = "(+ 1 2 3)\n(+ 4 5 6)";
        let file_path = "test_file.lisp";
        std::fs::write(file_path, file_content).unwrap();
    
        let mut env = Lisp::initialize();
        let mut io = MockIO::new(b"");
    
        Lisp::interpreter(file_path, &mut env, &mut io);
        let output = io.get_output();
        assert!(output.contains("6\n"));
        assert!(output.contains("15\n"));
    
        std::fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_file_interpreter_invalid_file() {
        // 测试解释器处理文件读取错误
        let invalid_file_path = "non_existent_file.lisp";
    
        let mut env = Lisp::initialize();
        let mut io = MockIO::new(b"");
    
        Lisp::interpreter(invalid_file_path, &mut env, &mut io);
        let output = io.get_output();
        assert!(output.is_empty()); // 应该没有输出，因为文件读取失败
    }

    #[test]
    fn test_empty_input() {
        let input = b"\nexit\n"; // 输入空行
        let mut env = Lisp::initialize();
        let mut io = MockIO::new(input);
    
        Lisp::repl(&mut env, &mut io);
        let output = io.get_output();
        println!("output: {}", output); // 调试输出以查看实际输出内容
        assert!(!output.contains("Parse Error")); // 确保没有解析错误
    }
    
    #[test]
    fn test_whitespace_input() {
        let input = b"   \nexit\n"; // 输入仅包含空格
        let mut env = Lisp::initialize();
        let mut io = MockIO::new(input);
    
        Lisp::repl(&mut env, &mut io);
        let output = io.get_output();
        println!("output: {}", output); // 调试输出以查看实际输出内容
        assert!(!output.contains("Parse Error")); // 确保没有解析错误
    }
    
    #[test]
    fn test_large_expression() {
        let input = b"(+ 1 2 3 4 5 6 7 8 9 10)\nexit\n"; // 大型表达式
        let mut env = Lisp::initialize();
        let mut io = MockIO::new(input);
    
        Lisp::repl(&mut env, &mut io);
        let output = io.get_output();
        assert!(output.contains("55")); // 检查是否正确计算
    }

    #[test]
    fn test_nested_expressions() {
        let input = b"(+ (* 2 3) (- 5 3))\nexit\n"; // 嵌套表达式
        let mut env = Lisp::initialize();
        let mut io = MockIO::new(input);
    
        Lisp::repl(&mut env, &mut io);
        let output = io.get_output();
        assert!(output.contains("8")); // 检查是否正确计算
    }

    #[test]
    fn test_multiple_expressions() {
        let input = b"(+ 1 2 3)\n(* 2 3)\nexit\n"; // 输入多个表达式
        let mut env = Lisp::initialize();
        let mut io = MockIO::new(input);
    
        Lisp::repl(&mut env, &mut io);
        let output = io.get_output();
        println!("output: {}", output); // 调试输出以查看实际输出内容
        assert!(output.contains("6\n")); // 检查第一个表达式的结果
        assert!(output.contains("6\n")); // 检查第二个表达式的结果
    }    
    
    #[test]
    fn test_interpreter_with_comments() {
        // 测试解释器处理带有注释的代码
        let file_content = "; This is a comment\n(+ 1 2 3)\n; Another comment\n(+ 4 5 6)";
        let file_path = "test_file_with_comments.lisp";
        std::fs::write(file_path, file_content).unwrap();
    
        let mut env = Lisp::initialize();
        let mut io = MockIO::new(b"");
    
        Lisp::interpreter(file_path, &mut env, &mut io);
        let output = io.get_output();
        assert!(output.contains("6\n"));
        assert!(output.contains("15\n"));
    
        std::fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_interpreter_with_unexpected_input() {
        // 准备包含意外输入的测试文件
        let file_content = "(+ 1 2 3) unexpected";
        let file_path = "test_file_with_unexpected_input.lisp";
        std::fs::write(file_path, file_content).unwrap();
    
        let mut env = Lisp::initialize();
        let mut io = MockIO::new(b"");
    
        Lisp::interpreter(file_path, &mut env, &mut io);
        let output = io.get_output();
        assert!(output.contains("Parse Error: Unexpected input after list"));
    
        std::fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_interpreter_file_read_error() {
        // 模拟文件读取错误
        let invalid_file_path = "invalid_path.lisp";
        
        let mut env = Lisp::initialize();
        let mut io = MockIO::new(b"");
    
        Lisp::interpreter(invalid_file_path, &mut env, &mut io);
        let output = io.get_output();
        assert!(output.is_empty()); // 确保没有输出，因为文件读取失败
    }

    #[test]
    fn test_deeply_nested_expressions() {
        let input = b"(+ 1 (+ 2 (+ 3 (+ 4 (+ 5 (+ 6 (+ 7 (+ 8 9))))))))\nexit\n";
        let mut env = Lisp::initialize();
        let mut io = MockIO::new(input);
    
        Lisp::repl(&mut env, &mut io);
        let output = io.get_output();
        assert!(output.contains("45")); // 检查是否正确计算
    }

    #[test]
    fn test_parser_error_handling() {
        let input = "(+ 1 2"; // 缺少右括号
        let result = Parser::read(input);
        assert_eq!(result, Err(LispError::new("Parse Error: Unexpected end of list")));
    
        let input = "(+ 1 2))"; // 多余的右括号
        let result = Parser::read(input);
        assert_eq!(result, Err(LispError::new("Unexpected input after list")));
    }
    
    #[test]
    fn test_evaluator_error_handling() {
        let input = b"(+ 1 'a)\nexit\n"; // 非数字加法
        let mut env = Lisp::initialize();
        let mut io = MockIO::new(input);
    
        Lisp::repl(&mut env, &mut io);
        let output = io.get_output();
        assert!(output.contains("Error")); // 确保处理错误
    }

    #[test]
    fn test_repl_exit() {
        let input = b"exit\n"; // 立即退出
        let mut env = Lisp::initialize();
        let mut io = MockIO::new(input);
    
        Lisp::repl(&mut env, &mut io);
        let output = io.get_output();
        assert!(output.contains("lisp:> ")); // 确保显示了提示符
    }
    
    #[test]
    fn test_repl_invalid_syntax() {
        let input = b"(+ 1 2))\nexit\n"; // 输入无效语法
        let mut env = Lisp::initialize();
        let mut io = MockIO::new(input);
    
        Lisp::repl(&mut env, &mut io);
        let output = io.get_output();
        assert!(output.contains("Parse Error")); // 确保捕获解析错误
    }

}
