use once_cell::sync::Lazy;
use regex::Regex;

use crate::token::{Expression, IntAtom, ListExpression, SymbolAtom};

static RE_NUMBER: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9]+$").unwrap());
static RE_WORD: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z\+\*/\-]+$").unwrap());

pub fn parse(s: &str) -> Box<dyn Expression> {
    let result = s.replace("(", "( ").replace(")", " )");
    let trimmed_str = result.trim();
    let tokens: Vec<String> = trimmed_str
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    parse_rec(tokens[0..tokens.len()].to_vec())
}

fn parse_rec(tokens: Vec<String>) -> Box<dyn Expression> {
    if tokens.len() == 1 {
        let token_str = &tokens[0];
        return match token_str.as_str() {
            s if RE_NUMBER.is_match(s) => Box::new(IntAtom {
                n: s.parse().unwrap(),
            }),
            s if RE_WORD.is_match(s) => Box::new(SymbolAtom {
                name: s.to_string(),
            }),
            _ => panic!("Unknown type token: {}", token_str),
        };
    }

    // リスト
    // TODO: 本来はほかの特殊形式も返せるようにする
    let mut ls = ListExpression { elems: Vec::new() };
    assert!(tokens.get(0).unwrap() == "(");

    let mut idx = 0;
    loop {
        idx += 1;
        if idx == tokens.len() {
            break;
        }

        let token: String = tokens[idx].clone();

        match token.as_str() {
            "(" => {
                let mut end_j = 0;
                for j in (idx + 1)..(tokens.len()) {
                    if tokens[j] == ")" {
                        end_j = j;
                        break;
                    }
                }

                if end_j == 0 {
                    panic!("Ummatched open parenthesis");
                }

                let inner_list_s = &tokens[idx..(end_j + 1)];
                let inner_list = parse_rec(inner_list_s.to_vec());
                ls.elems.push(inner_list);
                idx = end_j;
                continue;
            }
            ")" => {
                if idx == (tokens.len() - 1) {
                    break;
                } else {
                    panic!("Unmatched close parenthesis");
                }
            }
            _ => ls.elems.push(parse_rec(vec![token])),
        }
    }
    // TODO かっこが対応していなかったらエラー出す

    Box::new(ls)
}
