#[derive(Debug, PartialEq)]
enum Token {
    Var(String),
    Number(i32),
    Operator(char),
    ParenthesisOpen,
    ParenthesisClose,
}

#[derive(Debug, PartialEq)]
enum Operands {
    Var(String),
    Number(i32),
    Expr(Box<Expr>)
}

#[derive(Debug, PartialEq)]
struct Expr {
    operator: char,
    operands: [Operands; 2]
}

fn parse_int(input: &str) -> i32 {
    let mut i = 0;
    let chars = input.chars();
    let len = chars.count();
    let mut num = 0;

    for c in input.chars() {
        if(c.is_digit(10)) {
            let digit = c as i32 - '0' as i32;
            num += digit * i32::pow(10, (len - i - 1) as u32);
        }
        i += 1
    }

    num
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    let chars = input.chars();
    let mut temp_num = String::new();

    for char in chars {
        if (char.is_digit(10)) {
            temp_num.push(char);
            continue;
        } else if (temp_num.len() > 0) {
            tokens.push(Token::Number(parse_int(temp_num.clone().as_str())));
            temp_num.clear();
        }

        match char {
            '+' | '-' | '*' | '/' => tokens.push(Token::Operator(char)),
            'a'..='z' | 'A'..='Z' => tokens.push(Token::Var(char.to_string())),
            '(' => {
                tokens.push(Token::ParenthesisOpen)
            },
            ')' => {
                tokens.push(Token::ParenthesisClose)
            },
            ' '  => (),
            _ => panic!("Invalid character: {}", char)
        }
    }

    if (temp_num.len() > 0) {
        tokens.push(Token::Number(parse_int(temp_num.clone().as_str())));
    }

    tokens
}

fn precedence(op: &char) -> i32 {
    match op {
        '(' => 0,
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => panic!("Invalid operator: {}", op)
    }
}

fn merge_operands_to_expr(opStack: & mut Vec<char>, operandStack: & mut Vec<Operands>) {
    let b = operandStack.pop().unwrap();
    let a = operandStack.pop().unwrap();
    let expr = Expr {
        operator: opStack.pop().unwrap(),
        operands: [a, b]
    };
    operandStack.push(Operands::Expr(Box::new(expr)));
}

fn parse(input: &str) -> Option<Expr> {
    let tokens = tokenize(input);

    let mut opStack: Vec<char> = vec![];
    let mut operandStack: Vec<Operands> = vec![];

    for token in tokens {
        match token {
            Token::Var(var) => operandStack.push(Operands::Var(var)),
            Token::Number(num) => operandStack.push(Operands::Number(num)),
            Token::ParenthesisOpen => {
                opStack.push('(')
            },
            Token::ParenthesisClose => {
                while opStack.last() != Some(&'(') {
                    merge_operands_to_expr(&mut opStack, &mut operandStack);
                }
               opStack.pop();
            },
            Token::Operator(op)  => {
                if opStack.is_empty() || precedence(&op) > precedence(opStack.last().unwrap()) {
                    opStack.push(op)
                } else {
                    merge_operands_to_expr(&mut opStack, &mut operandStack);
                    opStack.push(op);
                }
            },
            _ => {}
        }
    }

    while !opStack.is_empty() && operandStack.len() > 1 {
        merge_operands_to_expr(&mut opStack, &mut operandStack);
    }

    match operandStack.pop().unwrap() {
        Operands::Expr(expr) => Some(*expr),
        _ => None
    }
}

fn main() {
    assert_eq!(parse_int("123"), 123);

    assert_eq!(tokenize("a + b * c"), vec![Token::Var("a".to_string()), Token::Operator('+'), Token::Var("b".to_string()), Token::Operator('*'), Token::Var("c".to_string())]);
    assert_eq!(tokenize("a * (b + c)"), vec![Token::Var("a".to_string()), Token::Operator('*'), Token::ParenthesisOpen, Token::Var("b".to_string()), Token::Operator('+'), Token::Var("c".to_string()), Token::ParenthesisClose]);
    assert_eq!(tokenize("a * (b + 35)"), vec![Token::Var("a".to_string()), Token::Operator('*'), Token::ParenthesisOpen, Token::Var("b".to_string()), Token::Operator('+'), Token::Number(35), Token::ParenthesisClose]);
    assert_eq!(tokenize("(a + b) / 2 + 100"), vec![Token::ParenthesisOpen, Token::Var("a".to_string()), Token::Operator('+'), Token::Var("b".to_string()), Token::ParenthesisClose, Token::Operator('/'), Token::Number(2), Token::Operator('+'), Token::Number(100)]);
    assert_eq!(tokenize("a + b * c / 2 - 100"), vec![Token::Var("a".to_string()), Token::Operator('+'), Token::Var("b".to_string()), Token::Operator('*'), Token::Var("c".to_string()), Token::Operator('/'), Token::Number(2), Token::Operator('-'), Token::Number(100)]);

    assert_eq!(parse("a + b * c"), Some(Expr {
        operator: '+',
        operands: [Operands::Var("a".to_string()), Operands::Expr(Box::new(Expr {
            operator: '*',
            operands: [Operands::Var("b".to_string()), Operands::Var("c".to_string())]
        }))]
    }));

    assert_eq!(parse("a * (b + 35)"), Some(Expr {
        operator: '*',
        operands: [Operands::Var("a".to_string()), Operands::Expr(Box::new(Expr {
            operator: '+',
            operands: [Operands::Var("b".to_string()), Operands::Number(35)]
        }))]
    }));

    assert_eq!(parse("a * b + c * d"), Some(Expr {
        operator: '+',
        operands: [Operands::Expr(Box::new(Expr {
            operator: '*',
            operands: [Operands::Var("a".to_string()), Operands::Var("b".to_string())]
        })), Operands::Expr(Box::new(Expr {
            operator: '*',
            operands: [Operands::Var("c".to_string()), Operands::Var("d".to_string())]
        }))]
    }));

    assert_eq!(parse("(a + b) / 2 + 100"), Some(Expr {
        operator: '+',
        operands: [Operands::Expr(Box::new(Expr {
            operator: '/',
            operands: [Operands::Expr(Box::new(Expr {
                operator: '+',
                operands: [Operands::Var("a".to_string()), Operands::Var("b".to_string())]
            })), Operands::Number(2)]
        })), Operands::Number(100)]
    }));

    assert_eq!(parse("a + b * c / 2 - 100"), Some(Expr {
        operator: '+',
        operands: [Operands::Var('a'.to_string()), Operands::Expr(Box::new(Expr {
            operator: '-',
            operands: [Operands::Expr(Box::new(Expr {
                operator: '/',
                operands: [Operands::Expr(Box::new(Expr {
                    operator: '*',
                    operands: [Operands::Var("b".to_string()), Operands::Var("c".to_string())]
                })), Operands::Number(2)]
            })), Operands::Number(100)]
        }))]
    }));

    println!("Все тесты прошли!")
}
