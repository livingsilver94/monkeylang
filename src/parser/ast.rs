#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    Let {
        identifier: String,
        expression: Expression,
    },
    Return(Expression),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    None,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct AST {
    statements: Vec<Statement>,
}

impl AST {
    pub fn new(st: Vec<Statement>) -> Self {
        Self { statements: st }
    }

    pub fn push(&mut self, st: Statement) {
        self.statements.push(st);
    }
}
