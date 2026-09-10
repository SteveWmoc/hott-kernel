use super::CheckError;
use super::convert::convert;
use super::reduce::{expose_pi, whnf};
use super::state::{CheckedGlobals, LocalContext};
use super::transform::{TransformError, shift};
use crate::error::{FormatError, FormatErrorClass};
use crate::syntax::{Arena, Natural, Term, TermId};

/// Finish the frozen `exposeUnaryMotive(C, X)` protocol after the caller has
/// synthesized the motive type `T_C`.
///
/// The operation is arena- and context-neutral: it returns only the literal
/// universe level exposed from the motive codomain, so every derived term used
/// while exposing or comparing the motive type is discarded before return.
pub(super) fn validate_unary_motive_type(
    arena: &mut Arena,
    globals: &CheckedGlobals,
    context: &mut LocalContext,
    declaration_index: usize,
    motive: TermId,
    motive_type: TermId,
    expected_domain: TermId,
) -> Result<Natural, CheckError> {
    let arena_checkpoint = arena.len();
    let context_checkpoint = context.len();

    let result = (|| {
        let Some((domain, codomain)) = expose_pi(arena, globals, motive_type)? else {
            return Err(CheckError::invalid_judgment(declaration_index, motive));
        };
        if !convert(arena, globals, domain, expected_domain)? {
            return Err(CheckError::invalid_judgment(declaration_index, motive));
        }

        context.push_checked(declaration_index, domain)?;
        let exposed = whnf(arena, globals, codomain)?;
        let Some(Term::Universe(level)) = arena.get(exposed) else {
            return Err(CheckError::invalid_judgment(declaration_index, motive));
        };
        Ok(level.clone())
    })();

    restore_context(context, context_checkpoint);
    arena.truncate(arena_checkpoint);
    result
}

/// Finish the frozen `exposeJMotive(C, A, a)` protocol after the caller has
/// synthesized the motive type `T_C`.
///
/// The second domain is compared against exactly
/// `Id_(shift A) (shift a) (var 0)` in the context extended by the first
/// domain. No metavariable, level search, or eta expansion is introduced.
/// Like unary motive recognition, this helper is arena- and context-neutral.
pub(super) struct JMotiveInput {
    pub(super) motive: TermId,
    pub(super) motive_type: TermId,
    pub(super) ty: TermId,
    pub(super) base: TermId,
}

pub(super) fn validate_j_motive_type(
    arena: &mut Arena,
    globals: &CheckedGlobals,
    context: &mut LocalContext,
    declaration_index: usize,
    input: JMotiveInput,
) -> Result<Natural, CheckError> {
    let JMotiveInput {
        motive,
        motive_type,
        ty,
        base,
    } = input;
    let arena_checkpoint = arena.len();
    let context_checkpoint = context.len();

    let result = (|| {
        let Some((first_domain, first_codomain)) = expose_pi(arena, globals, motive_type)? else {
            return Err(CheckError::invalid_judgment(declaration_index, motive));
        };
        if !convert(arena, globals, first_domain, ty)? {
            return Err(CheckError::invalid_judgment(declaration_index, motive));
        }

        context.push_checked(declaration_index, first_domain)?;
        let Some((path_domain, result_codomain)) = expose_pi(arena, globals, first_codomain)?
        else {
            return Err(CheckError::invalid_judgment(declaration_index, motive));
        };

        let shifted_ty = shift(arena, ty, 1, 0)
            .map_err(|error| from_transform_error(error, declaration_index))?;
        let shifted_base = shift(arena, base, 1, 0)
            .map_err(|error| from_transform_error(error, declaration_index))?;
        let newest = append_variable(arena, declaration_index, "0")?;
        let expected_path = append(
            arena,
            declaration_index,
            Term::Id(shifted_ty, shifted_base, newest),
        )?;
        if !convert(arena, globals, path_domain, expected_path)? {
            return Err(CheckError::invalid_judgment(declaration_index, motive));
        }

        context.push_checked(declaration_index, path_domain)?;
        let exposed = whnf(arena, globals, result_codomain)?;
        let Some(Term::Universe(level)) = arena.get(exposed) else {
            return Err(CheckError::invalid_judgment(declaration_index, motive));
        };
        Ok(level.clone())
    })();

    restore_context(context, context_checkpoint);
    arena.truncate(arena_checkpoint);
    result
}

