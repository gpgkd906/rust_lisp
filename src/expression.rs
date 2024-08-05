// expression.rs

use std::fmt;

#[derive(Clone, Debug)]
pub enum Expr {
    Symbol(String),
    Number(i64),
    Float(f64),
    Str(String),
    List(Vec<Expr>),
    DottedPair(Box<Expr>, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Symbol(s) => write!(f, "{}", s),
            Expr::Number(n) => write!(f, "{}", n),
            Expr::Float(n) => write!(f, "{}", n),
            Expr::Str(s) => write!(f, "\"{}\"", s.replace("\"", "\\\"")), // 正确处理引号的转义
            Expr::List(list) => {
                let list_str: Vec<String> = list.iter().map(|expr| format!("{}", expr)).collect();
                write!(f, "({})", list_str.join(" "))
            },
            Expr::DottedPair(car, cdr) => write!(f, "({} . {})", car, cdr),
        }
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Expr::Symbol(a), Expr::Symbol(b)) => a == b,
            (Expr::Number(a), Expr::Number(b)) => a == b,
            (Expr::Float(a), Expr::Float(b)) => a == b,
            (Expr::Str(a), Expr::Str(b)) => a == b,
            (Expr::List(a), Expr::List(b)) => a == b,
            (Expr::DottedPair(a1, a2), Expr::DottedPair(b1, b2)) => a1 == b1 && a2 == b2,
            _ => false,
        }
    }
}

impl Eq for Expr {}

impl Expr {
    #[allow(dead_code)]
    pub fn is_number(&self) -> bool {
        matches!(self, Expr::Number(_))
    }

    #[allow(dead_code)]
    pub fn is_symbol(&self) -> bool {
        matches!(self, Expr::Symbol(_))
    }

    #[allow(dead_code)]
    pub fn is_string(&self) -> bool {
        matches!(self, Expr::Str(_))
    }

    #[allow(dead_code)]
    pub fn is_list(&self) -> bool {
        matches!(self, Expr::List(_))
    }

    #[allow(dead_code)]
    pub fn is_float(&self) -> bool {
        matches!(self, Expr::Float(_))
    }

