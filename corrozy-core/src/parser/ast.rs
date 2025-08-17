

#[derive(Debug, Clone)]
pub enum AstNode {
    Program { statements: Vec<AstNode> },
    ExpressionStatement {
        expression: Box<Expression>
    },
    VariableDeclaration {
        var_type: Option<String>,
        name: String, 
        value: Box<Expression> 
    },
    ConstantDeclaration {
        name: String,
        const_type: Option<String>,
        value: Box<Expression>
    },
    PrintStatement { 
        expression: Box<Expression>,
        newline: bool
    },
    FunctionDeclaration { 
        name: String, 
        params: Vec<Parameter>, 
        return_type: Option<String>,
        body: Block
    },
    IfStatement {
        condition: Box<Expression>,
        then_block: Block,
        else_clause: Option<Box<ElseClause>>
    },
    WhileLoop {
        condition: Box<Expression>,
        body: Block
    },
    ForLoop {
        init: Option<Box<ForInit>>,
        condition: Option<Box<Expression>>,
        update: Option<Box<Expression>>,
        body: Block
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Variable(String),
    FunctionCall { 
        name: String, 
        args: Vec<Expression> 
    },
    BinaryOp { 
        left: Box<Expression>, 
        op: BinaryOperator, 
        right: Box<Expression> 
    },
    Parenthesized(Box<Expression>),
    Closure {
        params: Vec<Parameter>,
        return_type: Option<String>,
        body: Block
    },
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub param_type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub expression: Option<Box<Expression>>,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<AstNode>,
    pub return_statement: Option<ReturnStatement>,
}

impl Block {
    pub fn new() -> Self {
        Block {
            statements: Vec::new(),
            return_statement: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ElseClause {
    ElseIf(Box<AstNode>), 
    Else(Block),
}

#[derive(Debug, Clone)]
pub enum ForInit {
    VariableDeclaration(Box<AstNode>),
    Expression(Box<Expression>),
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    // Arithmetic
    Add,          // +
    Subtract,     // -
    Multiply,     // *
    Divide,       // /
    
    // Comparison
    Equal,        // ==
    NotEqual,     // !=
    Less,         // 
    Greater,      // >
    LessEqual,    // <=
    GreaterEqual, // >=
    
    // Logical
    And,          // &&
    Or,           // ||
}

impl BinaryOperator {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "+" => Some(Self::Add),
            "-" => Some(Self::Subtract),
            "*" => Some(Self::Multiply),
            "/" => Some(Self::Divide),
            "==" => Some(Self::Equal),
            "!=" => Some(Self::NotEqual),
            "<" => Some(Self::Less),
            ">" => Some(Self::Greater),
            "<=" => Some(Self::LessEqual),
            ">=" => Some(Self::GreaterEqual),
            "&&" => Some(Self::And),
            "||" => Some(Self::Or),
            _ => None,
        }
    }
    
    pub fn to_php(&self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Subtract => "-",
            Self::Multiply => "*",
            Self::Divide => "/",
            Self::Equal => "==",
            Self::NotEqual => "!=",
            Self::Less => "<",
            Self::Greater => ">",
            Self::LessEqual => "<=",
            Self::GreaterEqual => ">=",
            Self::And => "&&",
            Self::Or => "||",
        }
    }
}

#[derive(Debug, Clone)]
pub enum StringType {
    Interpolated(String),  // Para "content"
    Raw(String),          // Para 'content'
}

#[derive(Debug, Clone)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(StringType),
    Boolean(bool),
}

impl Literal {
    pub fn to_php(&self) -> String {
        match self {
            Self::Integer(n) => n.to_string(),
            Self::Float(f) => f.to_string(),
            Self::String(s) => s.to_php(),
            Self::Boolean(b) => (if *b { "true" } else { "false" }).to_string(),
        }
    }
}

impl StringType {
    pub fn to_php(&self) -> String {
        match self {
            Self::Interpolated(content) => format!("\"{}\"", content),
            Self::Raw(content) => format!("'{}'", content),
        }
    }
}