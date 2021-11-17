enum Value {
    String(String),
    Integer(i64),
}

#[derive(Debug)]
enum Instruction {
    Print,
    Add,
    Concat,
}

#[derive(Debug)]
enum Token {
    LParen,
    RParen,
    Instruction(Instruction),
    Integer(i64),
    String(String),
}

#[derive(Debug)]
enum Error {
    EvaluationError,
    UnknownToken,
    MismatchedTypes,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn evaluate(mut tokens: Vec<Token>) -> Result<(), Error> {
    let mut stack = vec![];
    while let Some(tok) = tokens.pop() {
        match tok {
            Token::LParen | Token::RParen => (),
            Token::Integer(i) => stack.push(Value::Integer(i)),
            Token::String(s) => stack.push(Value::String(s)),

            Token::Instruction(i) => {
                match i {
                    Instruction::Print => {
                        let p = stack.pop();
                        match p {
                            Some(Value::Integer(int)) => println!("{}", int),
                            Some(Value::String(string)) => println!("{}", string),
                            None => return Err(Error::EvaluationError),
                        }
                    }

                    Instruction::Add => {
                        let rhs = match stack.pop().unwrap() {
                            Value::Integer(int) => int,
                            _ => return Err(Error::MismatchedTypes),
                        };
                        let lhs = match stack.pop().unwrap() {
                            Value::Integer(int) => int,
                            _ => return Err(Error::MismatchedTypes),                            
                        };
                        stack.push(Value::Integer(rhs + lhs));
                    }
                    
                    Instruction::Concat => {
                        let rhs = match stack.pop().unwrap() {
                            Value::String(string) => string,
                            _ => return Err(Error::MismatchedTypes),                          
                        };
                        let lhs = match stack.pop().unwrap() {
                            Value::String(string) => string,
                            _ => return Err(Error::MismatchedTypes),                          
                        };
                        stack.push(Value::String(rhs + &lhs));
                    }
                }
            }
        }
    }

    Ok(())
}

fn tokenize(code: &str) -> Result<Vec<Token>, Error> {
    let mut tokens: Vec<Token> = vec![];
    let mut src = code.chars().peekable();

    while src.size_hint().0 > 0 {
        match src.next().unwrap() {
            // Only used for readability
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),

            w if w.is_ascii_whitespace() => (),

            k if k.is_ascii_alphabetic() => {
                let mut id = String::from(k);
                while let Some(l) = src.peek() {
                    if l.is_ascii_alphabetic() {
                        id.push(*l);
                        src.next();
                    } else { break; }
                }

                match id.as_str() {
                    "print" => tokens.push(Token::Instruction(Instruction::Print)),
                    "add" => tokens.push(Token::Instruction(Instruction::Add)),
                    "concat" => tokens.push(Token::Instruction(Instruction::Concat)),
                    _ => return Err(Error::UnknownToken)
                }
            }

            n if n.is_ascii_digit() => {
                let mut num = String::from(n);
                while let Some(d) = src.peek() {
                    if d.is_ascii_digit() {
                        num.push(*d);
                        src.next();
                    } else { break; }
                }
                tokens.push(Token::Integer(num.parse::<i64>().unwrap()));
            }

            '"' => {
                let mut string = String::new();
                while let Some(s) = src.peek() {
                    if *s == '"' { break; }
                    string.push(*s);
                    src.next();
                }
                src.next();
                tokens.push(Token::String(string));
            }

            _ => return Err(Error::UnknownToken)
        }
    }

    Ok(tokens)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code = "
        print
            add
                5
                add
                    10
                    10

        print
            concat
                \"Hello, \"
                \"world!\"
    ";
    let tokens = tokenize(&code)?;

    evaluate(tokens)?;

    Ok(())
}