    #[allow(dead_code)]
    pub fn is_dotted_pair(&self) -> bool {
        matches!(self, Expr::DottedPair(_, _))
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        match self {
            Expr::Number(n) => n.to_string(),
            Expr::Float(n) => n.to_string(),
            Expr::Symbol(s) => s.clone(),
            Expr::Str(s) => format!("\"{}\"", s.replace("\\", "\\\\").replace("\"", "\\\"")),
            Expr::List(list) => {
                let contents: Vec<String> = list.iter().map(|e| e.to_string()).collect();
                format!("({})", contents.join(" "))
            }
            Expr::DottedPair(car, cdr) => format!("({} . {})", car, cdr),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_number() {
        let expr = Expr::Number(42);
        assert!(expr.is_number());
        assert!(!expr.is_symbol());
        assert!(!expr.is_string());
        assert!(!expr.is_list());
    }

    #[test]
    fn test_is_symbol() {
        let expr = Expr::Symbol("foo".to_string());
        assert!(!expr.is_number());
        assert!(expr.is_symbol());
        assert!(!expr.is_string());
        assert!(!expr.is_list());
    }

    #[test]
    fn test_is_string() {
        let expr = Expr::Str("hello".to_string());
        assert!(!expr.is_number());
        assert!(!expr.is_symbol());
        assert!(expr.is_string());
        assert!(!expr.is_list());
    }

    #[test]
    fn test_is_list() {
        let expr = Expr::List(vec![
            Expr::Number(1),
            Expr::Symbol("+".to_string()),
            Expr::Number(2),
        ]);
        assert!(!expr.is_number());
        assert!(!expr.is_symbol());
        assert!(!expr.is_string());
        assert!(expr.is_list());
    }

    #[test]
    fn test_to_string_number() {
        let expr = Expr::Number(42);
        assert_eq!(expr.to_string(), "42");
    }

    #[test]
    fn test_to_string_symbol() {
        let expr = Expr::Symbol("foo".to_string());
        assert_eq!(expr.to_string(), "foo");
    }

    #[test]
    fn test_to_string_string() {
        let expr = Expr::Str("hello".to_string());
        assert_eq!(expr.to_string(), "\"hello\"");
    }

    #[test]
    fn test_to_string_list() {
        let expr = Expr::List(vec![
            Expr::Symbol("+".to_string()),
            Expr::Number(1),
            Expr::Number(2),
        ]);
        assert_eq!(expr.to_string(), "(+ 1 2)");
    }

    #[test]
    fn test_to_string_nested_list() {
        let expr = Expr::List(vec![
            Expr::Symbol("*".to_string()),
            Expr::Number(2),
            Expr::List(vec![
                Expr::Symbol("+".to_string()),
                Expr::Number(1),
                Expr::Number(3),
            ]),
        ]);
        assert_eq!(expr.to_string(), "(* 2 (+ 1 3))");
    }

    #[test]
    fn test_to_string_empty_list() {
        let expr = Expr::List(vec![]);
        assert_eq!(expr.to_string(), "()");
    }

    #[test]
    fn test_to_string_single_element_list() {
        let expr = Expr::List(vec![Expr::Number(42)]);
        assert_eq!(expr.to_string(), "(42)");
    }

    #[test]
    fn test_to_string_quoted_string() {
        let expr = Expr::Str("\"quoted\"".to_string());
        assert_eq!(expr.to_string(), "\"\\\"quoted\\\"\"");
    }

    #[test]
    fn test_empty_string() {
        let expr = Expr::Str("".to_string());
        assert_eq!(expr.to_string(), "\"\"");
    }

    #[test]
    fn test_large_number() {
        let expr = Expr::Number(1234567890);
        assert_eq!(expr.to_string(), "1234567890");
    }

    #[test]
    fn test_special_characters_in_symbol() {
        let expr = Expr::Symbol("!@#$%^&*()".to_string());
        assert_eq!(expr.to_string(), "!@#$%^&*()");
    }

    #[test]
    fn test_whitespace_symbol() {
        let expr = Expr::Symbol(" ".to_string());
        assert_eq!(expr.to_string(), " ");
    }

    #[test]
    fn test_unicode_string() {
        let expr = Expr::Str("こんにちは".to_string());
        assert_eq!(expr.to_string(), "\"こんにちは\"");
    }

    #[test]
    fn test_unicode_symbol() {
        let expr = Expr::Symbol("λ".to_string());
        assert_eq!(expr.to_string(), "λ");
    }

    #[test]
    fn test_display_number() {
        let expr = Expr::Number(42);
        assert_eq!(format!("{}", expr), "42");
    }

    #[test]
    fn test_display_symbol() {
        let expr = Expr::Symbol("foo".to_string());
        assert_eq!(format!("{}", expr), "foo");
    }

    #[test]
    fn test_display_string() {
        let expr = Expr::Str("hello".to_string());
        assert_eq!(format!("{}", expr), "\"hello\"");
    }

    #[test]
    fn test_display_list() {
        let expr = Expr::List(vec![
            Expr::Symbol("+".to_string()),
            Expr::Number(1),
            Expr::Number(2),
        ]);
        assert_eq!(format!("{}", expr), "(+ 1 2)");
    }

    #[test]
    fn test_display_nested_list() {
        let expr = Expr::List(vec![
            Expr::Symbol("*".to_string()),
            Expr::Number(2),
            Expr::List(vec![
                Expr::Symbol("+".to_string()),
                Expr::Number(1),
                Expr::Number(3),
            ]),
        ]);
        assert_eq!(format!("{}", expr), "(* 2 (+ 1 3))");
    }

    #[test]
    fn test_display_empty_list() {
        let expr = Expr::List(vec![]);
        assert_eq!(format!("{}", expr), "()");
    }

    #[test]
    fn test_display_single_element_list() {
        let expr = Expr::List(vec![Expr::Number(42)]);
        assert_eq!(format!("{}", expr), "(42)");
    }

    #[test]
    fn test_display_quoted_string() {
        let expr = Expr::Str("\"quoted\"".to_string());
        assert_eq!(format!("{}", expr), "\"\\\"quoted\\\"\"");
    }

    #[test]
    fn test_list_of_strings() {
        let expr = Expr::List(vec![
            Expr::Str("hello".to_string()),
            Expr::Str("world".to_string()),
        ]);
        assert_eq!(format!("{}", expr), "(\"hello\" \"world\")");
    }

    #[test]
    fn test_list_of_symbols() {
        let expr = Expr::List(vec![
            Expr::Symbol("foo".to_string()),
            Expr::Symbol("bar".to_string()),
        ]);
        assert_eq!(format!("{}", expr), "(foo bar)");
    }

    #[test]
    fn test_mixed_type_list() {
        let expr = Expr::List(vec![
            Expr::Symbol("sum".to_string()),
            Expr::Number(10),
            Expr::Str("ten".to_string()),
            Expr::List(vec![Expr::Symbol("nested".to_string())]),
        ]);
        assert_eq!(format!("{}", expr), "(sum 10 \"ten\" (nested))");
    }

    #[test]
    fn test_complex_nested_list() {
        let expr = Expr::List(vec![
            Expr::Symbol("define".to_string()),
            Expr::List(vec![
                Expr::Symbol("square".to_string()),
                Expr::Symbol("lambda".to_string()),
                Expr::List(vec![Expr::Symbol("x".to_string())]),
                Expr::List(vec![
                    Expr::Symbol("*".to_string()),
                    Expr::Symbol("x".to_string()),
                    Expr::Symbol("x".to_string()),
                ]),
            ]),
        ]);
        assert_eq!(format!("{}", expr), "(define (square lambda (x) (* x x)))");
    }

    #[test]
    fn test_to_string_float() {
        let expr = Expr::Float(42.5);
        assert_eq!(expr.to_string(), "42.5");
    }

    // 测试大浮点数的字符串化
    #[test]
    fn test_to_string_large_float() {
        let expr = Expr::Float(1234567890.123);
        assert_eq!(expr.to_string(), "1234567890.123");
    }

    // 测试空字符串
    #[test]
    fn test_to_string_empty_string() {
        let expr = Expr::Str("".to_string());
        assert_eq!(expr.to_string(), "\"\"");
    }

    // 测试列表中包含空列表
    #[test]
    fn test_to_string_list_with_empty_list() {
        let expr = Expr::List(vec![
            Expr::Symbol("+".to_string()),
            Expr::List(vec![]),
            Expr::Number(2),
        ]);
        assert_eq!(expr.to_string(), "(+ () 2)");
    }

    // 测试带有嵌套空列表的列表
    #[test]
    fn test_to_string_nested_empty_list() {
        let expr = Expr::List(vec![
            Expr::List(vec![
                Expr::List(vec![])
            ])
        ]);
        assert_eq!(expr.to_string(), "((()))");
    }

    // 测试具有特殊字符的字符串
    #[test]
    fn test_to_string_special_characters() {
        let expr = Expr::Str("Hello \"world\"!".to_string());
        assert_eq!(expr.to_string(), "\"Hello \\\"world\\\"!\"");
    }

    // 测试嵌套的带引号字符串
    #[test]
    fn test_to_string_nested_quoted_string() {
        let expr = Expr::Str("\"nested \\\"quotes\\\"\"".to_string());
        assert_eq!(expr.to_string(), "\"\\\"nested \\\\\\\"quotes\\\\\\\"\\\"\"");
    }

    #[test]
    fn test_to_string_escaped_backslashes() {
        let expr = Expr::Str("This is a backslash: \\".to_string());
        assert_eq!(expr.to_string(), "\"This is a backslash: \\\\\"");
    }
    
    #[test]
    fn test_to_string_multiple_nested_quotes() {
        let expr = Expr::Str("\"This is \\\"very\\\" nested\"".to_string());
        assert_eq!(expr.to_string(), "\"\\\"This is \\\\\\\"very\\\\\\\" nested\\\"\"");
    }
    
    #[test]
    fn test_to_string_multiple_backslashes_and_quotes() {
        let expr = Expr::Str("Backslash \\ and quote \"".to_string());
        assert_eq!(expr.to_string(), "\"Backslash \\\\ and quote \\\"\"");
    }
    
    #[test]
    fn test_to_string_nested_quoted_list() {
        let expr = Expr::List(vec![
            Expr::Str("\"first\"".to_string()),
            Expr::Str("\"second \\\"nested\\\"\"".to_string()),
        ]);
        assert_eq!(expr.to_string(), "(\"\\\"first\\\"\" \"\\\"second \\\\\\\"nested\\\\\\\"\\\"\")");
    }
    
    #[test]
    fn test_to_string_multiple_special_characters() {
        let expr = Expr::Str("Special characters: !@#$%^&*()".to_string());
        assert_eq!(expr.to_string(), "\"Special characters: !@#$%^&*()\"");
    }
    
    #[test]
    fn test_to_string_multiple_escaped_quotes_and_symbols() {
        let expr = Expr::Str("Symbols: \"lambda\" 'quote' \\backslash\\".to_string());
        assert_eq!(expr.to_string(), "\"Symbols: \\\"lambda\\\" 'quote' \\\\backslash\\\\\"");
    }

    // 测试浮点数边界情况
    #[test]
    fn test_to_string_small_float() {
        let expr = Expr::Float(0.0000000001);
        assert_eq!(format!("{:e}", expr.to_string().parse::<f64>().unwrap()), "1e-10");
    }
    
    // 测试更复杂的嵌套结构
    #[test]
    fn test_to_string_complex_nested_list_with_all_types() {
        let expr = Expr::List(vec![
            Expr::Symbol("lambda".to_string()),
            Expr::List(vec![Expr::Symbol("x".to_string())]),
            Expr::List(vec![
                Expr::Symbol("if".to_string()),
                Expr::Symbol("x".to_string()),
                Expr::List(vec![
                    Expr::Symbol("*".to_string()),
                    Expr::Number(2),
                    Expr::Float(3.14),
                ]),
                Expr::List(vec![
                    Expr::Symbol("quote".to_string()),
                    Expr::Str("false".to_string()),
                ]),
            ]),
        ]);
        assert_eq!(
            expr.to_string(),
            "(lambda (x) (if x (* 2 3.14) (quote \"false\")))"
        );
    }

    // 测试 PartialEq 实现
    #[test]
    fn test_partial_eq_for_numbers() {
        let expr1 = Expr::Number(42);
        let expr2 = Expr::Number(42);
        let expr3 = Expr::Number(43);
        assert_eq!(expr1, expr2);
        assert_ne!(expr1, expr3);
    }

    #[test]
    fn test_partial_eq_for_floats() {
        let expr1 = Expr::Float(42.0);
        let expr2 = Expr::Float(42.0);
        let expr3 = Expr::Float(43.0);
        assert_eq!(expr1, expr2);
        assert_ne!(expr1, expr3);
    }

    #[test]
    fn test_partial_eq_for_symbols() {
        let expr1 = Expr::Symbol("foo".to_string());
        let expr2 = Expr::Symbol("foo".to_string());
        let expr3 = Expr::Symbol("bar".to_string());
        assert_eq!(expr1, expr2);
        assert_ne!(expr1, expr3);
    }

    #[test]
    fn test_partial_eq_for_strings() {
        let expr1 = Expr::Str("hello".to_string());
        let expr2 = Expr::Str("hello".to_string());
        let expr3 = Expr::Str("world".to_string());
        assert_eq!(expr1, expr2);
        assert_ne!(expr1, expr3);
    }

    #[test]
    fn test_partial_eq_for_lists() {
        let expr1 = Expr::List(vec![Expr::Number(1), Expr::Symbol("x".to_string())]);
        let expr2 = Expr::List(vec![Expr::Number(1), Expr::Symbol("x".to_string())]);
        let expr3 = Expr::List(vec![Expr::Number(2), Expr::Symbol("x".to_string())]);
        assert_eq!(expr1, expr2);
        assert_ne!(expr1, expr3);
    }

    #[test]
    fn test_partial_eq_across_types() {
        let expr_number = Expr::Number(42);
        let expr_float = Expr::Float(42.0);
        let expr_symbol = Expr::Symbol("42".to_string());
        let expr_string = Expr::Str("42".to_string());
        let expr_list = Expr::List(vec![Expr::Number(42)]);

        assert_ne!(expr_number, expr_float);
        assert_ne!(expr_number, expr_symbol);
        assert_ne!(expr_number, expr_string);
        assert_ne!(expr_number, expr_list);
    }
}
