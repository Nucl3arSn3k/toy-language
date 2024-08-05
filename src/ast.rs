use crate::token::Token;
use crate::token::TokenType;
use crate::token::Type;

#[derive(Debug, Clone)]
pub enum ASTNode { //Enum with node types
    Program(Vec<ASTNode>),
    VariableDeclaration {
        identifier: String,
        initializer: Option<Box<ASTNode>>,
        line: u32,
        var_type: Option<Type>
    },
    DisplayStatement(String, u32),
    DisplayIntStatement(String, u32),
    DisplayStringStatement(String, u32),
    ExpressionStatement {
        expression: Box<ASTNode>,
        identifier: String,
        line: u32,
    },
    BinaryOp {
        operator: String,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
        line: u32,
    },
    Identifier(String, u32),
    Number(String, u32),
    StringLiteral(String, u32),
}

impl ASTNode {
    pub fn traverse<F>(&self, f: &F)
    where
        F: Fn(&ASTNode),
    {
        f(self);
        match self {
            ASTNode::Program(statements) => {
                for statement in statements {
                    statement.traverse(f);
                }
            }
            ASTNode::VariableDeclaration { initializer, .. } => {
                if let Some(init) = initializer {
                    init.traverse(f);
                }
            }
            ASTNode::ExpressionStatement { expression, .. } => {
                expression.traverse(f);
            }
            ASTNode::BinaryOp { left, right, .. } => {
                left.traverse(f);
                right.traverse(f);
            }
            _ => {}
        }
    }
}
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<ASTNode, String> {
        self.program()
    }

    fn program(&mut self) -> Result<ASTNode, String> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.statement()?);
        }
        Ok(ASTNode::Program(statements))
    }

    fn statement(&mut self) -> Result<ASTNode, String> {
        if self.match_token(&[TokenType::int_var]) {
            self.int_variable_declaration()
        }
        else if self.match_token(&[TokenType::str_var]) {
            self.str_variable_declaration()
        }
        else if self.match_token(&[TokenType::Display]) {
            self.display_statement()
        }
        else if self.match_token(&[TokenType::DisplayInt]) {
            self.display_int()
        }
        else if self.match_token(&[TokenType::DisplayStr]){

            self.display_string()
        }
         else {
            self.expression_statement()
        }
    }

    fn int_variable_declaration(&mut self) -> Result<ASTNode, String> {
        let var_token = self.previous().clone();
        let identifier = self.consume(&TokenType::Identifier, "Expected identifier.")?;

        let initializer = if self.match_token(&[TokenType::Equals]) {
            Some(Box::new(self.expression()?))
        } else {
            None
        };

        self.consume(
            &TokenType::Semicolon,
            "Expected ';' after variable declaration.",
        )?;

        Ok(ASTNode::VariableDeclaration {
            identifier: identifier.lexeme.clone(),
            initializer,
            var_type:Some(Type::Int),
            line: var_token.line,
        })
    }

    fn str_variable_declaration(&mut self) -> Result<ASTNode, String> {
        let var_token = self.previous().clone();
        let identifier = self.consume(&TokenType::Identifier, "Expected identifier.")?;

        let initializer = if self.match_token(&[TokenType::Equals]) {
            Some(Box::new(self.expression()?))
        } else {
            None
        };

        self.consume(
            &TokenType::Semicolon,
            "Expected ';' after variable declaration.",
        )?;

        Ok(ASTNode::VariableDeclaration {
            identifier: identifier.lexeme.clone(),
            initializer,
            var_type:Some(Type::Str),
            line: var_token.line,
        })
    }

    fn display_statement(&mut self) -> Result<ASTNode, String> {
        let display_token = self.previous().clone();
        let identifier = self.consume(&TokenType::Identifier, "Expected identifier.")?;
        self.consume(
            &TokenType::Semicolon,
            "Expected ';' after display statement.",
        )?;
        Ok(ASTNode::DisplayStatement(
            identifier.lexeme.clone(),
            display_token.line,
        ))
    }

    fn display_int(&mut self) -> Result<ASTNode, String>{
        let display_token = self.previous().clone();
        let identifier = self.consume(&TokenType::Identifier, "Expected identifier.")?;
        self.consume(
            &TokenType::Semicolon,
            "Expected ';' after display statement.",
        )?;
        Ok(ASTNode::DisplayStatement(
            identifier.lexeme.clone(),
            display_token.line,
        ))


    }


    fn display_string(&mut self) -> Result<ASTNode,String>{
        let display_token = self.previous().clone();
        let identifier = self.consume(&TokenType::Identifier, "Expected identifier.")?;
        self.consume(
            &TokenType::Semicolon,
            "Expected ';' after display statement.",
        )?;
        Ok(ASTNode::DisplayStatement(
            identifier.lexeme.clone(),
            display_token.line,
        ))



    }

    fn expression_statement(&mut self) -> Result<ASTNode, String> {
        let identifier = self.consume(&TokenType::Identifier, "Expected identifier.")?;
        let equals_token = self.consume(&TokenType::Equals, "Expected '='.")?;
        let expr = self.expression()?;
        self.consume(
            &TokenType::Semicolon,
            "Expected ';' after expression statement.",
        )?;
        Ok(ASTNode::ExpressionStatement {
            expression: Box::new(expr),
            identifier: identifier.lexeme.clone(),
            line: equals_token.line,
        })
    }

    fn expression(&mut self) -> Result<ASTNode, String> {
        self.term()
    }

    fn term(&mut self) -> Result<ASTNode, String> {
        let mut expr = self.factor()?;

        while self.match_token(&[
            TokenType::Plus,
            TokenType::Minus,
            TokenType::Star,
            TokenType::Slash,
        ]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = ASTNode::BinaryOp {
                operator: operator.lexeme,
                left: Box::new(expr),
                right: Box::new(right),
                line: operator.line,
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<ASTNode, String> {
        if self.match_token(&[TokenType::Number]) {
            let token = self.previous().clone();
            Ok(ASTNode::Number(token.lexeme, token.line))
        } else if self.match_token(&[TokenType::Identifier]) {
            let token = self.previous().clone();
            Ok(ASTNode::Identifier(token.lexeme, token.line))
        } else if self.match_token(&[TokenType::String]) {
            let token = self.previous().clone();
            Ok(ASTNode::StringLiteral(token.lexeme, token.line))
        } else if self.match_token(&[TokenType::LParen]) {
            let expr = self.expression()?;
            self.consume(&TokenType::RParen, "Expected ')' after expression.")?;
            Ok(expr)
        } else {
            Err(format!(
                "Unexpected token in factor at line {}.",
                self.peek().line
            ))
        }
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        &self.peek().token_type == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<Token, String> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(format!("{} at line {}.", message, self.peek().line))
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if let TokenType::Semicolon = self.previous().token_type {
                return;
            }

            match self.peek().token_type {
                TokenType::Var | TokenType::Display => return,
                _ => {
                    self.advance();
                }
            }
        }
    }
}
