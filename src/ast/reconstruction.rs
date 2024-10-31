use std::fmt::{write, Display};

use crate::ast::expr::InOperator;

use super::{
    expr::{Expression, InfixExpr}, stmt::{BlockStmt, Field, FunctionStmt, Statement, VariableStmt}, types::Type
};

impl Display for Statement<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Struct(struct_stmt) => todo!(),
            Statement::Enum(enum_stmt) => todo!(),
            Statement::Union(union_stmt) => todo!(),
            Statement::Label(label_stmt) => todo!(),
            Statement::Function(function_stmt) => function_stmt.fmt(f),
            Statement::Variable(variable_stmt) => variable_stmt.fmt(f),
            Statement::If(if_stmt) => todo!(),
            Statement::Switch(switch_stmt) => todo!(),
            Statement::While(while_stmt) => todo!(),
            Statement::DoWhile(do_while_stmt) => todo!(),
            Statement::For(for_stmt) => todo!(),
            Statement::Typedef(typedef_stmt) => todo!(),
            Statement::Return(return_stmt) => todo!(),
            Statement::Break(break_stmt) => todo!(),
            Statement::Continue(continue_stmt) => todo!(),
            Statement::Goto(goto_stmt) => todo!(),
            Statement::Block(block_stmt) => todo!(),
            Statement::Expression(expression) => f.write_str(&(expression.to_string()+";")),
        }
    }
}

impl Display for FunctionStmt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{ret_type} {func_name}({args}) {block}",
            ret_type = self.ret_type,
            func_name = self.name,
            args = args_to_string(&self.args),
            block = body_to_string(self.body.as_ref().unwrap()),
        )
    }
}

fn args_to_string(args: &[Field<'_>]) -> String {
    let mut str = String::new();
    for arg in args {
        str.push_str(&arg.to_string());
        str.push(',');
    }
    str.remove(str.len() - 1);
    str
}

fn body_to_string(block: &BlockStmt<'_>) -> String {
    let mut str = String::from(" {");
    for stmt in &block.block {
        str.push_str(&stmt.to_string());
    }
    str.push('}');
    str
}

impl Display for Field<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{type_} {name}",
            type_ = self.field_type,
            name = self.name
        )
    }
}

impl Display for Type<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Ident(id) => f.write_str(id),
            Type::Pointer {
                type_,
                is_const,
                is_restricted,
            } => write!(
                f,
                "{}{type_}*{}",
                if *is_const { " const" } else { "" },
                if *is_restricted { " restrict" } else { "" }
            ),
            Type::Array { type_, size } => write!(
                f,
                "{type_}[{}]",
                if let Some(size) = size {
                    size.to_string()
                } else {
                    String::new()
                }
            ),
            Type::Struct(_) => todo!(),
            Type::Union(_) => todo!(),
            Type::Enum(_) => todo!(),
        }
    }
}

impl Display for Expression<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::LiteralString(str) => write!(f, "\"{str}\""),
            Expression::LiteralChar(_) => todo!(),
            Expression::LiteralShort(_) => todo!(),
            Expression::LiteralInt(_) => todo!(),
            Expression::LiteralLong(_) => todo!(),
            Expression::LiteralFloat(_) => todo!(),
            Expression::LiteralDouble(_) => todo!(),
            Expression::Ident(id) => id.fmt(f),
            Expression::Prefix(prefix_expr) => todo!(),
            Expression::Infix(infix_expr) => infix_expr.fmt(f),
            Expression::Post(post_expr) => todo!(),
            Expression::Call(call_expr) => todo!(),
        }
    }
}

impl Display for InfixExpr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.left, match self.op {
            InOperator::Add => "+",
            InOperator::Sub => "-",
            InOperator::Mul => "*",
            InOperator::Div => "/",
            InOperator::Mod => "%",
            InOperator::LSh => todo!(),
            InOperator::RSh => todo!(),
            InOperator::BAnd => todo!(),
            InOperator::BOr => todo!(),
            InOperator::BXor => todo!(),
            InOperator::Eq => todo!(),
            InOperator::Neq => todo!(),
            InOperator::LT => todo!(),
            InOperator::GT => todo!(),
            InOperator::LTE => todo!(),
            InOperator::GTE => todo!(),
            InOperator::And => todo!(),
            InOperator::Or => todo!(),
            InOperator::Assign => "=",
            InOperator::AssignAdd => todo!(),
            InOperator::AssignSub => todo!(),
            InOperator::AssignMul => todo!(),
            InOperator::AssignDiv => todo!(),
            InOperator::AssignMod => todo!(),
            InOperator::AssignLsh => todo!(),
            InOperator::AssignRsh => todo!(),
            InOperator::AssingBAnd => todo!(),
            InOperator::AssignBOr => todo!(),
            InOperator::AssignBXor => todo!(),
        }, self.right)
    }
}

impl Display for VariableStmt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} = {};", self.type_, self.name, self.val.as_ref().unwrap())
    }
}
