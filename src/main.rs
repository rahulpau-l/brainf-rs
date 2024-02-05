use std::fs;
use std::io;

const MEMORY: usize = 30_000;

#[derive(Debug)]
enum Token {
    IncPointer,
    DecPointer,
    Increment,
    Decrement,
    Output,
    Input,
    SquareOpen,
    SquareClose,
}

#[derive(Debug)]
struct Lexer {
    input: String,
    token_vec: Vec<Token>,
}

impl Lexer {
    fn new(filename: &str) -> Result<Self, String> {
        if !filename.ends_with(".bf") {
            return Err("file extension not recognized".to_string());
        }

        let file_input = fs::read_to_string(filename).unwrap();
        let file_input = file_input.replace(" ", "");

        Ok(Self {
            input: file_input,
            token_vec: Vec::new(),
        })
    }

    fn generate_tokens(&mut self) {
        for char in self.input.chars() {
            match char {
                '>' => self.token_vec.push(Token::IncPointer),
                '<' => self.token_vec.push(Token::DecPointer),
                '+' => self.token_vec.push(Token::Increment),
                '-' => self.token_vec.push(Token::Decrement),
                '.' => self.token_vec.push(Token::Output),
                ',' => self.token_vec.push(Token::Input),
                '[' => self.token_vec.push(Token::SquareOpen),
                ']' => self.token_vec.push(Token::SquareClose),
                '\r' => continue,
                '\n' => continue,
                _ => panic!("Unknown character identified {}", char.is_whitespace()),
            }
        }
    }

    fn get_tokens(self) -> Vec<Token> {
        return self.token_vec;
    }
}

struct Interpreter {
    array: [u8; MEMORY],
    data_pointer: usize,
    instructions: Vec<Token>,
}

impl Interpreter {
    fn new() -> Result<Self, String> {
        let mut l = Lexer::new("hello.bf")?;
        l.generate_tokens();

        Ok(Self {
            array: [0; 30_000],
            data_pointer: 0,
            instructions: l.get_tokens(),
        })
    }

    fn run(&mut self) {
        let mut index: usize = 0;
        let mut index_of_open_bracket = 0;
        let mut index_of_close_bracket = 0;
        let mut skip;

        while index < self.instructions.len() {
            skip = false;
            match self.instructions[index] {
                Token::IncPointer => self.data_pointer += 1,
                Token::DecPointer => self.data_pointer -= 1,
                Token::Increment => self.array[self.data_pointer] += 1,
                Token::Decrement => self.array[self.data_pointer] -= 1,
                Token::Output => {
                    print!("{}", self.array[self.data_pointer] as char)
                }
                Token::Input => {
                    let mut user_char = String::new();
                    io::stdin()
                        .read_line(&mut user_char)
                        .expect("failed to get user input");

                    let bytes = user_char.bytes().nth(0).expect("no byte read");

                    self.array[self.data_pointer] = bytes;
                }
                Token::SquareOpen => {
                    index_of_open_bracket = index;

                    if self.array[self.data_pointer] == 0 {
                        index = index_of_close_bracket + 1;
                        skip = true;
                    }
                }
                Token::SquareClose => {
                    index_of_close_bracket = index;
                    if self.array[self.data_pointer] != 0 {
                        index = index_of_open_bracket;
                        skip = true;
                    }
                }
            }

            if !skip {
                index += 1;
            }
        }
    }

    fn end_of_program(&self) {
        println!(
            "data pointer: {}, array: {:?}",
            self.data_pointer, self.array
        )
    }
}

fn main() -> Result<(), String> {
    let mut inter = Interpreter::new()?;
    inter.run();
    inter.end_of_program();
    Ok(())
}
