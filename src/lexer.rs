use crate::tokens::Token;
use crate::tokens::Token::*;
use crate::lexer::FailureType::{GenericFailure, EndOfFileFailure};
use std::str;

enum FailureType {
    GenericFailure,
    EndOfFileFailure,
}

/* TODO create peek error */
enum ReadResult {
    Ok,
    Failure(FailureType),
}

struct TokenReader {
    buffer: Vec<u8>,
    index: usize,
    size: usize,
}

impl TokenReader {
    fn from(text: String) -> TokenReader {
        TokenReader { buffer: Vec::from(text.as_bytes()), index: 0, size: text.len() }
    }

    fn read_digit() -> Option<u8> { unimplemented!() }

    fn read_character(&mut self) -> Option<u8> {
        if self.index < self.size {
            Some(self.buffer[self.index])
        } else {
            None
        }
    }

    fn read_left_bracket(&mut self) -> Option<Token> {
        self.read_text("(", LeftBracket)
    }

    fn read_right_bracket(&mut self) -> Option<Token> {
        self.read_text(")", RightBracket)
    }

    fn read_bracket(&mut self) -> Option<Token> {
        self.read_left_bracket()
            .or_else(|| self.read_right_bracket())
    }

    // fn read_text(&mut self, text: String) -> ReadResult { unimplemented!() }

    fn read_chars(&mut self, length: usize) -> &[u8] {
        return &self.buffer[self.index..self.index + length];
    }

    fn read_end_of_file(&mut self) -> Option<Token> {
        if self.index == self.size {
            Some(EndOfFile)
        } else {
            None
        }
    }

    fn read_text<T>(&mut self, text: &str, on_success: T) -> Option<T> {
        if self.index + text.len() <= self.size {
            if self.read_chars(text.len()) == text.as_bytes() {
                self.index += text.len();
                Some(on_success)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn peek_at(&self, index: usize) -> Option<&u8> {
        return self.buffer.get(index);
    }

    fn peek(&self) -> Option<&u8> {
        return self.buffer.get(self.index);
    }

    fn check_peek(&mut self, character: &u8) -> ReadResult {
        if let Some(read) = self.peek() {
            if read == character {
                ReadResult::Ok
            } else {
                ReadResult::Failure(GenericFailure)
            }
        } else {
            ReadResult::Failure(EndOfFileFailure)
        }
    }

    fn check_peek_at(&mut self, index: usize, character: &u8) -> ReadResult {
        if let Some(read) = self.peek_at(index) {
            if read == character {
                ReadResult::Ok
            } else {
                ReadResult::Failure(GenericFailure)
            }
        } else {
            ReadResult::Failure(EndOfFileFailure)
        }
    }

    fn step(&mut self) {
        self.index += 1;
    }

    fn read_until_and_eat(&mut self, character: &u8) -> Option<&[u8]> {
        let mut local_index = self.index;
        loop {
            match self.check_peek_at(local_index, character) {
                ReadResult::Ok => {
                    local_index += 1;
                    break;
                }
                ReadResult::Failure(GenericFailure) => {
                    local_index += 1;
                }
                ReadResult::Failure(EndOfFileFailure) => {
                    return None;
                }
            }
        }
        let return_value = Some(&self.buffer[self.index..local_index - 1]);
        self.index = local_index;
        return return_value;
    }

    fn read_string_literal(&mut self) -> Option<Token> {
        if self.peek() == Some(&b'"') {
            self.step();
            match self.read_until_and_eat(&b'"') {
                None => None,
                Some(bytes) => Some(StringLiteral(String::from_utf8(bytes.to_vec()).expect("TODO"))) // TODO ghlahfuyasdfkas
            }
        } else {
            None
        }
    }

    fn read_while_true(&mut self, condition: fn(buffer: &[u8], character: &u8) -> bool) -> Option<&[u8]> {
        let mut local_index = self.index;
        loop {
            let result = self.peek_at(local_index);
            match result {
                None => {
                    break; // There's nothing here
                }
                Some(character) => {
                    if condition(&self.buffer[self.index..local_index], character) {
                        local_index += 1;
                    } else {
                        break;
                    }
                }
            }
        }

        let local_buffer = &self.buffer[self.index..local_index];
        if local_buffer.len() == 0 {
            None
        } else {
            self.index = local_index;
            Some(local_buffer)
        }
    }

    fn read_hex_literal(&mut self) -> Option<Token> {
        match self.read_text("0x", EndOfFile) {
            None => None,
            Some(_) => match self.read_while_true(|_buffer, character| character.is_ascii_hexdigit()) {
                None => panic!("hsahja SHOULDN'T BE LIKE THIS, 'UNREAD' IN THE CASE OF THIS e.g. 0x("),
                Some(bytes) => Some(HexLiteral(String::from_utf8(bytes.to_vec()).expect("aksahfkahlk")))
            }
        }
    }

    fn read_name(&mut self) -> Option<Token> {
        match self.read_while_true(|_buffer, character| character.is_ascii_alphabetic()) {
            Some(buffer) => Some(Name(String::from_utf8(buffer.to_vec()).expect("TODO"))),
            None => None
        }
    }

    fn read_white_space(&mut self) -> Option<Token> {
        self.read_while_true(|_buffer, character| character.is_ascii_whitespace())
            .and_then(|_| Some(Whitespace))
    }

    fn read_token(&mut self) -> Option<Token> {
        self.read_string_literal()
            .or_else(|| self.read_hex_literal())
            .or_else(|| self.read_name())
            .or_else(|| self.read_bracket())
            .or_else(|| self.read_end_of_file())
            .or_else(|| self.read_white_space())
            .or(None)
    }
}


pub fn tokenize(text: String) -> Vec<Token> {
    let mut token_reader = TokenReader::from(text);
    let mut tokens: Vec<Token> = vec!();
    loop {
        match token_reader.read_token() {
            Some(Token::EndOfFile) => {
                tokens.push(EndOfFile);
                break;
            }
            Some(token) => tokens.push(token),
            None => panic!("Crying because I wasn't able to read a token {:?}", tokens)
        }
    }
    return tokens;
}