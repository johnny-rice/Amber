use heraclitus_compiler::prelude::*;
use crate::modules::prelude::*;
use crate::modules::expression::expr::Expr;
use crate::modules::variable::{handle_variable_reference, prevent_constant_mutation, variable_name_extensions};
use crate::translate::compute::translate_computation_eval;
use crate::translate::compute::ArithOp;
use crate::modules::types::Type;

use super::shorthand_typecheck_allowed_types;

#[derive(Debug, Clone)]
pub struct ShorthandSub {
    var: String,
    expr: Box<Expr>,
    kind: Type,
    global_id: Option<usize>,
    is_ref: bool
}

impl SyntaxModule<ParserMetadata> for ShorthandSub {
    syntax_name!("Shorthand Sub");

    fn new() -> Self {
        Self {
            var: String::new(),
            expr: Box::new(Expr::new()),
            kind: Type::Null,
            global_id: None,
            is_ref: false
        }
    }

    fn parse(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        let var_tok = meta.get_current_token();
        self.var = variable(meta, variable_name_extensions())?;
        token(meta, "-=")?;
        let variable = handle_variable_reference(meta, &var_tok, &self.var)?;
        prevent_constant_mutation(meta, &var_tok, &self.var, variable.is_const)?;
        self.kind = variable.kind;
        self.global_id = variable.global_id;
        self.is_ref = variable.is_ref;
        syntax(meta, &mut *self.expr)?;
        shorthand_typecheck_allowed_types(meta, "subtract", &self.kind, &self.expr, &[
            Type::Num,
            Type::Int,
        ])?;
        Ok(())
    }
}

impl TranslateModule for ShorthandSub {
    //noinspection DuplicatedCode
    fn translate(&self, meta: &mut TranslateMetadata) -> FragmentKind {
        let var = VarExprFragment::new(&self.var, self.kind.clone())
            .with_global_id(self.global_id)
            .with_ref(self.is_ref);
        let expr = match self.kind {
            Type::Int => {
                let expr = self.expr.translate_eval(meta, self.is_ref);
                ArithmeticFragment::new(var.to_frag(), ArithOp::Sub, expr).to_frag()
            }
            Type::Num => {
                let expr = self.expr.translate_eval(meta, self.is_ref);
                translate_computation_eval(meta, ArithOp::Sub, Some(var.to_frag()), Some(expr), self.is_ref)
            }
            _ => unreachable!("Unsupported type {} in shorthand subtraction operation", self.kind)
        };
        VarStmtFragment::new(&self.var, self.kind.clone(), expr)
            .with_global_id(self.global_id)
            .with_ref(self.is_ref)
            .to_frag()
    }
}

impl DocumentationModule for ShorthandSub {
    fn document(&self, _meta: &ParserMetadata) -> String {
        "".to_string()
    }
}
