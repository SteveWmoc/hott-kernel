use super::CheckError;
use super::state::{CheckedGlobals, LocalContext};
use super::typecheck::{check_core, infer_universe_core};
use crate::syntax::{Declaration, Module};

/// Check a parsed Core v0.1 module in one forward declaration pass.
///
/// The module is borrowed mutably only because the checker uses its term arena
/// as append/truncate scratch space. The arena is restored to its original
/// length before every return, so a successful or failed check leaves the
/// parsed module unchanged.
pub fn check_module(module: &mut Module) -> Result<(), CheckError> {
    let (arena, declarations) = module.checking_parts();
    let arena_checkpoint = arena.len();
    let mut globals = CheckedGlobals::new();

    for declaration in declarations {
        let mut context = LocalContext::new();
        let result = check_declaration(arena, &mut globals, &mut context, declaration);

        // No checker-derived node may escape a declaration boundary. Every
        // committed global entry refers only to source-arena TermIds.
        arena.truncate(arena_checkpoint);

        result?;
        debug_assert!(context.is_empty());
    }

    Ok(())
}

fn check_declaration(
    arena: &mut crate::syntax::Arena,
    globals: &mut CheckedGlobals,
    context: &mut LocalContext,
    declaration: &Declaration,
) -> Result<(), CheckError> {
    debug_assert!(context.is_empty());

    // Section 15.5: the declared type must inhabit a literal universe before
    // the current declaration is made visible.
    let _ = infer_universe_core(arena, globals, context, declaration.ty())?;

    // Definitions additionally check their body against the declared type.
    // Postulates have no body.
    if let Some(body) = declaration.body() {
        check_core(arena, globals, context, body, declaration.ty())?;
    }

    // Commit only after every obligation for this declaration succeeds. This
    // makes self and forward references unavailable by construction.
    globals.commit(declaration)?;
    Ok(())
}
