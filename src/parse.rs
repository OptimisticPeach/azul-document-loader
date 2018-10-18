use crate::tokenize::Token;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub enum NType {
    Div,
    Label(String),
    Image(String),
    Text,
}

#[derive(Debug)]
pub struct TextArgument {
    pub body: String,
    pub font: Option<String>,
    pub size: Option<usize>,
}

impl TextArgument {
    fn create(contents: String, font: Option<String>, size: Option<usize>) -> TextArgument {
        TextArgument {
            body: contents,
            font: font,
            size: size,
        }
    }
}

#[derive(Debug)]
pub struct Node(pub NType, pub Option<String>);

#[derive(Debug)]
pub enum ASTPoint {
    Element(Node),
    Joint(Node, Vec<ASTPoint>),
}

fn check_brackets(tokens: &VecDeque<Token>) -> Option<String> {
    #[derive(PartialEq, Debug)]
    enum Bracket {
        Paren,
        Square,
    }

    let mut depths: Vec<Bracket> = Vec::new();

    for i in tokens {
        match i {
            &Token::OpenParen => {
                depths.push(Bracket::Paren);
            }
            &Token::OpenBracket => {
                depths.push(Bracket::Square);
            }
            &Token::CloseParen => {
                if let Some(x) = depths.last() {
                    if *x == Bracket::Paren {
                        depths.pop();
                    } else {
                        return Some(
                            "Attempted to close a square bracket with a parenthesis".to_string(),
                        );
                    }
                } else {
                    return Some("Attempted to close a parenthesis which didn't exist".to_string());
                }
            }
            &Token::CloseBracket => {
                if let Some(x) = depths.last() {
                    if *x == Bracket::Square {
                        depths.pop();
                    } else {
                        return Some(
                            "Attempted to close a parenthesis with a square bracket".to_string(),
                        );
                    }
                } else {
                    return Some(
                        "Attempted to close a square bracket which didn't exist".to_string(),
                    );
                }
            }
            _ => {}
        }
    }

    if depths.len() != 0 {
        return Some(format!(
            "Unclosed delimiters: {}",
            depths
                .iter()
                .map(|x| format!("{:?} ", x).to_string())
                .collect::<Vec<String>>()
                .join("")
        ));
    }

    None
}

fn check_syntax_lightly(source: &VecDeque<Token>) -> Option<String> {
    let allowed_tokens: HashMap<Token, Vec<Token>> = [
        (Token::Div, vec![Token::OpenBracket, Token::Semicolon]),
        (Token::Label, vec![Token::OpenParen]),
        (Token::Image, vec![Token::OpenParen]),
        (
            Token::Semicolon,
            vec![Token::Div, Token::Label, Token::Image, Token::Text, Token::CloseBracket],
        ),
        (
            Token::OpenBracket,
            vec![Token::Div, Token::Label, Token::Image, Token::Text],
        ),
        (
            Token::CloseBracket,
            vec![Token::Div, Token::Label, Token::Image, Token::Text, Token::CloseBracket],
        ),
        (Token::OpenParen, Vec::new()),
        (
            Token::CloseParen,
            vec![Token::Semicolon, Token::OpenBracket],
        ),
        (Token::Text, vec![Token::Semicolon, Token::OpenBracket]),
    ]
        .iter()
        .cloned()
        .collect();
    for i in 0..(source.len() - 1) {
        let current = &source[i];
        let next = &source[i + 1];
        //let previous = &source[i - 1];
        match source[i] {
            Token::Div => {
                if let Token::Id(_) = next {
                    continue;
                }
            }
            Token::Label => {
                if let Token::Id(_) = next {
                    continue;
                }
            }
            Token::Image => {
                if let Token::Id(_) = next {
                    continue;
                }
            }
            Token::Text => {
                if let Token::Id(_) = next {
                    continue;
                }
            }
            Token::OpenParen => {
                if let Token::Str(_) = *next {
                    continue;
                }
            }
            Token::Str(_) => {
                if let Token::Str(_) = *next {
                    continue;
                } else if let Token::Num(_) = *next {
                    continue;
                } else if *next == Token::CloseParen {
                    continue;
                }
                return Some("Improper text syntax".to_string());
            }
            Token::Num(_) => {
                if let Token::Str(_) = source[i - 1] {
                    if let Token::Str(_) = source[i - 2] {
                        continue;
                    }
                }
                panic!("A number must be preceded by two strings: \"Text\" and \"Font\"");
            }
            Token::Id(ref x) => {
                if *next != Token::Semicolon
                    && *next != Token::OpenParen
                    && *next != Token::OpenBracket
                {
                    return Some(format!("Unexpected token after id({}): {:?}", x, next));
                }
                continue;
            }
            _ => {}
        }
        if !allowed_tokens.get(current).unwrap().contains(next) {
            return Some(format!(
                "Unexpected token after {:?}, acceptable tokens: {:?}, found: {:?}",
                current,
                allowed_tokens.get(current),
                next
            ));
        }
    }
    None
}

fn parse_next(source: &mut VecDeque<Token>, text_strings: &mut Vec<TextArgument>) -> ASTPoint {
    let mut head = parse_next_node(source, text_strings);
    if source[0] == Token::OpenBracket {
        source.pop_front();
        match head {
            ASTPoint::Element(x) => {
                head = ASTPoint::Joint(x, parse_in(source, text_strings));
            }
            _ => panic!("Head should not be Joint!!! parse_next()"),
        }
        return head;
    } else if source[0] == Token::Semicolon {
        source.pop_front();
        return head;
    }

    panic!("Unexpected token: {:?}, countdown length: {}, AST: {:?}, texts: {:?}", source[0], source.len(), head, text_strings);
}

