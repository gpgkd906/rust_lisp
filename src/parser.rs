// parser.rs

use crate::exception::LispError;
use crate::expression::Expr;
use std::str::Chars;

pub struct Parser;

impl Parser {
    // 解析输入字符串并返回 AST
    pub fn read(input: &str) -> Result<Expr, LispError> {
        let mut chars = input.chars().peekable();
        let expr = Parser::parse_expr(&mut chars)?;

        // 确保解析完成后没有多余的输入
        Parser::skip_whitespace_and_comments(&mut chars);
        if chars.peek().is_some() {
            return Err(LispError::new("Unexpected input after list"));
        }

        Ok(expr)
    }

    fn parse_expr(chars: &mut std::iter::Peekable<Chars>) -> Result<Expr, LispError> {
        Parser::skip_whitespace_and_comments(chars);
        if let Some(&ch) = chars.peek() {
            match ch {
                '(' => Parser::parse_list(chars),
                '\'' => {
                    chars.next(); // 跳过单引号
                    let quoted_expr = Parser::parse_expr(chars)?;
                    Ok(Expr::List(vec![Expr::Symbol("quote".to_string()), quoted_expr]))
                }
                '"' => Parser::parse_string(chars),
                '-' => { // 检查减号或负号
                    chars.next(); // 先取出'-'
                    if let Some(&next_ch) = chars.peek() {
                        if next_ch.is_digit(10) { // 负号情况
                            Parser::parse_number_with_leading_sign(chars, true)
                        } else { // 减号情况
                            Ok(Expr::Symbol("-".to_string()))
                        }
                    } else {
                        Err(LispError::new("Invalid number"))
                    }
                }
                '0'..='9' => Parser::parse_number(chars),
                _ => Parser::parse_symbol(chars),
            }
        } else {
            Err(LispError::new("Unexpected end of input"))
        }
    }

    fn parse_list(chars: &mut std::iter::Peekable<Chars>) -> Result<Expr, LispError> {
        chars.next(); // 跳过 '('
        let mut list = Vec::new();
        loop {
            Parser::skip_whitespace_and_comments(chars);
            if let Some(&ch) = chars.peek() {
                if ch == ')' {
                    chars.next(); // 跳过 ')'
                    break;
                }
                list.push(Parser::parse_expr(chars)?);
            } else {
                return Err(LispError::new("Unexpected end of list"));
            }
        }
        Ok(Expr::List(list))
    }

    fn parse_symbol(chars: &mut std::iter::Peekable<Chars>) -> Result<Expr, LispError> {
        let mut symbol = String::new();
        while let Some(&ch) = chars.peek() {
            if ch.is_whitespace() || ch == '(' || ch == ')' {
                break;
            }
            symbol.push(chars.next().unwrap());
        }
        Ok(Expr::Symbol(symbol))
    }

    fn parse_number_with_leading_sign(chars: &mut std::iter::Peekable<Chars>, is_negative: bool) -> Result<Expr, LispError> {
        let mut number = String::new();

        if is_negative {
            number.push('-');
        }

        while let Some(&ch) = chars.peek() {
            if !ch.is_digit(10) {
                break;
            }
            number.push(chars.next().unwrap());
        }

        // 确保数字有效
        if number.len() == 1 && is_negative {
            return Err(LispError::new("Invalid number"));
        }

        number.parse::<i64>()
            .map(Expr::Number)
            .map_err(|_| LispError::new("Invalid number"))
    }

    fn parse_number(chars: &mut std::iter::Peekable<Chars>) -> Result<Expr, LispError> {
        let mut number = String::new();
        let mut is_number = false;

        // 读取所有连续的数字字符
        while let Some(&ch) = chars.peek() {
            if ch.is_digit(10) {
                number.push(chars.next().unwrap());
                is_number = true;
            } else {
                break;
            }
        }

        // 检查是否成功读取了一个有效的数字
        if !is_number {
            return Err(LispError::new("Invalid number"));
        }

        // 检查后续字符是否为非法字符
        if let Some(&ch) = chars.peek() {
            if !ch.is_whitespace() && ch != '(' && ch != ')' && ch != ';' {
                return Err(LispError::new("Invalid number"));
            }
        }

        // 将字符串解析为整数
        number.parse::<i64>()
            .map(Expr::Number)
            .map_err(|_| LispError::new("Invalid number"))
    }
    
    fn parse_string(chars: &mut std::iter::Peekable<Chars>) -> Result<Expr, LispError> {
        chars.next(); // 跳过 '"'
        let mut string = String::new();
        while let Some(&ch) = chars.peek() {
            match ch {
                '"' => {
                    chars.next(); // 跳过结尾的 '"'
                    return Ok(Expr::Str(string));
                }
                _ => string.push(chars.next().unwrap()),
            }
        }
        Err(LispError::new("Unterminated string literal"))
    }

