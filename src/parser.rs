// parser.rs
use crate::exception::LispError;
use crate::expression::Expr;
use std::str::Chars;

pub struct Parser;

impl Parser {
    // Parse the input string and return the AST
    pub fn read(input: &str) -> Result<Expr, LispError> {
        let mut chars = input.chars().peekable();
        
        // 如果输入是空的或者全是空白，返回空列表
        Parser::skip_whitespace_and_comments(&mut chars);
        if chars.peek().is_none() {
            return Ok(Expr::List(vec![])); // 解析器返回空列表作为合法输入
        }
        
        let expr = Parser::parse_expr(&mut chars)?;

        // Ensure there's no unexpected input after parsing
        Parser::skip_whitespace_and_comments(&mut chars);
        if chars.peek().is_some() {
            return Err(LispError::new("Unexpected input after list"));
        }

        Ok(expr)
    }

    pub fn parse_expr(chars: &mut std::iter::Peekable<Chars>) -> Result<Expr, LispError> {
        Parser::skip_whitespace_and_comments(chars);
        if let Some(&ch) = chars.peek() {
            match ch {
                '(' => Parser::parse_list(chars),
                '\'' => {
                    chars.next(); // Skip the single quote
                    let quoted_expr = Parser::parse_expr(chars)?;
                    Ok(Expr::List(vec![Expr::Symbol("quote".to_string()), quoted_expr]))
                }
                '`' => {
                    chars.next(); // Skip the backquote
                    let quoted_expr = Parser::parse_expr(chars)?;
                    Ok(Expr::List(vec![Expr::Symbol("quasiquote".to_string()), quoted_expr]))
                }
                ',' => {
                    chars.next(); // Skip the comma
                    let unquoted_expr = Parser::parse_expr(chars)?;
                    Ok(Expr::List(vec![Expr::Symbol("unquote".to_string()), unquoted_expr]))
                }
                '"' => Parser::parse_string(chars),
                '-' => {
                    chars.next(); // Consume the '-'
                    if let Some(&next_ch) = chars.peek() {
                        if next_ch.is_digit(10) || next_ch == '.' {
                            Parser::parse_number_with_leading_sign(chars, true)
                        } else if next_ch.is_whitespace() || next_ch == '(' || next_ch == ')' {
                            // Treat as a subtraction operator if followed by space, open or close parenthesis
                            Ok(Expr::Symbol("-".to_string()))
                        } else {
                            Parser::parse_symbol_with_leading_minus(chars)
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
        
    fn parse_symbol_with_leading_minus(chars: &mut std::iter::Peekable<Chars>) -> Result<Expr, LispError> {
        let mut symbol = String::from("-");
        while let Some(&ch) = chars.peek() {
            if ch.is_whitespace() || ch == '(' || ch == ')' {
                break;
            }
            symbol.push(chars.next().unwrap());
        }
        Ok(Expr::Symbol(symbol))
    }
    
    fn parse_list(chars: &mut std::iter::Peekable<Chars>) -> Result<Expr, LispError> {
        chars.next(); // Skip '('
        let mut list = Vec::new();
        loop {
            Parser::skip_whitespace_and_comments(chars);
            if let Some(&ch) = chars.peek() {
                if ch == ')' {
                    chars.next(); // Skip ')'
                    break;
                }
                list.push(Parser::parse_expr(chars)?);
            } else {
                return Err(LispError::new("Parse Error: Unexpected end of list")); // 确保在缺少右括号时生成错误
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
    
        let mut is_float = false;
    
        while let Some(&ch) = chars.peek() {
            if ch.is_digit(10) || ch == '.' {
                if ch == '.' {
                    if is_float {
                        return Err(LispError::new("Invalid float"));
                    }
                    is_float = true;
                }
                number.push(chars.next().unwrap());
            } else {
                break;
            }
        }
    
        // Ensure a valid number is read
        if number.len() == 1 && is_negative {
            return Ok(Expr::Symbol("-".to_string()));  // Treat it as a symbol if only "-"
        }
    
        // Check if the number is a float or an integer
        if is_float {
            number.parse::<f64>()
                .map(Expr::Float)
                .map_err(|_| LispError::new("Invalid float"))
        } else {
            number.parse::<i64>()
                .map(Expr::Number)
                .map_err(|_| LispError::new("Invalid number"))
        }
    }
    
    fn parse_number(chars: &mut std::iter::Peekable<Chars>) -> Result<Expr, LispError> {
        let mut number = String::new();
        let mut is_float = false;

        // Read all continuous digits and periods
        while let Some(&ch) = chars.peek() {
            if ch.is_digit(10) || ch == '.' {
                if ch == '.' {
                    if is_float {
                        return Err(LispError::new("Invalid float"));
                    }
                    is_float = true;
                }
                number.push(chars.next().unwrap());
            } else {
                break;
            }
        }

        // Ensure a valid number is read
        if number.is_empty() || number == "." {
            return Err(LispError::new("Invalid number"));
        }

        // Check next character legality
        if let Some(&ch) = chars.peek() {
            if !ch.is_whitespace() && ch != '(' && ch != ')' && ch != ';' {
                return Err(LispError::new("Invalid number"));
            }
        }

        // Parse as integer or float
        if is_float {
            number.parse::<f64>()
                .map(Expr::Float)
                .map_err(|_| LispError::new("Invalid float"))
        } else {
            number.parse::<i64>()
                .map(Expr::Number)
                .map_err(|_| LispError::new("Invalid number"))
        }
    }
    
    fn parse_string(chars: &mut std::iter::Peekable<Chars>) -> Result<Expr, LispError> {
        chars.next(); // Skip '"'
        let mut string = String::new();
        while let Some(&ch) = chars.peek() {
            match ch {
                '"' => {
                    chars.next(); // Skip the closing '"'
                    return Ok(Expr::Str(string));
                }
                _ => string.push(chars.next().unwrap()),
            }
        }
        Err(LispError::new("Unterminated string literal"))
    }

    // Skip whitespace characters and comments
    fn skip_whitespace_and_comments(chars: &mut std::iter::Peekable<Chars>) {
        while let Some(&ch) = chars.peek() {
            if ch.is_whitespace() || ch == '\n' || ch == '\r' {
                chars.next();
            } else if ch == ';' {
                // Skip entire line comments
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
        let input = "(- 2 1)";
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
        let input = "42abc";
        let result = Parser::read(input);
        assert_eq!(result, Err(LispError::new("Invalid number")));
    }

    #[test]
    fn test_parse_number_with_inline_comment() {
        let input = "123;test";
        let result = Parser::read(input);
        assert_eq!(result, Ok(Expr::Number(123)));
    }

    #[test]
    fn test_parse_number_with_space_and_comment() {
        let input = "123 ;test";
        let result = Parser::read(input);
        assert_eq!(result, Ok(Expr::Number(123)));
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
        // 对于空输入，应返回空列表而非错误
        assert_eq!(result, Ok(Expr::List(vec![])));
    }

    #[test]
    fn test_parse_whitespace_only_input() {
        let input = "   ";
        let result = Parser::read(input);
        // 对于仅空白的输入，应返回空列表而非错误
        assert_eq!(result, Ok(Expr::List(vec![])));
    }

    #[test]
    fn test_parse_invalid_symbol() {
        let input = "@#$%";
        let result = Parser::read(input);
        assert_eq!(result, Ok(Expr::Symbol("@#$%".to_string())));
    }

    #[test]
    fn test_parse_unexpected_end_of_list() {
        let input = "(1 2 3";
        let result = Parser::read(input);
        assert_eq!(result, Err(LispError::new("Parse Error: Unexpected end of list")));
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
        assert_eq!(result, Err(LispError::new("Parse Error: Unexpected end of list")));
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

    #[test]
    fn test_parse_multiline_expression() {
        let input = "(defun fib (n)\n\
                     (cond ((eq n 1) 1)\n\
                           ((eq n 0) 0)\n\
                           (t (+ (fib (- n 1)) (fib (- n 2))))))";
        let result = Parser::read(input);
        assert!(result.is_ok());
        if let Ok(expr) = result {
            assert_eq!(
                expr,
                Expr::List(vec![
                    Expr::Symbol("defun".to_string()),
                    Expr::Symbol("fib".to_string()),
                    Expr::List(vec![Expr::Symbol("n".to_string())]),
                    Expr::List(vec![
                        Expr::Symbol("cond".to_string()),
                        Expr::List(vec![
                            Expr::List(vec![Expr::Symbol("eq".to_string()), Expr::Symbol("n".to_string()), Expr::Number(1)]),
                            Expr::Number(1)
                        ]),
                        Expr::List(vec![
                            Expr::List(vec![Expr::Symbol("eq".to_string()), Expr::Symbol("n".to_string()), Expr::Number(0)]),
                            Expr::Number(0)
                        ]),
                        Expr::List(vec![
                            Expr::Symbol("t".to_string()),
                            Expr::List(vec![
                                Expr::Symbol("+".to_string()),
                                Expr::List(vec![Expr::Symbol("fib".to_string()), Expr::List(vec![Expr::Symbol("-".to_string()), Expr::Symbol("n".to_string()), Expr::Number(1)])]),
                                Expr::List(vec![Expr::Symbol("fib".to_string()), Expr::List(vec![Expr::Symbol("-".to_string()), Expr::Symbol("n".to_string()), Expr::Number(2)])])
                            ])
                        ])
                    ])
                ])
            );
        }
    }
        
    #[test]
    fn test_parse_negative_symbol() {
        let input = "-abc";
        let result = Parser::read(input);
        assert_eq!(result, Ok(Expr::Symbol("-abc".to_string()))); // Correctly parsed as a symbol
    }

    #[test]
    fn test_parse_negative_float() {
        let input = "-3.14";
        let result = Parser::read(input);
        assert_eq!(result, Ok(Expr::Float(-3.14)));
    }
        
    #[test]
    fn test_parse_float() {
        let input = "3.14";
        let result = Parser::read(input);
        assert_eq!(result, Ok(Expr::Float(3.14)));
    }
    
    #[test]
    fn test_parse_invalid_float() {
        let input = "3.14.159";
        let result = Parser::read(input);
        assert_eq!(result, Err(LispError::new("Invalid float")));
    }

    #[test]
    fn test_parse_symbol_with_numbers() {
        let input = "foo123";
        let result = Parser::read(input);
        assert_eq!(result, Ok(Expr::Symbol("foo123".to_string())));
    }
    
    #[test]
    fn test_parse_symbol_with_special_characters() {
        let input = "foo-bar_baz!";
        let result = Parser::read(input);
        assert_eq!(result, Ok(Expr::Symbol("foo-bar_baz!".to_string())));
    }

    #[test]
    fn test_parse_expression_with_leading_whitespace() {
        let input = "   (+ 1 2)";
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
    fn test_parse_expression_with_trailing_whitespace() {
        let input = "(+ 1 2)   ";
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
    fn test_parse_expression_with_comment_inside_list() {
        let input = "(+ 1 ; comment\n 2)";
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
    fn test_parse_complex_expression() {
        let input = "(begin\n\
                        (define r 10) ; radius\n\
                        (define pi 3.14159)\n\
                        (define area (* pi (* r r)))\n\
                        area)";
        let result = Parser::read(input);
        assert!(result.is_ok());
        if let Ok(expr) = result {
            assert_eq!(expr, Expr::List(vec![
                Expr::Symbol("begin".to_string()),
                Expr::List(vec![
                    Expr::Symbol("define".to_string()),
                    Expr::Symbol("r".to_string()),
                    Expr::Number(10)
                ]),
                Expr::List(vec![
                    Expr::Symbol("define".to_string()),
                    Expr::Symbol("pi".to_string()),
                    Expr::Float(3.14159)
                ]),
                Expr::List(vec![
                    Expr::Symbol("define".to_string()),
                    Expr::Symbol("area".to_string()),
                    Expr::List(vec![
                        Expr::Symbol("*".to_string()),
                        Expr::Symbol("pi".to_string()),
                        Expr::List(vec![
                            Expr::Symbol("*".to_string()),
                            Expr::Symbol("r".to_string()),
                            Expr::Symbol("r".to_string())
                        ])
                    ])
                ]),
                Expr::Symbol("area".to_string())
            ]));
        }
    }
    
}
