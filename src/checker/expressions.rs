use crate::{
    ast::{self, Expr, Node},
    error::ErrorCode,
    types::{self, DataType, IntegralSize},
};

use super::Checker;

impl<'a> Checker<'a> {
    pub(crate) fn check_expr(&mut self, node: &mut Node<Expr>) -> DataType {
        let copy = node.clone();
        let typ = match &mut node.src {
            Expr::Ident(i) => self.check_ident(copy, i),
            Expr::NumLit(_) => types::Int::new(IntegralSize::Int32), // Floats aren't real, they can't hurt you
            Expr::BoolLit(_) => types::Bool::new(),
            Expr::BinaryOp(binop) => self.check_binop(binop),
            Expr::UnaryOp(unop) => self.check_unop(unop),

            // In case any other expressions are added
            #[allow(unreachable_patterns)]
            _ => {
                self.panic(
                    "Invalid expression".to_owned(),
                    node,
                    ErrorCode::InvalidExpression,
                );
                panic!()
            }
        };

        node.typ = Some(typ.clone());
        typ
    }

    pub(crate) fn check_ident(&mut self, node: Node<Expr>, ident: &mut ast::Ident) -> DataType {
        match self.top.borrow().get(&ident.name) {
            Ok(vari) => match vari.borrow().typ.as_ref() {
                Some(val) => return val.clone(),
                None => self.panic(
                    format!("The variable '{}' does not exist", ident.name),
                    &node,
                    ErrorCode::VariableNotFound,
                ),
            },
            Err(err) => self.panic(
                format!("The variable '{}' does not exist", ident.name),
                &node,
                err,
            ),
        };
        DataType::Bool(types::Bool {})
    }

    pub(crate) fn check_binop(&mut self, binop: &mut ast::BinaryOp) -> DataType {
        let left_typ = self.check_expr(&mut binop.lhs);
        let right_typ = self.check_expr(&mut binop.rhs);

        if !left_typ.eq(&right_typ) {
            self.panic(
                format!(
                    "Cannot use the {} operator on {} and {}",
                    binop.op.src.src_strings().get(0).unwrap(),
                    left_typ.to_string(),
                    right_typ.to_string(),
                ),
                &binop.op,
                ErrorCode::TypeMismatch,
            );
        }

        left_typ // Since left_typ == right_typ
    }

    pub(crate) fn check_unop(&mut self, unop: &mut ast::UnaryOp) -> DataType {
        self.check_expr(&mut unop.val)
    }
}
