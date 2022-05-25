use std::*;

#[derive(Debug, Clone)]
enum TokenKind {
    TokenWord,
    TokenInt,
    TokenCount
}

#[derive(Debug, Clone)]
struct TokenValue {
    string: String,
    integer: i64
}

#[derive(Debug, Clone)]
struct Token {
    kind:  TokenKind,
    value: TokenValue
}

impl Token {
    fn new() -> Token {
        Token {
            kind:  TokenKind::TokenWord,
            value: TokenValue {
                string:  "".to_string(),
                integer: 0
            }
        }
    }
}

struct Lexer {
    tokens: Vec<Token>,
    token:  Token,
    string: String,
    attok:  usize,
    atstr:  usize
}

impl Lexer {
    fn new() -> Lexer {
        Lexer {
            tokens: vec![],
            token:  Token::new(),
            string: "".to_string(),
            attok:  0,
            atstr:  0
        }
    }

    fn lex_line(&mut self, line: String) {
        self.atstr = 0;
        self.string = line;
        self.string.push(' ');
    }

    fn is_cur_token_digit(&self) -> bool {
        let mut is = false;
        for c in self.token.value.string.chars() {
            is = c.is_numeric();
        }
        is
    }

    fn all_tokens(&self) -> usize {
        self.string.split(' ').collect::<Vec<&str>>().len()-1
    }

    fn next(&mut self) -> Token {
        assert!(self.atstr < self.string.len());

        self.token = Token::new();

        loop {
            if !((self.string.as_bytes()[self.atstr] as char).is_whitespace()) {
                self.token.value.string.push(self.string.as_bytes()[self.atstr] as char);
            }

            if (self.string.as_bytes()[self.atstr] as char).is_whitespace() {
                self.atstr += 1;
                break;
            }
            self.atstr += 1;
        }

        if self.is_cur_token_digit() {
            self.token.value.integer = self.token.value.string.parse::<i64>().unwrap();
            self.token.value.string  = "".to_string();
            self.token.kind          = TokenKind::TokenInt;
        }

        assert!(TokenKind::TokenCount as i64 == 2);

        self.attok += 1;
        self.tokens.push(self.token.clone());
        self.token.clone()
    }
}

////////////////////////////////////////

#[derive(Debug, Clone)]
enum OpType {
    OpPushInt,
    OpPlus,
    OpPrint,
    OpNull,
    OpCount
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Op {
    op: OpType,
    operand: i64
}

impl Op {
    fn new() -> Op {
        Op {
            op:      OpType::OpNull,
            operand: 0
        }
    }
}

////////////////////////////////////////

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Program {
    ops: Vec<Op>
}

#[allow(unreachable_patterns)]
impl Program {
    fn new() -> Program {
        Program {
            ops: vec![]
        }
    }

    fn from_token(&mut self, token: Token) -> Op {
        let mut pushed = Op::new();
        assert!(TokenKind::TokenCount as i64 == 2);
        assert!(OpType::OpCount as i64 == 4);

        match token.kind {
            TokenKind::TokenInt =>
                pushed = Op {op: OpType::OpPushInt, operand: token.value.integer},
            TokenKind::TokenWord => {
                match token.value.string.as_str() {
                    "print" =>
                        pushed = Op {op: OpType::OpPrint, operand: 0},
                    "+"     =>
                        pushed = Op {op: OpType::OpPlus, operand: 0},
                    &_      => {
                        eprintln!("ERROR: Unknown word: `{}`", token.value.string);
                        process::exit(1);
                    }
                }
            },
            _ => assert!(false, "Unreachable")
        }
        self.ops.push(pushed.clone());
        pushed.clone()
    }

    fn simulate(&self) {
        let mut stack: Vec<i64> = vec![];

        assert!(OpType::OpCount as i64 == 4);

        for i in &self.ops {
            match i.op {
                OpType::OpPushInt => stack.push(i.operand),
                OpType::OpPlus    => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(a + b);
                },
                OpType::OpPrint   => {
                    println!("{}", stack.pop().unwrap());
                },
                OpType::OpNull    => {},
                _ => assert!(false, "Unreachable")
            }
        }
    }
}

////////////////////////////////////////

fn compile_tokens_into_ops(lexer: &mut Lexer) -> Program {
    let mut program = Program::new();

    while lexer.attok < lexer.all_tokens() {
        program.from_token(lexer.next());
    }

    program.clone()
}

////////////////////////////////////////

fn main() {
    let mut lexer = Lexer::new();
    lexer.lex_line("34 35 + print".to_string());

    let program = compile_tokens_into_ops(&mut lexer);
    program.simulate();
}
