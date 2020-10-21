use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Prop {
    pub name: char,
    pub val: bool,
}

impl fmt::Display for Prop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}: {})", self.name, self.val)
    }
}

impl Prop {
    pub fn new<S: Into<char>>(id: S, val: bool) -> Prop {
        Prop { name: id.into(), val }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Super(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    Prop(Prop),
}

impl Expr {
    pub fn from_prop(p: Prop) -> Box<Expr> {
        Box::new(Expr::Prop(p))
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        macro_rules! display_bin_expr {
            ($e:ident, $e1: ident, $op: tt, $f: ident) => {{
                write!($f, "(")?;
                $e.fmt($f)?;
                write!($f, " {} ", $op)?;
                $e1.fmt($f)?;
                write!($f, ")")
            }};
        }
        match self {
            Expr::And(e, e2) => display_bin_expr!(e, e2, "∧", f),
            Expr::Or(e, e2) => display_bin_expr!(e, e2, "∨", f),
            Expr::Super(e, e2) => display_bin_expr!(e, e2, "⊃", f),
            Expr::Not(e) => {
                write!(f, "¬")?;
                e.fmt(f)
            }
            Expr::Prop(e) => { e.fmt(f) }
        }
    }
}
