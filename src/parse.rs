use once_cell::sync::Lazy;
use regex::Regex;

use crate::token::{Expression, IntAtom, ListExpression, SymbolAtom};

static RE_NUMBER: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9]+$").expect("Invalid regular expression"));
static RE_WORD: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z\+\*/\-]+$").expect("Invalid regular expression"));

/// 文字列をパースしてExpressionを返す
pub fn parse(input_string: &str) -> Result<Box<dyn Expression>, String> {
    let formatted = input_string.replace("(", "( ").replace(")", " )");
    let trimmed = formatted.trim();
    let token_strings: Vec<String> = trimmed
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    parse_rec(token_strings[0..token_strings.len()].to_vec())
}

fn parse_rec(tokens: Vec<String>) -> Result<Box<dyn Expression>, String> {
    // アトム
    if tokens.len() == 1 {
        let token_str = &tokens[0];
        return match token_str.as_str() {
            // int
            s if RE_NUMBER.is_match(s) => Ok(Box::new(IntAtom {
                n: s.parse().map_err(|e| format!("Failed to parse number: {}", e))?,
            })),
            // symbol
            s if RE_WORD.is_match(s) => Ok(Box::new(SymbolAtom {
                name: s.to_string(),
            })),
            _ => Err(format!("Unknown type token: {}", token_str)),
        };
    }

    // リスト
    // TODO: ほかの特殊形式も返せるようにする
    let mut list_expression = ListExpression { elems: Vec::new() };
    if tokens.get(0) != Some(&"(".to_string()) {
        return Err("Expected '(' at start of list".to_string());
    }

    let mut idx = 0;
    loop {
        idx += 1;
        if idx == tokens.len() {
            return Err("Unmatched open parenthesis".to_string());
        }

        let token: String = tokens[idx].clone();  // TODO cloneしない方法はないか

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
                list_expression.elems.push(inner_list);

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
            _ => list_expression.elems.push(parse_rec(vec![token])?),
        }
    }

    Ok(Box::new(list_expression))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        let expected = "IntAtom { n: 42 }";
        let actual = format!("{:?}", parse("42").unwrap());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_symbol() {
        let expected = "SymbolAtom { name: \"add\" }";
        let actual = format!("{:?}", parse("add").unwrap());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_list() {
        let expected = "ListExpression { elems: [SymbolAtom { name: \"+\" }, IntAtom { n: 1 }, ListExpression { elems: [SymbolAtom { name: \"*\" }, IntAtom { n: 2 }, IntAtom { n: 4 }] }] }";
        let actual = format!("{:?}", parse("(+ 1 (* 2 4))").unwrap());
        assert_eq!(actual, expected);
    }
}
