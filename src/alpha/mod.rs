#![allow(unused)] // TODO: Remove once this stuff has been stabilized

mod procedure;
mod procedure_like;
mod router;
mod rspc;

pub use self::rspc::*;
pub use procedure::*;
pub use procedure_like::*;
pub use router::*;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::Rspc;

    #[allow(non_upper_case_globals)]
    const t: Rspc<()> = Rspc::new();

    #[test]
    fn test_alpha_api() {
        // TODO: Get Context switching?
        // TODO: `TMeta` working on a procedure level?

        let r = t
            .router()
            .procedure(
                "todo",
                t.with(|mw| {
                    mw.middleware(|mw| async move {
                        let state = (mw.req.clone(), mw.ctx.clone(), mw.input.clone());
                        Ok(mw.with_state(state))
                    })
                    .resp(|state, result| async move {
                        println!(
                            "[LOG] req='{:?}' ctx='{:?}'  input='{:?}' result='{:?}'",
                            state.0, state.1, state.2, result
                        );
                        Ok(result)
                    })
                })
                .query(|ctx, _: ()| {
                    println!("TODO: {:?}", ctx);
                    Ok(())
                }),
                // .meta(()),
            )
            .procedure(
                "todo2",
                t.with(|mw| {
                    mw.middleware(|mw| async move {
                        let state = (mw.req.clone(), mw.ctx.clone(), mw.input.clone());
                        Ok(mw.with_state(state))
                    })
                    .resp(|state, result| async move {
                        println!(
                            "[LOG] req='{:?}' ctx='{:?}'  input='{:?}' result='{:?}'",
                            state.0, state.1, state.2, result
                        );
                        Ok(result)
                    })
                })
                .with(|mw| {
                    mw.middleware(|mw| async move {
                        let state = (mw.req.clone(), mw.ctx.clone(), mw.input.clone());
                        Ok(mw.with_state(state))
                    })
                    .resp(|state, result| async move {
                        println!(
                            "[LOG] req='{:?}' ctx='{:?}'  input='{:?}' result='{:?}'",
                            state.0, state.1, state.2, result
                        );
                        Ok(result)
                    })
                })
                .query(|ctx, _: ()| {
                    println!("TODO: {:?}", ctx);
                    Ok(())
                }),
                // .meta(()),
            )
            .procedure(
                "todo3",
                t.query(|ctx, _: ()| {
                    println!("TODO: {:?}", ctx);
                    Ok(())
                }),
            )
            .compat();

        r.export_ts(PathBuf::from("./demo.bindings.ts")).unwrap();
    }

    // fn admin_middleware() -> impl Middleware {} // TODO: basic middleware + context switching
    // TODO: Allowing a router to take parameters -> Will require proxy syntax on frontend
    // TODO: Internally storing those as `fn()` instead of `impl Fn()` (Basically using a `Cow` for functions)??
}