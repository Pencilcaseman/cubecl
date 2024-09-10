use quote::quote;
use syn::{spanned::Spanned, ExprForLoop, ExprIf, ExprLoop, ExprWhile, Ident};

use crate::{
    expression::{Block, Expression},
    operator::Operator,
    scope::Context,
    statement::{parse_pat, Statement},
};

use super::helpers::Unroll;

pub fn expand_for_loop(for_loop: ExprForLoop, context: &mut Context) -> syn::Result<Expression> {
    let span = for_loop.span();
    let unroll = Unroll::from_attributes(&for_loop.attrs, context)?.map(|it| it.value);

    let right = Expression::from_expr(*for_loop.expr.clone(), context)
        .map_err(|_| syn::Error::new(span, "Unsupported for loop expression"))?;
    let var = parse_pat(*for_loop.pat)?;

    if right.is_const() && !matches!(right, Expression::Range { .. }) {
        return expand_for_in_loop(var.ident, right, for_loop.body, context);
    }

    let block = context.with_scope(|context| {
        context.push_variable(
            var.ident.clone(),
            var.ty.clone(),
            false,
            var.is_ref,
            var.is_mut,
        );
        Block::from_block(for_loop.body, context)
    })?;

    Ok(Expression::ForLoop {
        range: Box::new(right),
        unroll: unroll.map(Box::new),
        var_name: var.ident,
        var_ty: var.ty,
        block,
    })
}

fn expand_for_in_loop(
    var_name: Ident,
    right: Expression,
    block: syn::Block,
    context: &mut Context,
) -> syn::Result<Expression> {
    let statements = block
        .stmts
        .into_iter()
        .map(|stmt| Statement::from_stmt(stmt, context))
        .collect::<Result<Vec<_>, _>>()?;

    let right = right.to_tokens(context);
    let statements = statements.into_iter().map(|it| it.to_tokens(context));
    let for_loop = Expression::VerbatimTerminated {
        tokens: quote! {
            for #var_name in #right {
                #(#statements)*
            }
        },
    };
    Ok(for_loop)
}

pub fn expand_while_loop(while_loop: ExprWhile, context: &mut Context) -> syn::Result<Expression> {
    let span = while_loop.span();

    let condition = Expression::from_expr(*while_loop.cond, context)
        .map_err(|_| syn::Error::new(span, "Unsupported while condition"))?;
    let inverted = Expression::Unary {
        input: Box::new(condition),
        operator: Operator::Not,
        ty: None,
    };

    let block = context.with_scope(|ctx| Block::from_block(while_loop.body, ctx))?;
    Ok(Expression::WhileLoop {
        condition: Box::new(inverted),
        block,
    })
}

pub fn expand_loop(loop_expr: ExprLoop, context: &mut Context) -> syn::Result<Expression> {
    let block = context.with_scope(|ctx| Block::from_block(loop_expr.body, ctx))?;
    Ok(Expression::Loop(block))
}

pub fn expand_if(if_expr: ExprIf, context: &mut Context) -> syn::Result<Expression> {
    let span = if_expr.span();
    let condition = Expression::from_expr(*if_expr.cond, context)
        .map_err(|_| syn::Error::new(span, "Unsupported while condition"))?;

    let then_block = context.with_scope(|ctx| Block::from_block(if_expr.then_branch, ctx))?;
    let else_branch = if let Some((_, else_branch)) = if_expr.else_branch {
        Some(context.with_scope(|ctx| Expression::from_expr(*else_branch, ctx))?)
    } else {
        None
    };
    Ok(Expression::If {
        condition: Box::new(condition),
        then_block,
        else_branch: else_branch.map(Box::new),
    })
}

impl Block {
    pub fn from_block(block: syn::Block, context: &mut Context) -> syn::Result<Self> {
        let mut statements = block
            .stmts
            .into_iter()
            .map(|stmt| Statement::from_stmt(stmt, context))
            .collect::<Result<Vec<_>, _>>()?;
        // Pop implicit return if it exists so we can assign it as the block output
        let ret = match statements.pop() {
            Some(Statement::Expression {
                expression,
                terminated: false,
                ..
            }) => Some(expression),
            Some(stmt) => {
                statements.push(stmt);
                None
            }
            _ => None,
        };
        let ty = ret.as_ref().and_then(|ret| ret.ty());
        Ok(Self {
            inner: statements,
            ret,
            ty,
        })
    }
}