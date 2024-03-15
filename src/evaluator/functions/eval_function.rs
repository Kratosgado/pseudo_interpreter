use crate::{
    evaluator::{evaluator::FuncArgs, EvalExpression, EvalResult, EvalStatement, Expr, Statement},
    Evaluator,
};

pub trait EvalFunction {
    fn eval_function(&mut self, statement: &Statement);
}

impl EvalFunction for Evaluator {
    fn eval_function(&mut self, statement: &Statement) {
        if let Statement::Function(name, params, fstatements, ret_ment) = statement {
            fn func(
                eval: &mut Evaluator,
                params: &Vec<Expr>,
                fstatements: &Box<Vec<Statement>>,
                ret_ment: &Option<Expr>,
            ) -> Option<Expr> {
                let mut p: Vec<EvalResult> = Vec::new();
                for param in params.iter() {
                    p.push(eval.evaluate_expr(param))
                }
                for statement in fstatements.iter() {
                    eval.evaluate_statement(statement);
                }
                ret_ment.clone()
            }
            let func_args = FuncArgs{
                params: params.clone(),
                statements: fstatements.clone(),
                ret_ment: ret_ment.clone(),
            };
            self.function_table.insert(name.clone(), Box::new(func));
            self.function_args.insert(name.clone(), func_args);
            self.next_statement();
            // todo!("function evaluator not finished")
        }
    }
}
