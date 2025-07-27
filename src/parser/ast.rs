

#[derive(Debug, Clone)]
pub enum AstNode {
    Program { statements: Vec<AstNode> },
    ExpressionStatement {
        expression: Box<Expression>
    },
    VariableDeclaration {
        var_type: String, 
        name: String, 
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
        body: Vec<AstNode> 
    },
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
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub param_type: String,
    pub name: String,
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
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}

impl Literal {
    pub fn to_php(&self) -> String {
        match self {
            Self::Integer(n) => n.to_string(),
            Self::Float(f) => f.to_string(),
            Self::String(s) => format!("\"{}\"", s),
            Self::Boolean(b) => (if *b { "true" } else { "false" }).to_string(),
        }
    }
}