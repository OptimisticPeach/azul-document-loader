#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum Token {
    Div,
    Label,
    Image,
    Text,
    Semicolon,
    OpenBracket,
    CloseBracket,
    OpenParen,
    CloseParen,
    Num(usize),
    Id(String),
    Str(String),
}

fn read_until<F>(source: &Vec<char>, index: &mut usize, check: &F) -> String
where
    F: Fn(char) -> bool,
{
    let mut string = "".to_string();
    let mut new_index = 0;
    loop {
        if check(source[*index + new_index]) {
            break;
        }
        string.push(source[*index + new_index]);
        new_index += 1;
    }
    *index += new_index;
    string
}

fn ignore_whitespace(source: &Vec<char>, index: &mut usize){
    loop {
        if source[*index] != ' ' && source[*index] != '\r' && source[*index] != '\t' && source[*index] != '\n' {
            break;
        }
        *index += 1;
    }
}

pub fn tokenize(source: &String) -> Vec<Token> {
    let dictionary: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_"
        .chars()
        .collect();
    let numbers: Vec<char> = "1234567890".chars().collect();
    let source: Vec<char> = source.chars().collect();
    let mut output: Vec<Token> = Vec::new();
    let mut index = 0;

    let mut line_num = 0;

    while index < source.len() {
        ignore_whitespace(&source, &mut index);
        let matched = match source[index] {
            ';' => {
                output.push(Token::Semicolon);
                true
            }
            ':' => false,
            '(' => {
                output.push(Token::OpenParen);
                true
            }
            ')' => {
                output.push(Token::CloseParen);
                true
            }
            '[' => {
                output.push(Token::OpenBracket);
                true
            }
            ']' => {
                output.push(Token::CloseBracket);
                true
            }
            _ => false,
        };
        if matched {
            index += 1;
            continue;
        }

        if source[index] == '{' {
            if !source[index..].contains(&'}') {
                panic!("Unclosed Comment at line {}", line_num);
            }
            read_until(&source, &mut index, &|x: char| x == '}');
            index += 1;
            continue;
        }

        if source[index] == ':' {
            // let mut id_name = String::new();
            // let mut new_index = 1;
            // while dictionary.contains(&source[index + new_index]) {
            //     id_name.push(source[index + new_index]);
            //     new_index += 1;
            // }
            // index += new_index;
            // output.push(Token::Id(id_name));
            index += 1;
            ignore_whitespace(&source, &mut index);
            output.push(Token::Id(read_until(&source, &mut index, &|x| {
                !dictionary.contains(&x)
            })));
            continue;
        }

        // if source[index] == '$' {
        //     if source[index + 1] != '"' {
        //         panic!("Text font and size is formatted like so: text(\"Hello World\" $\"sans-serif\" 10)");
        //     }
        //     index += 2;
        //     output.push(Token::Str(read_until(&source, &mut index, &|x| {
        //         !dictionary.contains(&x)
        //     })));
        //     output.push(Token::Num(
        //         read_until(&source, &mut index, &|x| !numbers.contains(&x))
        //             .replace(" ", "")
        //             .parse::<usize>()
        //             .unwrap(),
        //     ));
        //     continue;
        // }



        if source[index] == '"' {
            if !source[index..].contains(&'"') {
                panic!("Unclosed text at line {}", line_num);
            }
            let mut text_contents = String::new();
            let mut new_index = 1;
            loop {
                if source[index + new_index] == '"' {
                    break;
                }
                if source[index + new_index] == '\\' {
                    let c = match source[index + new_index + 1] {
                        't' => '\t',
                        'n' => '\n',
                        '\\' => '\\',
                        '"' => {
                            if !source[(index + new_index + 2)..].contains(&'"') {
                                panic!("Unclosed text at line {}", line_num);
                            }
                            '"'
                        }
                        _ => panic!(
                            "Found an invalid escape character `{}` at line {}",
                            source[index + new_index + 1],
                            line_num
                        ),
                    };
                    text_contents.push(c);
                    new_index += 2;
                    continue;
                }
                text_contents.push(source[index + new_index]);
                new_index += 1;
            }
            index += new_index + 1;
            output.push(Token::Str(text_contents));
            continue;
        }

        if source[index] == 'd' {
            if source[index + 1] == 'i'
                && source[index + 2] == 'v'
                && !dictionary.contains(&source[index + 3])
            {
                output.push(Token::Div);
                index += 3;
                continue;
            }
        }

        if source[index] == 'l' {
            if source[index + 1] == 'a'
                && source[index + 2] == 'b'
                && source[index + 3] == 'e'
                && source[index + 4] == 'l'
                && !dictionary.contains(&source[index + 5])
            {
                output.push(Token::Label);
                index += 5;
                continue;
            }
        }

        if source[index] == 'i' {
            if source[index + 1] == 'm'
                && source[index + 2] == 'a'
                && source[index + 3] == 'g'
                && source[index + 4] == 'e'
                && !dictionary.contains(&source[index + 5])
            {
                output.push(Token::Image);
                index += 5;
                continue;
            }
        }

        if source[index] == 't' {
            if source[index + 1] == 'e'
                && source[index + 2] == 'x'
                && source[index + 3] == 't'
                && !dictionary.contains(&source[index + 4])
            {
                output.push(Token::Text);
                index += 4;
                continue;
            }
        }

        if numbers.contains(&source[index]){
            output.push(Token::Num(read_until(&source, &mut index, &|x| {!numbers.contains(&x)}).parse::<usize>().unwrap()));
            continue;
        }

        println!(
            "End of loop: Current char number {:?}, previous char: {:?}, current char: {:?}, next char: {:?}",
            source[index].to_string().as_bytes(),
            source[index - 1],
            source[index],
            source[index + 1]
        );
        panic!("Unexpectedly reached end of loop!");
    }

    output
}
