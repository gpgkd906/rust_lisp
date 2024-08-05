// expression.rs

use std::fmt;

#[derive(Clone, Debug)]
pub enum Expr {
    Symbol(String),
    Number(i64),
    Str(String),
    List(Vec<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Symbol(s) => write!(f, "{}", s),
            Expr::Number(n) => write!(f, "{}", n),
            Expr::Str(s) => write!(f, "\"{}\"", s.replace("\"", "\\\"")), // 正确处理引号的转义
            Expr::List(list) => {
                let list_str: Vec<String> = list.iter().map(|expr| format!("{}", expr)).collect();
                write!(f, "({})", list_str.join(" "))
            }
        }
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Expr::Symbol(a), Expr::Symbol(b)) => a == b,
            (Expr::Number(a), Expr::Number(b)) => a == b,
            (Expr::Str(a), Expr::Str(b)) => a == b,
            (Expr::List(a), Expr::List(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for Expr {}

impl Expr {
    pub fn is_number(&self) -> bool {
        matches!(self, Expr::Number(_))
    }

    pub fn is_symbol(&self) -> bool {
        matches!(self, Expr::Symbol(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Expr::Str(_))
    }

    pub fn is_list(&self) -> bool {
        matches!(self, Expr::List(_))
    }

    pub fn to_string(&self) -> String {
        match self {
            Expr::Number(n) => n.to_string(),
            Expr::Symbol(s) => s.clone(),
            Expr::Str(s) => format!("\"{}\"", s.replace("\"", "\\\"")),
            Expr::List(list) => {
                let contents: Vec<String> = list.iter().map(|e| e.to_string()).collect();
                format!("({})", contents.join(" "))
            }
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
}
