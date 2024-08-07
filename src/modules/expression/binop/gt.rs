use heraclitus_compiler::prelude::*;
use crate::docs::module::DocumentationModule;
use crate::translate::compute::{ArithOp, translate_computation};
use crate::utils::{ParserMetadata, TranslateMetadata};
use crate::translate::module::TranslateModule;
use super::{super::expr::Expr, parse_left_expr, expression_arms_of_type};
use crate::modules::types::{Typed, Type};

#[derive(Debug, Clone)]
pub struct Gt {
    left: Box<Expr>,
    right: Box<Expr>
}

impl Typed for Gt {
    fn get_type(&self) -> Type {
        Type::Bool
    }
}

impl SyntaxModule<ParserMetadata> for Gt {
    syntax_name!("Gt");

    fn new() -> Self {
        Gt {
            left: Box::new(Expr::new()),
            right: Box::new(Expr::new())
        }
    }

    fn parse(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        parse_left_expr(meta, &mut self.left, ">")?;
        let tok = meta.get_current_token();
        token(meta, ">")?;
        syntax(meta, &mut *self.right)?;
        let l_type = self.left.get_type();
        let r_type = self.right.get_type();
        let message = format!("Cannot compare two values of different types '{l_type}' > '{r_type}'");
        let predicate = |kind| matches!(kind, Type::Num);
        expression_arms_of_type(meta, &l_type, &r_type, predicate, tok, &message)?;
        Ok(())
    }
}

impl TranslateModule for Gt {
    fn translate(&self, meta: &mut TranslateMetadata) -> String {
        let left = self.left.translate(meta);
        let right = self.right.translate(meta);
        translate_computation(meta, ArithOp::Gt, Some(left), Some(right))
    }
}

impl DocumentationModule for Gt {
    fn document(&self, _meta: &ParserMetadata) -> String {
        "".to_string()
    }
}
