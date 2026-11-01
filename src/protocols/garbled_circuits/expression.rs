use std::ops::{BitAnd, BitOr, BitXor, Not};


#[derive(Clone)]
pub enum Expr {
    Input(usize),
    Not(Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Xor(Box<Expr>, Box<Expr>),
}

impl Expr{
    pub fn num_inputs(&self) -> usize {
        match self {
            Expr::Input(id) => *id + 1,
            Expr::Not(inner) => inner.num_inputs(),
            Expr::And(left, right) 
            | Expr::Or(left, right) 
            | Expr::Xor(left, right) => {
                std::cmp::max(left.num_inputs(), right.num_inputs())
            }
        }
    }
}


impl Not for Expr {
    type Output = Expr;
    fn not(self) -> Self::Output {
        Expr::Not(Box::new(self))
    }
}

impl BitAnd for Expr {
    type Output = Expr;
    fn bitand(self, rhs: Self) -> Self::Output {
        Expr::And(Box::new(self), Box::new(rhs))
    }
}

impl BitOr for Expr {
    type Output = Expr;
    fn bitor(self, rhs: Self) -> Self::Output {
        Expr::Or(Box::new(self), Box::new(rhs))
    }
}

impl BitXor for Expr {
    type Output = Expr;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Expr::Xor(Box::new(self), Box::new(rhs))
    }
}

impl Not for &Expr {
    type Output = Expr;
    fn not(self) -> Self::Output {
        Expr::Not(Box::new(self.clone()))
    }
}

impl BitAnd<&Expr> for Expr {
    type Output = Expr;
    fn bitand(self, rhs: &Expr) -> Self::Output {
        Expr::And(Box::new(self), Box::new(rhs.clone()))
    }
}

impl BitAnd<Expr> for &Expr {
    type Output = Expr;
    fn bitand(self, rhs: Expr) -> Self::Output {
        Expr::And(Box::new(self.clone()), Box::new(rhs))
    }
}


impl BitOr<&Expr> for Expr {
    type Output = Expr;
    fn bitor(self, rhs: &Expr) -> Self::Output {
        Expr::Or(Box::new(self), Box::new(rhs.clone()))
    }
}

impl BitOr<Expr> for &Expr {
    type Output = Expr;
    fn bitor(self, rhs: Expr) -> Self::Output {
        Expr::Or(Box::new(self.clone()), Box::new(rhs))
    }
}

impl BitXor<&Expr> for Expr {
    type Output = Expr;
    fn bitxor(self, rhs: &Expr) -> Self::Output {
        Expr::Xor(Box::new(self), Box::new(rhs.clone()))
    }
}

impl BitXor<Expr> for &Expr {
    type Output = Expr;
    fn bitxor(self, rhs: Expr) -> Self::Output {
        Expr::Xor(Box::new(self.clone()), Box::new(rhs))
    }
}


impl BitAnd for &Expr {
    type Output = Expr;
    fn bitand(self, rhs: Self) -> Self::Output {
        Expr::And(Box::new(self.clone()), Box::new(rhs.clone()))
    }
}

impl BitOr for &Expr {
    type Output = Expr;
    fn bitor(self, rhs: Self) -> Self::Output {
        Expr::Or(Box::new(self.clone()), Box::new(rhs.clone()))
    }
}

impl BitXor for &Expr {
    type Output = Expr;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Expr::Xor(Box::new(self.clone()), Box::new(rhs.clone()))
    }
}