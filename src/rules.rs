////////////////////////////////////////////////////////////////////////
/// RULES
////////////////////////////////////////////////////////////////////////
use crate::expressions::*;

/// A rule as specified in the rules document
pub trait TRule {
    // every rule may have a comment describing why it failed
    fn get_comment(&self) -> &str;
    fn set_comment(&mut self, comment: String);
    // every rule may be evaluated
    fn eval(&self, items: &[String]) -> bool;
}

pub enum Rule {
    Order(Order), // TODO refactor this into a rule?
    Note(Note),
    Conflict(Conflict),
    Requires(Requires),
}

impl TRule for Rule {
    fn get_comment(&self) -> &str {
        match self {
            Rule::Order(o) => o.get_comment(),
            Rule::Note(o) => o.get_comment(),
            Rule::Conflict(o) => o.get_comment(),
            Rule::Requires(o) => o.get_comment(),
        }
    }

    fn set_comment(&mut self, comment: String) {
        match self {
            Rule::Order(_) => {}
            Rule::Note(n) => n.set_comment(comment),
            Rule::Conflict(c) => c.set_comment(comment),
            Rule::Requires(r) => r.set_comment(comment),
        }
    }

    fn eval(&self, items: &[String]) -> bool {
        match self {
            Rule::Order(o) => o.eval(items),
            Rule::Note(o) => o.eval(items),
            Rule::Conflict(o) => o.eval(items),
            Rule::Requires(o) => o.eval(items),
        }
    }
}

////////////////////////////////////////////////////////////////////////
/// IMPLEMENTATIONS
////////////////////////////////////////////////////////////////////////

/// The Note Rule <Note for A>
/// Notes simply check the expression and notify the user if eval is true
#[derive(Default, Clone)]
pub struct Order {
    pub name_a: String,
    pub name_b: String,
}

impl Order {
    pub fn new(name_a: String, name_b: String) -> Self {
        Self { name_a, name_b }
    }
}
impl TRule for Order {
    fn get_comment(&self) -> &str {
        ""
    }
    fn set_comment(&mut self, _comment: String) {}

    /// Notes evaluate as true if the expression evaluates as true
    fn eval(&self, _items: &[String]) -> bool {
        false
    }
}

/// The Note Rule <Note for A>
/// The [Note] rule prints the given message when any of the following expressions is true.
#[derive(Default, Clone)]
pub struct Note {
    pub comment: String,
    pub expressions: Vec<Expression>,
}

impl Note {
    pub fn new(comment: String, expressions: &[Expression]) -> Self {
        Self {
            comment,
            expressions: expressions.to_vec(),
        }
    }
}
impl TRule for Note {
    fn get_comment(&self) -> &str {
        self.comment.as_str()
    }
    fn set_comment(&mut self, comment: String) {
        self.comment = comment;
    }
    /// Notes evaluate as true if any of the containing expressions evaluates as true
    fn eval(&self, items: &[String]) -> bool {
        for expr in &self.expressions {
            if expr.eval(items) {
                return true;
            }
        }
        false
    }
}

/// The Conflict Rule <A conflicts with B>
/// Conflicts evaluate as true if both expressions evaluate as true
#[derive(Default, Clone)]
pub struct Conflict {
    pub comment: String,
    // todo: make first atomic?
    pub expression_a: Option<Expression>,
    pub expression_b: Option<Expression>,
}
impl Conflict {
    pub fn new(comment: String, expression_a: Expression, expression_b: Expression) -> Self {
        Self {
            comment,
            expression_a: Some(expression_a),
            expression_b: Some(expression_b),
        }
    }
}
impl TRule for Conflict {
    fn get_comment(&self) -> &str {
        self.comment.as_str()
    }
    fn set_comment(&mut self, comment: String) {
        self.comment = comment;
    }
    /// Conflicts evaluate as true if both expressions evaluate as true
    fn eval(&self, items: &[String]) -> bool {
        if let Some(expr_a) = &self.expression_a {
            if let Some(expr_b) = &self.expression_b {
                return expr_a.eval(items) && expr_b.eval(items);
            }
        }
        false
    }
}

/// The Require Rule <A requires B>
/// Requires evaluates as true if A is true and B is not true
#[derive(Default, Clone)]
pub struct Requires {
    pub comment: String,
    // todo: make first atomic?
    pub expression_a: Option<Expression>,
    pub expression_b: Option<Expression>,
}
impl Requires {
    pub fn new(comment: String, expression_a: Expression, expression_b: Expression) -> Self {
        Self {
            comment,
            expression_a: Some(expression_a),
            expression_b: Some(expression_b),
        }
    }
}
impl TRule for Requires {
    fn get_comment(&self) -> &str {
        self.comment.as_str()
    }
    fn set_comment(&mut self, comment: String) {
        self.comment = comment;
    }
    /// Requires evaluates as true if A is true and B is not true
    fn eval(&self, items: &[String]) -> bool {
        if let Some(expr_a) = &self.expression_a {
            if let Some(expr_b) = &self.expression_b {
                return expr_a.eval(items) && !expr_b.eval(items);
            }
        }
        false
    }
}
