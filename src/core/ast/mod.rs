// AST (Abstract Syntax Tree) - Universal intermediate representation
// This module defines the universal AST that all languages are converted to

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    Program(Vec<Box<ASTNode>>),
    
    // Variable declarations
    VarDeclaration {
        name: String,
        data_type: DataType,
        value: Option<Box<ASTNode>>,
    },
    
    // Function definition
    FunctionDef {
        name: String,
        params: Vec<Parameter>,
        return_type: DataType,
        body: Box<ASTNode>,
    },
    
    // Expressions
    BinaryOp {
        op: BinaryOperator,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },
    
    UnaryOp {
        op: UnaryOperator,
        operand: Box<ASTNode>,
    },
    
    Assignment {
        target: String,
        value: Box<ASTNode>,
    },
    
    MethodCall {
        object: Box<ASTNode>,
        method: String,
        args: Vec<Box<ASTNode>>,
    },
    
    FunctionCall {
        name: String,
        args: Vec<Box<ASTNode>>,
    },
    
    // Control flow
    IfStatement {
        condition: Box<ASTNode>,
        then_branch: Box<ASTNode>,
        else_branch: Option<Box<ASTNode>>,
    },
    
    WhileLoop {
        condition: Box<ASTNode>,
        body: Box<ASTNode>,
    },
    
    ForLoop {
        variable: String,
        iterable: Box<ASTNode>,
        body: Box<ASTNode>,
    },
    
    Block(Vec<Box<ASTNode>>),
    
    ReturnStatement(Option<Box<ASTNode>>),
    
    // Literals
    IntegerLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BooleanLiteral(bool),
    NullLiteral,
    
    // Variables and identifiers
    Identifier(String),
    
    // Array/List operations
    ArrayLiteral(Vec<Box<ASTNode>>),
    ArrayAccess {
        array: Box<ASTNode>,
        index: Box<ASTNode>,
    },
    
    // Comments
    Comment(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    Integer,
    Float,
    String,
    Boolean,
    Void,
    Array,
    Any,
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataType::Integer => write!(f, "int"),
            DataType::Float => write!(f, "float"),
            DataType::String => write!(f, "string"),
            DataType::Boolean => write!(f, "bool"),
            DataType::Void => write!(f, "void"),
            DataType::Array => write!(f, "array"),
            DataType::Any => write!(f, "any"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Subtract => write!(f, "-"),
            BinaryOperator::Multiply => write!(f, "*"),
            BinaryOperator::Divide => write!(f, "/"),
            BinaryOperator::Modulo => write!(f, "%"),
            BinaryOperator::Equal => write!(f, "=="),
            BinaryOperator::NotEqual => write!(f, "!="),
            BinaryOperator::Less => write!(f, "<"),
            BinaryOperator::LessEqual => write!(f, "<="),
            BinaryOperator::Greater => write!(f, ">"),
            BinaryOperator::GreaterEqual => write!(f, ">="),
            BinaryOperator::And => write!(f, "&&"),
            BinaryOperator::Or => write!(f, "||"),
            BinaryOperator::BitwiseAnd => write!(f, "&"),
            BinaryOperator::BitwiseOr => write!(f, "|"),
            BinaryOperator::BitwiseXor => write!(f, "^"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Not,
    Negate,
    BitwiseNot,
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOperator::Not => write!(f, "!"),
            UnaryOperator::Negate => write!(f, "-"),
            UnaryOperator::BitwiseNot => write!(f, "~"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    pub name: String,
    pub data_type: DataType,
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTNode::Program(nodes) => {
                for node in nodes {
                    writeln!(f, "{}", node)?;
                }
                Ok(())
            }
            ASTNode::IntegerLiteral(n) => write!(f, "{}", n),
            ASTNode::StringLiteral(s) => write!(f, "\"{}\"", s),
            ASTNode::BooleanLiteral(b) => write!(f, "{}", b),
            ASTNode::Identifier(name) => write!(f, "{}", name),
            _ => write!(f, "{:?}", self),
        }
    }
}
