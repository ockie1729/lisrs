use once_cell::sync::Lazy;
use regex::Regex;

use crate::token::Expr;

static RE_NUMBER: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[0-9]+$").expect("Invalid regular expression"));
static RE_WORD: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z\+\*/\-]+$").expect("Invalid regular expression"));

/// 文字列をパースしてExprを返す
pub fn parse(input_string: &str) -> Result<Expr, String> {
    let formatted = input_string.replace("(", "( ").replace(")", " )");
    let trimmed = formatted.trim();
    let token_strings: Vec<String> = trimmed.split_whitespace().map(|s| s.to_string()).collect();
    parse_rec(token_strings[0..token_strings.len()].to_vec())
}

fn parse_rec(tokens: Vec<String>) -> Result<Expr, String> {
    // アトム
    if tokens.len() == 1 {
        let token_str = &tokens[0];
        return match token_str.as_str() {
            // int
            s if RE_NUMBER.is_match(s) => Ok(Expr::Int(
                s.parse()
                    .map_err(|e| format!("Failed to parse number: {}", e))?,
            )),
            // symbol
            s if RE_WORD.is_match(s) => Ok(Expr::Symbol(s.to_string())),
            _ => Err(format!("Unknown type token: {}", token_str)),
        };
    }

    // リスト・Lambda式
    // TODO: ほかの特殊形式も返せるようにする
    if tokens.get(0) != Some(&"(".to_string()) {
        return Err("Expected '(' at start of list".to_string());
    }

    // Lambda式
    let token = tokens.get(1).ok_or("Invalid syntax")?; // TODO エラーを返す
    if token == "lambda" {
        let t = tokens.get(2).ok_or("Invalid syntax")?; // TODO エラーを返す
        if t != "(" {
            return Err("Invalid syntax".to_string());
        }

        let mut args: Vec<String> = vec![];
        let mut body_begin_idx = 0;
        for i in 3..(tokens.len() - 2) {
            match tokens.get(i) {
                None => return Err("Invalid syntax".to_string()),
                Some(s) => match s.as_str() {
                    ")" => {
                        body_begin_idx = i;
                        break;
                    }
                    _ => args.push(s.to_string()),
                },
            }
        }
        if body_begin_idx <= 0 {
            return Err("Invalid syntax".to_string());
        }

        let body = match parse_rec(tokens[body_begin_idx + 1..tokens.len() - 1].to_vec()) {
            Err(s) => return Err(s),
            Ok(t) => t,
        };

        return Ok(Expr::Lambda(args, Box::new(body)));
    }

    // リスト
    let mut elems: Vec<Expr> = Vec::new();
    let mut idx = 0;
    loop {
        idx += 1;
        if idx == tokens.len() {
            return Err("Unmatched open parenthesis".to_string());
        }

        let token: String = tokens[idx].clone(); // TODO cloneしない方法はないか

        match token.as_str() {
            // リストの内部のリストをパース
            "(" => {
                let mut list_end: usize = 0;
                for j in (idx + 1)..(tokens.len()) {
                    if tokens[j] == ")" {
                        list_end = j;
                        break;
                    }
                }
                if list_end == 0 {
                    return Err("Unmatched open parenthesis".to_string());
                }

                let s = &tokens[idx..(list_end + 1)];
                let inner_list = parse_rec(s.to_vec())?;
                elems.push(inner_list);

                idx = list_end;
                continue;
            }
            // リスト末尾（のはずの）の閉じかっこを処理
            ")" => {
                if idx == (tokens.len() - 1) {
                    break;
                } else {
                    return Err("Unmatched close parenthesis".to_string());
                }
            }
            // それ以外は単独のtokenをパース
            _ => elems.push(parse_rec(vec![token])?),
        }
    }

    Ok(Expr::List(elems))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        let expected = Expr::Int(42);
        let actual = parse("42").unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_symbol() {
        let expected = Expr::Symbol("add".to_string());
        let actual = parse("add").unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_list() {
        let expected = Expr::List(vec![
            Expr::Symbol("+".to_string()),
            Expr::Int(1),
            Expr::List(vec![
                Expr::Symbol("*".to_string()),
                Expr::Int(2),
                Expr::Int(4),
            ]),
        ]);
        let actual = parse("(+ 1 (* 2 4))").unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_lambda() {
        let expected = Expr::Lambda(
            vec!["x".to_string(), "y".to_string()],
            Box::new(Expr::List(vec![
                Expr::Symbol("+".to_string()),
                Expr::Symbol("x".to_string()),
                Expr::Int(2),
            ])),
        );
        let actual = parse("(lambda (x y) (+ x 2))").unwrap();
        assert_eq!(actual, expected);
    }
}