    // 跳过空白字符和注释
    fn skip_whitespace_and_comments(chars: &mut std::iter::Peekable<Chars>) {
        while let Some(&ch) = chars.peek() {
            if ch.is_whitespace() {
                chars.next();
            } else if ch == ';' {
                // 跳过整行注释
                while let Some(&ch) = chars.peek() {
                    chars.next();
                    if ch == '\n' {
                        break;
                    }
                }
            } else {
                break;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::Expr;
    use crate::exception::LispError;

    #[test]
    fn test_parse_number() {
        let input = "42";
        let result = Parser::read(input);
        assert_eq!(result, Ok(Expr::Number(42)));
    }

    #[test]
    fn test_parse_negative_number() {
        let input = "-42";
        let result = Parser::read(input);
        assert_eq!(result, Ok(Expr::Number(-42)));
    }

    #[test]
    fn test_parse_subtraction() {
        let input = "(- 2 1)"; // 减法表达式
        let result = Parser::read(input);
        assert!(result.is_ok());
        if let Ok(expr) = result {
            assert_eq!(expr, Expr::List(vec![
                Expr::Symbol("-".to_string()), 
                Expr::Number(2), 
                Expr::Number(1)
            ]));
        }
    }

    #[test]
    fn test_parse_invalid_number() {
        let input = "42abc"; // 无效数字后跟随非法字符
        let result = Parser::read(input);
        assert_eq!(result, Err(LispError::new("Invalid number")));
    }

    #[test]
    fn test_parse_number_with_inline_comment() {
        let input = "123;test";
        let result = Parser::read(input);
        assert_eq!(result, Ok(Expr::Number(123))); // 解析成功，忽略注释部分
    }

    #[test]
    fn test_parse_number_with_space_and_comment() {
        let input = "123 ;test";
        let result = Parser::read(input);
        assert_eq!(result, Ok(Expr::Number(123))); // 解析成功，忽略注释部分
    }

    #[test]
    fn test_parse_negative_sign_without_number() {
        let input = "-";
        let result = Parser::read(input);
        assert_eq!(result, Err(LispError::new("Invalid number")));
    }

    #[test]
    fn test_parse_symbol() {
        let input = "foo";
        let result = Parser::read(input);
        assert_eq!(result, Ok(Expr::Symbol("foo".to_string())));
    }

    #[test]
    fn test_parse_simple_expression() {
        let input = "(+ 1 2)";
        let result = Parser::read(input);
        assert!(result.is_ok());
        if let Ok(expr) = result {
            assert_eq!(expr, Expr::List(vec![
                Expr::Symbol("+".to_string()), 
                Expr::Number(1), 
                Expr::Number(2)
            ]));
        }
    }

    #[test]
    fn test_parse_nested_expression() {
        let input = "(+ 1 (* 2 3))";
        let result = Parser::read(input);
        assert!(result.is_ok());
        if let Ok(expr) = result {
            assert_eq!(expr, Expr::List(vec![
                Expr::Symbol("+".to_string()), 
                Expr::Number(1), 
                Expr::List(vec![
                    Expr::Symbol("*".to_string()), 
                    Expr::Number(2), 
                    Expr::Number(3)
                ])
            ]));
        }
    }

    #[test]
    fn test_parse_quoted_expression() {
        let input = "'(1 2 3)";
        let result = Parser::read(input);
        assert!(result.is_ok());
        if let Ok(expr) = result {
            assert_eq!(expr, Expr::List(vec![
                Expr::Symbol("quote".to_string()), 
                Expr::List(vec![
                    Expr::Number(1), 
                    Expr::Number(2), 
                    Expr::Number(3)
                ])
            ]));
        }
    }

    #[test]
    fn test_parse_string() {
        let input = "\"hello\"";
        let result = Parser::read(input);
        assert_eq!(result, Ok(Expr::Str("hello".to_string())));
    }

    #[test]
    fn test_parse_unterminated_string() {
        let input = "\"hello";
        let result = Parser::read(input);
        assert_eq!(result, Err(LispError::new("Unterminated string literal")));
    }

    #[test]
    fn test_parse_empty_input() {
        let input = "";
        let result = Parser::read(input);
        assert_eq!(result, Err(LispError::new("Unexpected end of input")));
    }

    #[test]
    fn test_parse_whitespace_only_input() {
        let input = "   ";
        let result = Parser::read(input);
        assert_eq!(result, Err(LispError::new("Unexpected end of input")));
    }

    #[test]
    fn test_parse_invalid_symbol() {
        let input = "@#$%";
        let result = Parser::read(input);
        assert_eq!(result, Ok(Expr::Symbol("@#$%".to_string()))); // 符号解析无异常
    }

    #[test]
    fn test_parse_unexpected_end_of_list() {
        let input = "(1 2 3";
        let result = Parser::read(input);
        assert_eq!(result, Err(LispError::new("Unexpected end of list")));
    }

    #[test]
    fn test_parse_extra_closing_paren() {
        let input = "(1 2 3))";
        let result = Parser::read(input);
        assert_eq!(result, Err(LispError::new("Unexpected input after list")));
    }

    #[test]
    fn test_parse_nested_error() {
        let input = "(1 (2 3";
        let result = Parser::read(input);
        assert_eq!(result, Err(LispError::new("Unexpected end of list")));
    }

    #[test]
    fn test_parse_with_comments() {
        let input = "
            ; this is a comment
            (fib 6) ;should be 8
            ;exit
        ";
        let result = Parser::read(input);
        assert!(result.is_ok());
        if let Ok(expr) = result {
            assert_eq!(expr, Expr::List(vec![
                Expr::Symbol("fib".to_string()), 
                Expr::Number(6)
            ]));
        }
    }

    #[test]
    fn test_parse_inline_comment() {
        let input = "(+ 1 2) ; this adds two numbers";
        let result = Parser::read(input);
        assert!(result.is_ok());
        if let Ok(expr) = result {
            assert_eq!(expr, Expr::List(vec![
                Expr::Symbol("+".to_string()), 
                Expr::Number(1), 
                Expr::Number(2)
            ]));
        }
    }
}
