// PARSER - Converts tokens to AST
// Builds an Abstract Syntax Tree from tokenized input

use crate::core::{ast::*, lexer::{Token, TokenType, Lexer}};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Result<ASTNode, String> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            match self.parse_statement() {
                Ok(stmt) => statements.push(Box::new(stmt)),
                Err(e) => return Err(e),
            }
        }

        Ok(ASTNode::Program(statements))
    }

    fn parse_statement(&mut self) -> Result<ASTNode, String> {
        // Skip comments and empty lines
        self.skip_semicolons();

        if self.is_at_end() {
            return Ok(ASTNode::Block(Vec::new()));
        }

        let token_type = &self.peek().token_type;

        match token_type {
            TokenType::Let | TokenType::Const | TokenType::Var => self.parse_var_declaration(),
            TokenType::Function => self.parse_function_def(),
            TokenType::If => self.parse_if_statement(),
            TokenType::While => self.parse_while_loop(),
            TokenType::For => self.parse_for_loop(),
            TokenType::Return => self.parse_return_statement(),
            TokenType::LeftBrace => self.parse_block(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_var_declaration(&mut self) -> Result<ASTNode, String> {
        self.advance(); // consume let/const/var

        let name = self.parse_identifier()?;
        
        let data_type = if self.match_token(&TokenType::Colon) {
            self.parse_type()?
        } else {
            DataType::Any
        };

        let value = if self.match_token(&TokenType::Equal) {
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };

        self.skip_semicolons();

        Ok(ASTNode::VarDeclaration {
            name,
            data_type,
            value,
        })
    }

    fn parse_function_def(&mut self) -> Result<ASTNode, String> {
        self.advance(); // consume 'function'

        let name = self.parse_identifier()?;

        self.expect(&TokenType::LeftParen)?;
        let params = self.parse_parameters()?;
        self.expect(&TokenType::RightParen)?;

        let return_type = if self.match_token(&TokenType::Arrow) {
            self.parse_type()?
        } else {
            DataType::Void
        };

        let body = Box::new(self.parse_block()?);

        Ok(ASTNode::FunctionDef {
            name,
            params,
            return_type,
            body,
        })
    }

    fn parse_parameters(&mut self) -> Result<Vec<Parameter>, String> {
        let mut params = Vec::new();

        if self.peek().token_type != TokenType::RightParen {
            loop {
                let name = self.parse_identifier()?;
                let data_type = if self.match_token(&TokenType::Colon) {
                    self.parse_type()?
                } else {
                    DataType::Any
                };

                params.push(Parameter { name, data_type });

                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
        }

        Ok(params)
    }

    fn parse_if_statement(&mut self) -> Result<ASTNode, String> {
        self.advance(); // consume 'if'

        let condition = Box::new(self.parse_expression()?);
        let then_branch = Box::new(self.parse_statement()?);

        let else_branch = if self.match_token(&TokenType::Else) {
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };

        Ok(ASTNode::IfStatement {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn parse_while_loop(&mut self) -> Result<ASTNode, String> {
        self.advance(); // consume 'while'

        let condition = Box::new(self.parse_expression()?);
        let body = Box::new(self.parse_statement()?);

        Ok(ASTNode::WhileLoop { condition, body })
    }

    fn parse_for_loop(&mut self) -> Result<ASTNode, String> {
        self.advance(); // consume 'for'

        let variable = self.parse_identifier()?;
        self.expect(&TokenType::In)?;
        let iterable = Box::new(self.parse_expression()?);

        let body = Box::new(self.parse_statement()?);

        Ok(ASTNode::ForLoop {
            variable,
            iterable,
            body,
        })
    }

    fn parse_return_statement(&mut self) -> Result<ASTNode, String> {
        self.advance(); // consume 'return'

        let value = if self.peek().token_type == TokenType::Semicolon 
            || self.peek().token_type == TokenType::RightBrace
            || self.is_at_end() {
            None
        } else {
            Some(Box::new(self.parse_expression()?))
        };

        self.skip_semicolons();
        Ok(ASTNode::ReturnStatement(value))
    }

    fn parse_block(&mut self) -> Result<ASTNode, String> {
        self.expect(&TokenType::LeftBrace)?;

        let mut statements = Vec::new();

        while self.peek().token_type != TokenType::RightBrace && !self.is_at_end() {
            self.skip_semicolons();
            if self.peek().token_type != TokenType::RightBrace {
                statements.push(Box::new(self.parse_statement()?));
            }
        }

        self.expect(&TokenType::RightBrace)?;

        Ok(ASTNode::Block(statements))
    }

    fn parse_expression_statement(&mut self) -> Result<ASTNode, String> {
        let expr = self.parse_expression()?;
        self.skip_semicolons();
        Ok(expr)
    }

    fn parse_expression(&mut self) -> Result<ASTNode, String> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<ASTNode, String> {
        let expr = self.parse_or()?;

        if self.match_token(&TokenType::Equal) {
            if let ASTNode::Identifier(name) = expr {
                let value = Box::new(self.parse_assignment()?);
                return Ok(ASTNode::Assignment {
                    target: name,
                    value,
                });
            }
        }

        Ok(expr)
    }

    fn parse_or(&mut self) -> Result<ASTNode, String> {
        let mut expr = self.parse_and()?;

        while self.match_token(&TokenType::PipePipe) {
            let right = Box::new(self.parse_and()?);
            expr = ASTNode::BinaryOp {
                op: BinaryOperator::Or,
                left: Box::new(expr),
                right,
            };
        }

        Ok(expr)
    }

    fn parse_and(&mut self) -> Result<ASTNode, String> {
        let mut expr = self.parse_bitwise_or()?;

        while self.match_token(&TokenType::AmpAmp) {
            let right = Box::new(self.parse_bitwise_or()?);
            expr = ASTNode::BinaryOp {
                op: BinaryOperator::And,
                left: Box::new(expr),
                right,
            };
        }

        Ok(expr)
    }

    fn parse_bitwise_or(&mut self) -> Result<ASTNode, String> {
        let mut expr = self.parse_bitwise_xor()?;

        while self.match_token(&TokenType::Pipe) {
            let right = Box::new(self.parse_bitwise_xor()?);
            expr = ASTNode::BinaryOp {
                op: BinaryOperator::BitwiseOr,
                left: Box::new(expr),
                right,
            };
        }

        Ok(expr)
    }

    fn parse_bitwise_xor(&mut self) -> Result<ASTNode, String> {
        let mut expr = self.parse_bitwise_and()?;

        while self.match_token(&TokenType::Caret) {
            let right = Box::new(self.parse_bitwise_and()?);
            expr = ASTNode::BinaryOp {
                op: BinaryOperator::BitwiseXor,
                left: Box::new(expr),
                right,
            };
        }

        Ok(expr)
    }

    fn parse_bitwise_and(&mut self) -> Result<ASTNode, String> {
        let mut expr = self.parse_equality()?;

        while self.match_token(&TokenType::Amp) {
            let right = Box::new(self.parse_equality()?);
            expr = ASTNode::BinaryOp {
                op: BinaryOperator::BitwiseAnd,
                left: Box::new(expr),
                right,
            };
        }

        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<ASTNode, String> {
        let mut expr = self.parse_comparison()?;

        while let Some(op) = self.match_binary_op(&[TokenType::EqualEqual, TokenType::NotEqual]) {
            let right = Box::new(self.parse_comparison()?);
            expr = ASTNode::BinaryOp {
                op,
                left: Box::new(expr),
                right,
            };
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<ASTNode, String> {
        let mut expr = self.parse_additive()?;

        while let Some(op) = self.match_binary_op(&[
            TokenType::Greater, TokenType::GreaterEqual,
            TokenType::Less, TokenType::LessEqual
        ]) {
            let right = Box::new(self.parse_additive()?);
            expr = ASTNode::BinaryOp {
                op,
                left: Box::new(expr),
                right,
            };
        }

        Ok(expr)
    }

    fn parse_additive(&mut self) -> Result<ASTNode, String> {
        let mut expr = self.parse_multiplicative()?;

        while let Some(op) = self.match_binary_op(&[TokenType::Plus, TokenType::Minus]) {
            let right = Box::new(self.parse_multiplicative()?);
            expr = ASTNode::BinaryOp {
                op,
                left: Box::new(expr),
                right,
            };
        }

        Ok(expr)
    }

    fn parse_multiplicative(&mut self) -> Result<ASTNode, String> {
        let mut expr = self.parse_unary()?;

        while let Some(op) = self.match_binary_op(&[TokenType::Star, TokenType::Slash, TokenType::Percent]) {
            let right = Box::new(self.parse_unary()?);
            expr = ASTNode::BinaryOp {
                op,
                left: Box::new(expr),
                right,
            };
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<ASTNode, String> {
        if let Some(op) = self.match_unary_op(&[TokenType::Bang, TokenType::Minus, TokenType::Tilde]) {
            let operand = Box::new(self.parse_unary()?);
            return Ok(ASTNode::UnaryOp { op, operand });
        }

        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Result<ASTNode, String> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.match_token(&TokenType::LeftBracket) {
                let index = Box::new(self.parse_expression()?);
                self.expect(&TokenType::RightBracket)?;
                expr = ASTNode::ArrayAccess {
                    array: Box::new(expr),
                    index,
                };
            } else if self.match_token(&TokenType::Dot) {
                let method = self.parse_identifier()?;
                if self.match_token(&TokenType::LeftParen) {
                    let args = self.parse_arguments()?;
                    self.expect(&TokenType::RightParen)?;
                    expr = ASTNode::MethodCall {
                        object: Box::new(expr),
                        method,
                        args,
                    };
                }
            } else if self.match_token(&TokenType::LeftParen) {
                if let ASTNode::Identifier(name) = expr {
                    let args = self.parse_arguments()?;
                    self.expect(&TokenType::RightParen)?;
                    expr = ASTNode::FunctionCall { name, args };
                } else {
                    return Err("Cannot call non-identifier".to_string());
                }
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<ASTNode, String> {
        let token_type = &self.peek().token_type.clone();

        match token_type {
            TokenType::Integer(n) => {
                let val = *n;
                self.advance();
                Ok(ASTNode::IntegerLiteral(val))
            }
            TokenType::Float(f) => {
                let val = *f;
                self.advance();
                Ok(ASTNode::FloatLiteral(val))
            }
            TokenType::String(s) => {
                let val = s.clone();
                self.advance();
                Ok(ASTNode::StringLiteral(val))
            }
            TokenType::True => {
                self.advance();
                Ok(ASTNode::BooleanLiteral(true))
            }
            TokenType::False => {
                self.advance();
                Ok(ASTNode::BooleanLiteral(false))
            }
            TokenType::Null => {
                self.advance();
                Ok(ASTNode::NullLiteral)
            }
            TokenType::Identifier(_) => {
                let ident = self.parse_identifier()?;
                Ok(ASTNode::Identifier(ident))
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(&TokenType::RightParen)?;
                Ok(expr)
            }
            TokenType::LeftBracket => {
                self.advance();
                let mut elements = Vec::new();
                while self.peek().token_type != TokenType::RightBracket {
                    elements.push(Box::new(self.parse_expression()?));
                    if !self.match_token(&TokenType::Comma) {
                        break;
                    }
                }
                self.expect(&TokenType::RightBracket)?;
                Ok(ASTNode::ArrayLiteral(elements))
            }
            _ => Err(format!("Unexpected token: {:?}", token_type)),
        }
    }

    fn parse_arguments(&mut self) -> Result<Vec<Box<ASTNode>>, String> {
        let mut args = Vec::new();

        if self.peek().token_type != TokenType::RightParen {
            loop {
                args.push(Box::new(self.parse_expression()?));
                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
        }

        Ok(args)
    }

    fn parse_identifier(&mut self) -> Result<String, String> {
        if let TokenType::Identifier(name) = &self.peek().token_type {
            let result = name.clone();
            self.advance();
            Ok(result)
        } else {
            Err(format!("Expected identifier, got {:?}", self.peek().token_type))
        }
    }

    fn parse_type(&mut self) -> Result<DataType, String> {
        match &self.peek().token_type {
            TokenType::Int => { self.advance(); Ok(DataType::Integer) }
            TokenType::Float_KW => { self.advance(); Ok(DataType::Float) }
            TokenType::String_KW => { self.advance(); Ok(DataType::String) }
            TokenType::Bool => { self.advance(); Ok(DataType::Boolean) }
            TokenType::Void => { self.advance(); Ok(DataType::Void) }
            _ => { self.advance(); Ok(DataType::Any) }
        }
    }

    fn match_token(&mut self, token_type: &TokenType) -> bool {
        if std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_binary_op(&mut self, token_types: &[TokenType]) -> Option<BinaryOperator> {
        let current = &self.peek().token_type;
        let op = match current {
            TokenType::Plus => Some(BinaryOperator::Add),
            TokenType::Minus => Some(BinaryOperator::Subtract),
            TokenType::Star => Some(BinaryOperator::Multiply),
            TokenType::Slash => Some(BinaryOperator::Divide),
            TokenType::Percent => Some(BinaryOperator::Modulo),
            TokenType::EqualEqual => Some(BinaryOperator::Equal),
            TokenType::NotEqual => Some(BinaryOperator::NotEqual),
            TokenType::Less => Some(BinaryOperator::Less),
            TokenType::LessEqual => Some(BinaryOperator::LessEqual),
            TokenType::Greater => Some(BinaryOperator::Greater),
            TokenType::GreaterEqual => Some(BinaryOperator::GreaterEqual),
            TokenType::AmpAmp => Some(BinaryOperator::And),
            TokenType::PipePipe => Some(BinaryOperator::Or),
            TokenType::Amp => Some(BinaryOperator::BitwiseAnd),
            TokenType::Pipe => Some(BinaryOperator::BitwiseOr),
            TokenType::Caret => Some(BinaryOperator::BitwiseXor),
            _ => None,
        };

        if let Some(o) = op {
            for token_type in token_types {
                if std::mem::discriminant(current) == std::mem::discriminant(token_type) {
                    self.advance();
                    return Some(o);
                }
            }
        }
        None
    }

    fn match_unary_op(&mut self, token_types: &[TokenType]) -> Option<UnaryOperator> {
        let current = &self.peek().token_type;
        let op = match current {
            TokenType::Bang => Some(UnaryOperator::Not),
            TokenType::Minus => Some(UnaryOperator::Negate),
            TokenType::Tilde => Some(UnaryOperator::BitwiseNot),
            _ => None,
        };

        if let Some(o) = op {
            for token_type in token_types {
                if std::mem::discriminant(current) == std::mem::discriminant(token_type) {
                    self.advance();
                    return Some(o);
                }
            }
        }
        None
    }

    fn expect(&mut self, token_type: &TokenType) -> Result<(), String> {
        if std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(token_type) {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, got {:?}", token_type, self.peek().token_type))
        }
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.position += 1;
        }
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.position).cloned()
            .unwrap_or_else(|| Token {
                token_type: TokenType::Eof,
                line: 0,
                column: 0,
            })
    }

    fn skip_semicolons(&mut self) {
        while self.match_token(&TokenType::Semicolon) {}
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }
}

pub fn parse(input: &str) -> Result<ASTNode, String> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    parser.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic() {
        let result = parse("let x = 10;");
        assert!(result.is_ok());
    }
}
