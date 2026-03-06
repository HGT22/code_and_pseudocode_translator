// LEXER - Tokenizes source code
// Breaks down source code into tokens that can be parsed

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Integer(i64),
    Float(f64),
    String(String),
    Identifier(String),

    // Keywords
    Let,
    Const,
    Var,
    Function,
    If,
    Else,
    While,
    For,
    In,
    Return,
    Class,
    True,
    False,
    Null,
    Void,
    Int,
    Float_KW,
    String_KW,
    Bool,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Bang,
    AmpAmp,
    PipePipe,
    Amp,
    Pipe,
    Caret,
    Tilde,

    // Punctuation
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Semicolon,
    Comma,
    Dot,
    Colon,
    Arrow,

    // Special
    Newline,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.skip_whitespace_and_comments();
            if self.is_at_end() {
                break;
            }

            match self.next_token() {
                Some(token) => tokens.push(token),
                None => self.advance(),
            }
        }

        tokens.push(Token {
            token_type: TokenType::Eof,
            line: self.line,
            column: self.column,
        });

        tokens
    }

    fn next_token(&mut self) -> Option<Token> {
        let ch = self.peek()?;

        if ch.is_alphabetic() || ch == '_' {
            return Some(self.read_identifier());
        }

        if ch.is_numeric() {
            return Some(self.read_number());
        }

        match ch {
            '"' => Some(self.read_string()),
            '+' => Some(self.single_token(TokenType::Plus)),
            '-' => {
                self.advance();
                if self.peek() == Some('>') {
                    self.advance();
                    Some(Token {
                        token_type: TokenType::Arrow,
                        line: self.line,
                        column: self.column - 2,
                    })
                } else {
                    Some(Token {
                        token_type: TokenType::Minus,
                        line: self.line,
                        column: self.column - 1,
                    })
                }
            }
            '*' => Some(self.single_token(TokenType::Star)),
            '/' => Some(self.single_token(TokenType::Slash)),
            '%' => Some(self.single_token(TokenType::Percent)),
            '=' => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Some(Token {
                        token_type: TokenType::EqualEqual,
                        line: self.line,
                        column: self.column - 2,
                    })
                } else {
                    Some(Token {
                        token_type: TokenType::Equal,
                        line: self.line,
                        column: self.column - 1,
                    })
                }
            }
            '!' => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Some(Token {
                        token_type: TokenType::NotEqual,
                        line: self.line,
                        column: self.column - 2,
                    })
                } else {
                    Some(Token {
                        token_type: TokenType::Bang,
                        line: self.line,
                        column: self.column - 1,
                    })
                }
            }
            '<' => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Some(Token {
                        token_type: TokenType::LessEqual,
                        line: self.line,
                        column: self.column - 2,
                    })
                } else {
                    Some(Token {
                        token_type: TokenType::Less,
                        line: self.line,
                        column: self.column - 1,
                    })
                }
            }
            '>' => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Some(Token {
                        token_type: TokenType::GreaterEqual,
                        line: self.line,
                        column: self.column - 2,
                    })
                } else {
                    Some(Token {
                        token_type: TokenType::Greater,
                        line: self.line,
                        column: self.column - 1,
                    })
                }
            }
            '&' => {
                self.advance();
                if self.peek() == Some('&') {
                    self.advance();
                    Some(Token {
                        token_type: TokenType::AmpAmp,
                        line: self.line,
                        column: self.column - 2,
                    })
                } else {
                    Some(Token {
                        token_type: TokenType::Amp,
                        line: self.line,
                        column: self.column - 1,
                    })
                }
            }
            '|' => {
                self.advance();
                if self.peek() == Some('|') {
                    self.advance();
                    Some(Token {
                        token_type: TokenType::PipePipe,
                        line: self.line,
                        column: self.column - 2,
                    })
                } else {
                    Some(Token {
                        token_type: TokenType::Pipe,
                        line: self.line,
                        column: self.column - 1,
                    })
                }
            }
            '^' => Some(self.single_token(TokenType::Caret)),
            '~' => Some(self.single_token(TokenType::Tilde)),
            '(' => Some(self.single_token(TokenType::LeftParen)),
            ')' => Some(self.single_token(TokenType::RightParen)),
            '{' => Some(self.single_token(TokenType::LeftBrace)),
            '}' => Some(self.single_token(TokenType::RightBrace)),
            '[' => Some(self.single_token(TokenType::LeftBracket)),
            ']' => Some(self.single_token(TokenType::RightBracket)),
            ';' => Some(self.single_token(TokenType::Semicolon)),
            ',' => Some(self.single_token(TokenType::Comma)),
            '.' => Some(self.single_token(TokenType::Dot)),
            ':' => Some(self.single_token(TokenType::Colon)),
            _ => None,
        }
    }

    fn read_identifier(&mut self) -> Token {
        let start_column = self.column;
        let mut ident = String::new();

        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        let token_type = match ident.as_str() {
            "let" => TokenType::Let,
            "const" => TokenType::Const,
            "var" => TokenType::Var,
            "function" | "fn" => TokenType::Function,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "while" => TokenType::While,
            "for" => TokenType::For,
            "in" => TokenType::In,
            "return" => TokenType::Return,
            "class" => TokenType::Class,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "null" | "nil" => TokenType::Null,
            "void" => TokenType::Void,
            "int" => TokenType::Int,
            "float" => TokenType::Float_KW,
            "string" | "str" => TokenType::String_KW,
            "bool" | "boolean" => TokenType::Bool,
            _ => TokenType::Identifier(ident),
        };

        Token {
            token_type,
            line: self.line,
            column: start_column,
        }
    }

    fn read_number(&mut self) -> Token {
        let start_column = self.column;
        let mut number = String::new();
        let mut is_float = false;

        while let Some(ch) = self.peek() {
            if ch.is_numeric() {
                number.push(ch);
                self.advance();
            } else if ch == '.' && !is_float {
                is_float = true;
                number.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        let token_type = if is_float {
            TokenType::Float(number.parse().unwrap_or(0.0))
        } else {
            TokenType::Integer(number.parse().unwrap_or(0))
        };

        Token {
            token_type,
            line: self.line,
            column: start_column,
        }
    }

    fn read_string(&mut self) -> Token {
        let start_column = self.column;
        self.advance(); // skip opening quote
        let mut string = String::new();

        while let Some(ch) = self.peek() {
            if ch == '"' {
                self.advance();
                break;
            }
            if ch == '\\' {
                self.advance();
                if let Some(escaped) = self.peek() {
                    let escaped_char = match escaped {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '"' => '"',
                        '\\' => '\\',
                        _ => escaped,
                    };
                    string.push(escaped_char);
                    self.advance();
                }
            } else {
                string.push(ch);
                self.advance();
            }
        }

        Token {
            token_type: TokenType::String(string),
            line: self.line,
            column: start_column,
        }
    }

    fn single_token(&mut self, token_type: TokenType) -> Token {
        let column = self.column;
        self.advance();
        Token {
            token_type,
            line: self.line,
            column,
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        while let Some(ch) = self.peek() {
            if ch == ' ' || ch == '\t' {
                self.advance();
            } else if ch == '\n' {
                self.advance();
                self.line += 1;
                self.column = 1;
            } else if ch == '/' && self.peek_ahead(1) == Some('/') {
                // Single line comment
                while self.peek() != Some('\n') && !self.is_at_end() {
                    self.advance();
                }
            } else if ch == '/' && self.peek_ahead(1) == Some('*') {
                // Multi-line comment
                self.advance();
                self.advance();
                while !self.is_at_end() {
                    if self.peek() == Some('*') && self.peek_ahead(1) == Some('/') {
                        self.advance();
                        self.advance();
                        break;
                    }
                    if self.peek() == Some('\n') {
                        self.line += 1;
                        self.column = 1;
                    }
                    self.advance();
                }
            } else {
                break;
            }
        }
    }

    fn peek(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }

    fn peek_ahead(&self, n: usize) -> Option<char> {
        if self.position + n < self.input.len() {
            Some(self.input[self.position + n])
        } else {
            None
        }
    }

    fn advance(&mut self) {
        if self.position < self.input.len() {
            self.position += 1;
            self.column += 1;
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_basic() {
        let mut lexer = Lexer::new("let x = 10;");
        let tokens = lexer.tokenize();
        assert!(tokens.len() > 0);
        assert_eq!(tokens[tokens.len() - 1].token_type, TokenType::Eof);
    }
}