fn parse_next_node(source: &mut VecDeque<Token>, text_strings: &mut Vec<TextArgument>) -> ASTPoint {
    match source.pop_front().unwrap() {
        Token::Div => {
            let mut pop = false;
            let z;
            if let Token::Id(ref x) = source[0] {
                pop = true;
                z = ASTPoint::Element(Node(NType::Div, Some(x.clone())));
            } else {
                z = ASTPoint::Element(Node(NType::Div, None));
            }
            if pop {
                source.pop_front();
            }
            z
        }
        Token::Label => {
            let mut pop = false;
            let y = if let Token::Id(ref x) = source[0] {
                pop = true;
                ASTPoint::Element(Node(NType::Label("".to_string()), Some(x.clone())))
            } else {
                ASTPoint::Element(Node(NType::Label("".to_string()), None))
            };
            if pop {
                source.pop_front();
            }
            let string;

            if source.pop_front().unwrap() == Token::OpenParen {
                if let Token::Str(x) = source.pop_front().unwrap() {
                    string = x;
                } else {
                    panic!("Labels should always have some text attached to them");
                }
                if source.pop_front().unwrap() != Token::CloseParen {
                    panic!("Unclosed parenthesis");
                }
            } else {
                panic!("Labels should have text: label (\"abcd\")");
            }

            match y {
                ASTPoint::Element(x) => ASTPoint::Element(Node(NType::Label(string), x.1)),
                _ => panic!("Should not reach this branch..."),
            }
        }
        Token::Image => {
            let mut pop = false;
            let y = if let Token::Id(ref x) = source[0] {
                pop = true;
                ASTPoint::Element(Node(NType::Image("".to_string()), Some(x.clone())))
            } else {
                ASTPoint::Element(Node(NType::Image("".to_string()), None))
            };
            if pop {
                source.pop_front();
            }

            let string;

            if source.pop_front().unwrap() == Token::OpenParen {
                if let Token::Str(x) = source.pop_front().unwrap() {
                    string = x;
                } else {
                    panic!("Images should always have an imageid attached to them");
                }
                if source.pop_front().unwrap() != Token::CloseParen {
                    panic!("Unclosed parenthesis");
                }
            } else {
                panic!("Image should have an imageid: image (\"abcd\")");
            }

            match y {
                ASTPoint::Element(x) => ASTPoint::Element(Node(NType::Image(string), x.1)),
                _ => panic!("Should not reach this branch..."),
            }
        }
        Token::Text => {
            let mut pop = false;
            let original_node = if let Token::Id(ref x) = source[0] {
                pop = true;
                ASTPoint::Element(Node(NType::Text, Some(x.clone())))
            } else {
                ASTPoint::Element(Node(NType::Text, None))
            };

            if pop {
                source.pop_front();
            }

            if source.pop_front().unwrap() == Token::OpenParen {
                if let Token::Str(x) = source.pop_front().unwrap() {
                    let next = source.pop_front().unwrap();
                    if let Token::Str(font) = next {
                        let sizeorparen = source.pop_front().unwrap();
                        if let Token::Num(size) = sizeorparen {
                            if source.pop_front().unwrap() == Token::CloseParen{
                                text_strings.push(TextArgument::create(x, Some(font), Some(size)));
                            }
                            else{
                                panic!("Unexpected token after size in text");
                            }
                        } else if sizeorparen == Token::CloseParen {
                            text_strings.push(TextArgument::create(x, Some(font), None));
                        } else {
                            panic!(
                                "Improper token after a second string inside of a text: {:?}",
                                sizeorparen
                            );
                        }
                    } else if next == Token::CloseParen {
                        text_strings.push(TextArgument::create(x, None, None));
                    } else {
                        panic!(
                            "Improper token after a second string inside of a text: {:?}",
                            next
                        );
                    }
                } else {
                    panic!("Texts should always contain text");
                }
            } else {
                panic!("Text node needs to have contents: text (\"abcd\")");
            }

            match original_node {
                ASTPoint::Element(x) => ASTPoint::Element(Node(NType::Text, x.1)),
                _ => panic!("Should not reach this branch..."),
            }
        }
        x => {
            panic!("Unexpected token: {:?}", x);
        }
    }
}

fn parse_in(source: &mut VecDeque<Token>, text_strings: &mut Vec<TextArgument>) -> Vec<ASTPoint> {
    let mut points: Vec<ASTPoint> = Vec::new();
    while source[0] != Token::CloseBracket {
        points.push(parse_next(source, text_strings));
    }
    if points.len() == 0 {
        panic!("Joint length should not be 0");
    }

    source.pop_front();

    points
}

pub fn parse(source: &mut VecDeque<Token>) -> (Vec<TextArgument>, ASTPoint) {
    if let Some(x) = check_brackets(source) {
        panic!("{}", x);
    }
    if let Some(x) = check_syntax_lightly(source) {
        panic!("{}", x);
    }
    let mut strings = Vec::new();
    let ast = parse_next(source, &mut strings);
    (strings, ast)
}
