use crate::expression::{Expr, Literal};

impl Expr {
    pub fn evaluate(&self) -> String {
        match self {
            Expr::Literal(literal) => format!("{literal}"),
            Expr::Group(_) => todo!(),
            Expr::Unary(_, _) => todo!(),
            Expr::Arithmetic(bin_op, left, right) => match (left.as_ref(), right.as_ref()) {
                (
                    &Expr::Literal(Literal::Number(num_left)),
                    &Expr::Literal(Literal::Number(num_right)),
                ) => match bin_op {
                    crate::expression::BinOp::Add => format!("{}", num_left + num_right),
                    crate::expression::BinOp::Sub => format!("{}", num_left - num_right),
                    crate::expression::BinOp::Mul => format!("{}", num_left * num_right),
                    crate::expression::BinOp::Div => todo!("{}", num_left / num_right),
                    _ => "".to_string(),
                },
                _ => "".to_string(),
            },
        }
    }
}