/// Build `C a (refl a)` for the J computation branch.
pub(super) fn build_j_branch_type(
    arena: &mut Arena,
    declaration_index: usize,
    motive: TermId,
    base: TermId,
) -> Result<TermId, CheckError> {
    let at_base = append(arena, declaration_index, Term::App(motive, base))?;
    let reflexivity = append(arena, declaration_index, Term::Refl(base))?;
    append(arena, declaration_index, Term::App(at_base, reflexivity))
}

/// Build `C b p`, the synthesized result type of J.
pub(super) fn build_j_result_type(
    arena: &mut Arena,
    declaration_index: usize,
    motive: TermId,
    endpoint: TermId,
    path: TermId,
) -> Result<TermId, CheckError> {
    let at_endpoint = append(arena, declaration_index, Term::App(motive, endpoint))?;
    append(arena, declaration_index, Term::App(at_endpoint, path))
}

/// Build `C x` for a unary motive.
pub(super) fn apply_unary_motive(
    arena: &mut Arena,
    declaration_index: usize,
    motive: TermId,
    argument: TermId,
) -> Result<TermId, CheckError> {
    append(arena, declaration_index, Term::App(motive, argument))
}

/// Build the frozen natural-number step type
/// `Pi(k : Nat). Pi(h : C k). C (succ k)`.
///
/// The motive is shifted once under `k` and twice under `k,h`; this is the
/// soundness-sensitive de Bruijn bookkeeping that the Core specification
/// requires rather than an alpha-renaming convenience.
pub(super) fn build_nat_step_type(
    arena: &mut Arena,
    declaration_index: usize,
    motive: TermId,
) -> Result<TermId, CheckError> {
    let nat = append(arena, declaration_index, Term::Nat)?;

    let motive_under_k = shift(arena, motive, 1, 0)
        .map_err(|error| from_transform_error(error, declaration_index))?;
    let k = append_variable(arena, declaration_index, "0")?;
    let motive_at_k = append(arena, declaration_index, Term::App(motive_under_k, k))?;

    let motive_under_k_and_h = shift(arena, motive, 2, 0)
        .map_err(|error| from_transform_error(error, declaration_index))?;
    let shifted_k = append_variable(arena, declaration_index, "1")?;
    let successor = append(arena, declaration_index, Term::Succ(shifted_k))?;
    let motive_at_successor = append(
        arena,
        declaration_index,
        Term::App(motive_under_k_and_h, successor),
    )?;

    let induction_step = append(
        arena,
        declaration_index,
        Term::Pi(motive_at_k, motive_at_successor),
    )?;
    append(arena, declaration_index, Term::Pi(nat, induction_step))
}

fn restore_context(context: &mut LocalContext, checkpoint: usize) {
    assert!(
        context.len() >= checkpoint,
        "motive recognition never removes pre-existing local entries"
    );
    while context.len() > checkpoint {
        context.pop().expect("extra motive-local entry exists");
    }
}

fn append_variable(
    arena: &mut Arena,
    declaration_index: usize,
    text: &str,
) -> Result<TermId, CheckError> {
    let index =
        Natural::from_decimal(text).map_err(|error| from_format_error(error, declaration_index))?;
    append(arena, declaration_index, Term::Var(index))
}

fn append(arena: &mut Arena, declaration_index: usize, term: Term) -> Result<TermId, CheckError> {
    arena
        .push(term)
        .map_err(|error| from_format_error(error, declaration_index))
}

fn from_transform_error(_: TransformError, declaration_index: usize) -> CheckError {
    CheckError::resource_exhausted(declaration_index)
}

fn from_format_error(error: FormatError, declaration_index: usize) -> CheckError {
    match error.class() {
        FormatErrorClass::ResourceExhausted => CheckError::resource_exhausted(declaration_index),
        _ => unreachable!("checker-derived motive terms preserve the arena invariant"),
    }
}
