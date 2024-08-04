// parser.rs

use crate::exception::LispError;
use crate::expression::Expr;
use std::str::Chars;

pub struct Parser;

impl Parser {
    // 解析输入字符串并返回 AST
    pub fn read(input: &str) -> Result<Expr, LispError> {
        let mut chars = input.chars().peekable();
        Parser::parse_expr(&mut chars)
    }

    // 解析单个表达式
    fn parse_expr(chars: &mut std::iter::Peekable<Chars>) -> Result<Expr, LispError> {
        Parser::skip_whitespace(chars);
        match chars.peek() {
            Some(&ch) => match ch {
                '(' => Parser::parse_list(chars),
                '\'' => {
                    chars.next(); // 跳过单引号
                    let quoted_expr = Parser::parse_expr(chars)?;
                    Ok(Expr::List(vec![Expr::Symbol("quote".to_string()), quoted_expr]))
                }
                '"' => Parser::parse_string(chars),
                '0'..='9' => Parser::parse_number(chars),
                _ => Parser::parse_symbol(chars),
            },
            None => Err(LispError::new("Unexpected end of input")),
        }
    }

    // 解析列表
    fn parse_list(chars: &mut std::iter::Peekable<Chars>) -> Result<Expr, LispError> {
        chars.next(); // 跳过 '('
        let mut list = Vec::new();
        loop {
            Parser::skip_whitespace(chars);
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

    // 解析符号
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

    // 解析数字
    fn parse_number(chars: &mut std::iter::Peekable<Chars>) -> Result<Expr, LispError> {
        let mut number = String::new();
        while let Some(&ch) = chars.peek() {
            if !ch.is_digit(10) {
                break;
            }
            number.push(chars.next().unwrap());
        }
        number.parse::<i64>()
            .map(Expr::Number)
            .map_err(|_| LispError::new("Invalid number"))
    }

    // 解析字符串
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

    // 跳过空白字符
    fn skip_whitespace(chars: &mut std::iter::Peekable<Chars>) {
        while let Some(&ch) = chars.peek() {
            if !ch.is_whitespace() {
                break;
            }
            chars.next();
        }
    }
}
