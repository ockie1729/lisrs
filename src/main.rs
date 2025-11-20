use once_cell::sync::Lazy;
use regex::Regex;
use std::io;

static RE_NUMBER: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9]+$").unwrap());
static RE_WORD: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z\+\*/\-]+$").unwrap());

use std::fmt;

trait Expression: fmt::Debug {}

trait Atom {
    type Val;

    fn val(&self) -> &Self::Val;
}

#[derive(Debug)]
struct IntAtom {
    n: i32,
}

impl Expression for IntAtom {}

impl Atom for IntAtom {
    type Val = i32;

    fn val(&self) -> &Self::Val {
        &self.n
    }
}

#[derive(Debug)]
struct SymbolAtom {
    name: String,
}

impl Expression for SymbolAtom {}

impl Atom for SymbolAtom {
    type Val = String;

    fn val(&self) -> &Self::Val {
        &self.name
    }
}

// TODO Boxとdynの意味を理解する
#[derive(Debug)]
struct ListExpression {
    elems: Vec<Box<dyn Expression>>,
}

impl Expression for ListExpression {}

fn main() {
    let mut input_str: String = String::new();

    println!("input lisp expressoin:");
    io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");

    let tokens: Box<dyn Expression> = parse(&input_str);

    // FIXME debug
    println!("{tokens:?}");
}

fn parse(s: &str) -> Box<dyn Expression> {
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
