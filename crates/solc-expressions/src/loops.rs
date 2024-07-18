use crate::{variable::Variable, ContextBuilder, Flatten};
use graph::nodes::SubContextKind;
use graph::ContextEdge;
use graph::Edge;

use graph::{
    elem::Elem,
    nodes::{Concrete, Context, ContextNode},
    AnalyzerBackend, GraphBackend,
};
use shared::{ExprErr, IntoExprErr, RangeArena};

use solang_parser::pt::{CodeLocation, Expression, Loc, Statement};

impl<T> Looper for T where
    T: AnalyzerBackend<Expr = Expression, ExprErr = ExprErr> + Sized + GraphBackend
{
}

/// Dealing with loops
pub trait Looper:
    GraphBackend + AnalyzerBackend<Expr = Expression, ExprErr = ExprErr> + Sized
{
    /// Resets all variables referenced in the loop because we don't elegantly handle loops
    fn reset_vars(
        &mut self,
        arena: &mut RangeArena<Elem<Concrete>>,
        loc: Loc,
        ctx: ContextNode,
        body: &Statement,
    ) -> Result<(), ExprErr> {
        let og_ctx = ctx;
        let subctx = Context::new_loop_subctx(ctx, loc, self).into_expr_err(loc)?;
        ctx.set_child_call(subctx, self).into_expr_err(loc)?;
        self.add_edge(subctx, ctx, Edge::Context(ContextEdge::Loop));

        self.traverse_statement(body, None);
        self.interpret(subctx, body.loc(), arena);
        self.apply_to_edges(subctx, loc, arena, &|analyzer, arena, ctx, loc| {
            let vars = subctx.local_vars(analyzer).clone();
            vars.iter().for_each(|(name, var)| {
                // widen to max range
                if let Some(inheritor_var) = ctx.var_by_name(analyzer, name) {
                    let inheritor_var = inheritor_var.latest_version(analyzer);
                    if let Some(r) = var
                        .underlying(analyzer)
                        .unwrap()
                        .ty
                        .default_range(analyzer)
                        .unwrap()
                    {
                        let new_inheritor_var = analyzer
                            .advance_var_in_ctx(inheritor_var, loc, ctx)
                            .unwrap();
                        let res = new_inheritor_var
                            .set_range_min(analyzer, arena, r.min)
                            .into_expr_err(loc);
                        let _ = analyzer.add_if_err(res);
                        let res = new_inheritor_var
                            .set_range_max(analyzer, arena, r.max)
                            .into_expr_err(loc);
                        let _ = analyzer.add_if_err(res);
                    }
                }
            });

            let subctx_kind = SubContextKind::new_fn_ret(ctx, og_ctx);
            let sctx = Context::add_subctx(subctx_kind, loc, analyzer, None).into_expr_err(loc)?;
            ctx.set_child_call(sctx, analyzer).into_expr_err(loc)
        })
    }

    /// Handles a while-loop
    fn while_loop(
        &mut self,
        arena: &mut RangeArena<Elem<Concrete>>,
        loc: Loc,
        ctx: ContextNode,
        _limiter: &Expression,
        body: &Statement,
    ) -> Result<(), ExprErr> {
        // TODO: improve this
        self.apply_to_edges(ctx, loc, arena, &|analyzer, arena, ctx, loc| {
            analyzer.reset_vars(arena, loc, ctx, body)
        })
    }
}
