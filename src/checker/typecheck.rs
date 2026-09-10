use super::CheckError;
use super::convert::convert;
use super::motives::{
    JMotiveInput, apply_unary_motive, build_j_branch_type, build_j_result_type,
    build_nat_step_type, validate_j_motive_type, validate_unary_motive_type,
};
use super::reduce::{expose_pi, expose_sigma, whnf};
use super::state::{CheckedGlobals, LocalContext};
use super::transform::{TransformError, substitute_top};
use crate::error::{FormatError, FormatErrorClass};
use crate::syntax::{Arena, Natural, Term, TermId};

/// Synthesize a type for the complete Core v0.1 term language.
///
/// On logical or resource failure, arena and local-context changes made by the
/// operation are rolled back. Successful synthesis may retain derived arena
/// nodes because the returned `TermId` can refer to them.
pub(super) fn synthesize_core(
    arena: &mut Arena,
    globals: &CheckedGlobals,
    context: &mut LocalContext,
    term: TermId,
) -> Result<TermId, CheckError> {
    match run(arena, globals, context, Goal::Synthesize(term))? {
        Value::Term(ty) => Ok(ty),
        _ => unreachable!("synthesis produces exactly one synthesized type"),
    }
}

/// Check a term against an already validated expected type in Core v0.1.
///
/// Successful checking is arena-neutral because no derived `TermId` escapes.
pub(super) fn check_core(
    arena: &mut Arena,
    globals: &CheckedGlobals,
    context: &mut LocalContext,
    term: TermId,
    expected: TermId,
) -> Result<(), CheckError> {
    match run(arena, globals, context, Goal::Check(term, expected))? {
        Value::Checked => Ok(()),
        _ => unreachable!("checking produces exactly one checked marker"),
    }
}

// Compatibility shims keep the PR #20 regression file mechanically stable.
#[cfg(test)]
pub(super) fn synthesize_motive_free(
    arena: &mut Arena,
    globals: &CheckedGlobals,
    context: &mut LocalContext,
    term: TermId,
) -> Result<Option<TermId>, CheckError> {
    synthesize_core(arena, globals, context, term).map(Some)
}

#[cfg(test)]
pub(super) fn check_motive_free(
    arena: &mut Arena,
    globals: &CheckedGlobals,
    context: &mut LocalContext,
    term: TermId,
    expected: TermId,
) -> Result<Option<()>, CheckError> {
    check_core(arena, globals, context, term, expected).map(|()| Some(()))
}

#[derive(Clone, Copy)]
enum Goal {
    Synthesize(TermId),
    Check(TermId, TermId),
}

fn run(
    arena: &mut Arena,
    globals: &CheckedGlobals,
    context: &mut LocalContext,
    goal: Goal,
) -> Result<Value, CheckError> {
    let arena_checkpoint = arena.len();
    let context_checkpoint = context.len();
    let declaration_index = globals.next_declaration_index();
    let mut tasks = Vec::new();
    let mut values = Vec::new();

    let initial = match goal {
        Goal::Synthesize(term) => Task::Synthesize(term),
        Goal::Check(term, expected) => Task::Check(term, expected),
    };
    push_task(&mut tasks, declaration_index, initial)?;

    let result = run_machine(
        arena,
        globals,
        context,
        declaration_index,
        &mut tasks,
        &mut values,
    );

    match result {
        Ok(()) => {
            assert_eq!(
                context.len(),
                context_checkpoint,
                "successful typing restores the local context"
            );
            assert_eq!(values.len(), 1, "typing produces exactly one result");
            let value = values.pop().expect("one typing result exists");
            if matches!(goal, Goal::Check(_, _)) {
                arena.truncate(arena_checkpoint);
            }
            Ok(value)
        }
        Err(error) => {
            restore_context(context, context_checkpoint);
            arena.truncate(arena_checkpoint);
            Err(error)
        }
    }
}

fn run_machine(
    arena: &mut Arena,
    globals: &CheckedGlobals,
    context: &mut LocalContext,
    declaration_index: usize,
    tasks: &mut Vec<Task>,
    values: &mut Vec<Value>,
) -> Result<(), CheckError> {
    while let Some(task) = tasks.pop() {
        match task {
            Task::Synthesize(source) => {
                let term = arena
                    .get(source)
                    .expect("synthesis source belongs to arena")
                    .clone();
                match term {
                    Term::Var(index) => {
                        let ty = context.lookup(arena, declaration_index, source, &index)?;
                        push_value(values, declaration_index, Value::Term(ty))?;
                    }
                    Term::Global(index) => {
                        let ty = globals.lookup(source, &index)?.ty();
                        push_value(values, declaration_index, Value::Term(ty))?;
                    }
                    Term::Universe(level) => {
                        let next = level
                            .try_add_usize(1)
                            .map_err(|error| from_format_error(error, declaration_index))?;
                        let ty = append(arena, declaration_index, Term::Universe(next))?;
                        push_value(values, declaration_index, Value::Term(ty))?;
                    }
                    Term::Pi(domain, codomain) => {
                        push_task(
                            tasks,
                            declaration_index,
                            Task::ProductAfterDomain {
                                kind: ProductKind::Pi,
                                domain,
                                codomain,
                            },
                        )?;
                        push_task(tasks, declaration_index, Task::InferUniverse(domain))?;
                    }
                    Term::Sigma(domain, codomain) => {
                        push_task(
                            tasks,
                            declaration_index,
                            Task::ProductAfterDomain {
                                kind: ProductKind::Sigma,
                                domain,
                                codomain,
                            },
                        )?;
                        push_task(tasks, declaration_index, Task::InferUniverse(domain))?;
                    }
                    Term::Lam(_) | Term::Pair(_, _) => {
                        return Err(CheckError::invalid_judgment(declaration_index, source));
                    }
                    Term::App(function, argument) => {
                        push_task(
                            tasks,
                            declaration_index,
                            Task::AppAfterFunction { source, argument },
                        )?;
                        push_task(tasks, declaration_index, Task::Synthesize(function))?;
                    }
                    Term::Fst(principal) => {
                        push_task(tasks, declaration_index, Task::FstAfterPrincipal { source })?;
                        push_task(tasks, declaration_index, Task::Synthesize(principal))?;
                    }
                    Term::Snd(principal) => {
                        push_task(
                            tasks,
                            declaration_index,
                            Task::SndAfterPrincipal { source, principal },
                        )?;
                        push_task(tasks, declaration_index, Task::Synthesize(principal))?;
                    }
                    Term::Id(ty, left, right) => {
                        push_task(
                            tasks,
                            declaration_index,
                            Task::IdAfterUniverse { ty, left, right },
                        )?;
                        push_task(tasks, declaration_index, Task::InferUniverse(ty))?;
                    }
                    Term::Refl(value) => {
                        push_task(tasks, declaration_index, Task::ReflAfterValue { value })?;
                        push_task(tasks, declaration_index, Task::Synthesize(value))?;
                    }
                    Term::Empty | Term::Unit | Term::Nat => {
                        let ty = append_universe_zero(arena, declaration_index)?;
                        push_value(values, declaration_index, Value::Term(ty))?;
                    }
                    Term::Star => {
                        let ty = append(arena, declaration_index, Term::Unit)?;
                        push_value(values, declaration_index, Value::Term(ty))?;
                    }
                    Term::Zero => {
                        let ty = append(arena, declaration_index, Term::Nat)?;
                        push_value(values, declaration_index, Value::Term(ty))?;
                    }
                    Term::Succ(predecessor) => {
                        let nat = append(arena, declaration_index, Term::Nat)?;
                        push_task(tasks, declaration_index, Task::SuccAfterCheck { nat })?;
                        push_task(tasks, declaration_index, Task::Check(predecessor, nat))?;
                    }
                    Term::Ann(value, ty) => {
                        push_task(
                            tasks,
                            declaration_index,
                            Task::AnnAfterUniverse { value, ty },
                        )?;
                        push_task(tasks, declaration_index, Task::InferUniverse(ty))?;
                    }
                    Term::J(ty, base, motive, branch, endpoint, path) => {
                        push_task(
                            tasks,
                            declaration_index,
                            Task::JAfterUniverse {
                                ty,
                                base,
                                motive,
                                branch,
                                endpoint,
                                path,
                            },
                        )?;
                        push_task(tasks, declaration_index, Task::InferUniverse(ty))?;
                    }
                    Term::EmptyElim(motive, scrutinee) => {
                        push_task(
                            tasks,
                            declaration_index,
                            Task::EmptyAfterMotive { motive, scrutinee },
                        )?;
                        push_task(tasks, declaration_index, Task::Synthesize(motive))?;
                    }
                    Term::UnitElim(motive, branch, scrutinee) => {
                        push_task(
                            tasks,
                            declaration_index,
                            Task::UnitAfterMotive {
                                motive,
                                branch,
                                scrutinee,
                            },
                        )?;
                        push_task(tasks, declaration_index, Task::Synthesize(motive))?;
                    }
                    Term::NatElim(motive, zero_case, step, scrutinee) => {
                        push_task(
                            tasks,
                            declaration_index,
                            Task::NatAfterMotive {
                                motive,
                                zero_case,
                                step,
                                scrutinee,
                            },
                        )?;
                        push_task(tasks, declaration_index, Task::Synthesize(motive))?;
                    }
                }
            }
            Task::JAfterUniverse {
                ty,
                base,
                motive,
                branch,
                endpoint,
                path,
            } => {
                let _ = pop_level(values);
                push_task(
                    tasks,
                    declaration_index,
                    Task::JAfterBase {
                        ty,
                        base,
                        motive,
                        branch,
                        endpoint,
                        path,
                    },
                )?;
                push_task(tasks, declaration_index, Task::Check(base, ty))?;
            }
            Task::JAfterBase {
                ty,
                base,
                motive,
                branch,
                endpoint,
                path,
            } => {
                pop_checked(values);
                push_task(
                    tasks,
                    declaration_index,
                    Task::JAfterMotive {
                        ty,
                        base,
                        motive,
                        branch,
                        endpoint,
                        path,
                    },
                )?;
                push_task(tasks, declaration_index, Task::Synthesize(motive))?;
            }
            Task::JAfterMotive {
                ty,
                base,
                motive,
                branch,
                endpoint,
                path,
            } => {
                let motive_type = pop_term(values);
                let _ = validate_j_motive_type(
                    arena,
                    globals,
                    context,
                    declaration_index,
                    JMotiveInput {
                        motive,
                        motive_type,
                        ty,
                        base,
                    },
                )?;
                let branch_type = build_j_branch_type(arena, declaration_index, motive, base)?;
                push_task(
                    tasks,
                    declaration_index,
                    Task::JAfterBranch {
                        ty,
                        base,
                        motive,
                        endpoint,
                        path,
                    },
                )?;
                push_task(tasks, declaration_index, Task::Check(branch, branch_type))?;
            }
            Task::JAfterBranch {
                ty,
                base,
                motive,
                endpoint,
                path,
            } => {
                pop_checked(values);
                push_task(
                    tasks,
                    declaration_index,
                    Task::JAfterEndpoint {
                        ty,
                        base,
                        motive,
                        endpoint,
                        path,
                    },
                )?;
                push_task(tasks, declaration_index, Task::Check(endpoint, ty))?;
            }
            Task::JAfterEndpoint {
                ty,
                base,
                motive,
                endpoint,
                path,
            } => {
                pop_checked(values);
                let path_type = append(arena, declaration_index, Term::Id(ty, base, endpoint))?;
                push_task(
                    tasks,
                    declaration_index,
                    Task::JAfterPath {
                        motive,
                        endpoint,
                        path,
                    },
                )?;
                push_task(tasks, declaration_index, Task::Check(path, path_type))?;
            }
            Task::JAfterPath {
                motive,
                endpoint,
                path,
            } => {
                pop_checked(values);
                let result = build_j_result_type(arena, declaration_index, motive, endpoint, path)?;
                push_value(values, declaration_index, Value::Term(result))?;
            }
            Task::EmptyAfterMotive { motive, scrutinee } => {
                let motive_type = pop_term(values);
                let empty = append(arena, declaration_index, Term::Empty)?;
                let _ = validate_unary_motive_type(
                    arena,
                    globals,
                    context,
                    declaration_index,
                    motive,
                    motive_type,
                    empty,
                )?;
                push_task(
                    tasks,
                    declaration_index,
                    Task::EmptyAfterScrutinee { motive, scrutinee },
                )?;
                push_task(tasks, declaration_index, Task::Check(scrutinee, empty))?;
            }
            Task::EmptyAfterScrutinee { motive, scrutinee } => {
                pop_checked(values);
                let result = apply_unary_motive(arena, declaration_index, motive, scrutinee)?;
                push_value(values, declaration_index, Value::Term(result))?;
            }
            Task::UnitAfterMotive {
                motive,
                branch,
                scrutinee,
            } => {
                let motive_type = pop_term(values);
                let unit = append(arena, declaration_index, Term::Unit)?;
                let _ = validate_unary_motive_type(
                    arena,
                    globals,
                    context,
                    declaration_index,
                    motive,
                    motive_type,
                    unit,
                )?;
                let star = append(arena, declaration_index, Term::Star)?;
                let branch_type = apply_unary_motive(arena, declaration_index, motive, star)?;
                push_task(
                    tasks,
                    declaration_index,
                    Task::UnitAfterBranch {
                        motive,
                        scrutinee,
                        unit,
                    },
                )?;
                push_task(tasks, declaration_index, Task::Check(branch, branch_type))?;
            }
            Task::UnitAfterBranch {
                motive,
                scrutinee,
                unit,
            } => {
                pop_checked(values);
                push_task(
                    tasks,
                    declaration_index,
                    Task::UnitAfterScrutinee { motive, scrutinee },
                )?;
                push_task(tasks, declaration_index, Task::Check(scrutinee, unit))?;
            }
            Task::UnitAfterScrutinee { motive, scrutinee } => {
                pop_checked(values);
                let result = apply_unary_motive(arena, declaration_index, motive, scrutinee)?;
                push_value(values, declaration_index, Value::Term(result))?;
            }
            Task::NatAfterMotive {
                motive,
                zero_case,
                step,
                scrutinee,
            } => {
                let motive_type = pop_term(values);
                let nat = append(arena, declaration_index, Term::Nat)?;
                let _ = validate_unary_motive_type(
                    arena,
                    globals,
                    context,
                    declaration_index,
                    motive,
                    motive_type,
                    nat,
                )?;
                let zero = append(arena, declaration_index, Term::Zero)?;
                let zero_type = apply_unary_motive(arena, declaration_index, motive, zero)?;
                push_task(
                    tasks,
                    declaration_index,
                    Task::NatAfterZero {
                        motive,
                        step,
                        scrutinee,
                        nat,
                    },
                )?;
                push_task(tasks, declaration_index, Task::Check(zero_case, zero_type))?;
            }
            Task::NatAfterZero {
                motive,
                step,
                scrutinee,
                nat,
            } => {
                pop_checked(values);
                let step_type = build_nat_step_type(arena, declaration_index, motive)?;
                push_task(
                    tasks,
                    declaration_index,
                    Task::NatAfterStep {
                        motive,
                        scrutinee,
                        nat,
                    },
                )?;
                push_task(tasks, declaration_index, Task::Check(step, step_type))?;
            }
            Task::NatAfterStep {
                motive,
                scrutinee,
                nat,
            } => {
                pop_checked(values);
                push_task(
                    tasks,
                    declaration_index,
                    Task::NatAfterScrutinee { motive, scrutinee },
                )?;
                push_task(tasks, declaration_index, Task::Check(scrutinee, nat))?;
            }
            Task::NatAfterScrutinee { motive, scrutinee } => {
                pop_checked(values);
                let result = apply_unary_motive(arena, declaration_index, motive, scrutinee)?;
                push_value(values, declaration_index, Value::Term(result))?;
            }
            Task::Check(source, expected) => {
                let term = arena
                    .get(source)
                    .expect("checking source belongs to arena")
                    .clone();
                match term {
                    Term::Lam(body) => {
                        let Some((domain, codomain)) = expose_pi(arena, globals, expected)? else {
                            return Err(CheckError::invalid_judgment(declaration_index, source));
                        };
                        context.push_checked(declaration_index, domain)?;
                        push_task(tasks, declaration_index, Task::LamAfterBody { domain })?;
                        push_task(tasks, declaration_index, Task::Check(body, codomain))?;
                    }
                    Term::Pair(first, second) => {
                        let Some((domain, codomain)) = expose_sigma(arena, globals, expected)?
                        else {
                            return Err(CheckError::invalid_judgment(declaration_index, source));
                        };
                        push_task(
                            tasks,
                            declaration_index,
                            Task::PairAfterFirst {
                                first,
                                second,
                                codomain,
                            },
                        )?;
                        push_task(tasks, declaration_index, Task::Check(first, domain))?;
                    }
                    _ => {
                        push_task(
                            tasks,
                            declaration_index,
                            Task::CheckAfterSynthesis { source, expected },
                        )?;
                        push_task(tasks, declaration_index, Task::Synthesize(source))?;
                    }
                }
            }
            Task::InferUniverse(subject) => {
                push_task(
                    tasks,
                    declaration_index,
                    Task::InferUniverseAfterSynthesis { subject },
                )?;
                push_task(tasks, declaration_index, Task::Synthesize(subject))?;
            }
            Task::InferUniverseAfterSynthesis { subject } => {
                let synthesized = pop_term(values);
                let exposed = whnf(arena, globals, synthesized)?;
                let Term::Universe(level) = arena
                    .get(exposed)
                    .expect("exposed synthesized type belongs to arena")
                else {
                    return Err(CheckError::invalid_judgment(declaration_index, subject));
                };
                push_value(values, declaration_index, Value::Level(level.clone()))?;
            }
            Task::ProductAfterDomain {
                kind,
                domain,
                codomain,
            } => {
                let domain_level = pop_level(values);
                context.push_checked(declaration_index, domain)?;
                push_task(
                    tasks,
                    declaration_index,
                    Task::ProductAfterCodomain {
                        kind,
                        domain,
                        domain_level,
                    },
                )?;
                push_task(tasks, declaration_index, Task::InferUniverse(codomain))?;
            }
            Task::ProductAfterCodomain {
                kind,
                domain,
                domain_level,
            } => {
                let codomain_level = pop_level(values);
                assert_eq!(
                    context.pop(),
                    Some(domain),
                    "product synthesis pops the domain it pushed"
                );
                let level = if domain_level >= codomain_level {
                    domain_level
                } else {
                    codomain_level
                };
                let ty = append(arena, declaration_index, Term::Universe(level))?;
                let _ = kind;
                push_value(values, declaration_index, Value::Term(ty))?;
            }
            Task::AppAfterFunction { source, argument } => {
                let function_type = pop_term(values);
                let Some((domain, codomain)) = expose_pi(arena, globals, function_type)? else {
                    return Err(CheckError::invalid_judgment(declaration_index, source));
                };
                push_task(
                    tasks,
                    declaration_index,
                    Task::AppAfterArgument { codomain, argument },
                )?;
                push_task(tasks, declaration_index, Task::Check(argument, domain))?;
            }
            Task::AppAfterArgument { codomain, argument } => {
                pop_checked(values);
                let result = substitute_top(arena, codomain, argument)
                    .map_err(|error| from_transform_error(error, declaration_index))?;
                push_value(values, declaration_index, Value::Term(result))?;
            }
            Task::FstAfterPrincipal { source } => {
                let principal_type = pop_term(values);
                let Some((domain, _)) = expose_sigma(arena, globals, principal_type)? else {
                    return Err(CheckError::invalid_judgment(declaration_index, source));
                };
                push_value(values, declaration_index, Value::Term(domain))?;
            }
            Task::SndAfterPrincipal { source, principal } => {
                let principal_type = pop_term(values);
                let Some((_, codomain)) = expose_sigma(arena, globals, principal_type)? else {
                    return Err(CheckError::invalid_judgment(declaration_index, source));
                };
                let first = append(arena, declaration_index, Term::Fst(principal))?;
                let result = substitute_top(arena, codomain, first)
                    .map_err(|error| from_transform_error(error, declaration_index))?;
                push_value(values, declaration_index, Value::Term(result))?;
            }
            Task::IdAfterUniverse { ty, left, right } => {
                let level = pop_level(values);
                push_task(
                    tasks,
                    declaration_index,
                    Task::IdAfterLeft { ty, right, level },
                )?;
                push_task(tasks, declaration_index, Task::Check(left, ty))?;
            }
            Task::IdAfterLeft { ty, right, level } => {
                pop_checked(values);
                push_task(tasks, declaration_index, Task::IdAfterRight { level })?;
                push_task(tasks, declaration_index, Task::Check(right, ty))?;
            }
            Task::IdAfterRight { level } => {
                pop_checked(values);
                let result = append(arena, declaration_index, Term::Universe(level))?;
                push_value(values, declaration_index, Value::Term(result))?;
            }
            Task::ReflAfterValue { value } => {
                let ty = pop_term(values);
                let result = append(arena, declaration_index, Term::Id(ty, value, value))?;
                push_value(values, declaration_index, Value::Term(result))?;
            }
            Task::SuccAfterCheck { nat } => {
                pop_checked(values);
                push_value(values, declaration_index, Value::Term(nat))?;
            }
            Task::AnnAfterUniverse { value, ty } => {
                let _ = pop_level(values);
                push_task(tasks, declaration_index, Task::AnnAfterCheck { ty })?;
                push_task(tasks, declaration_index, Task::Check(value, ty))?;
            }
            Task::AnnAfterCheck { ty } => {
                pop_checked(values);
                push_value(values, declaration_index, Value::Term(ty))?;
            }
            Task::LamAfterBody { domain } => {
                pop_checked(values);
                assert_eq!(
                    context.pop(),
                    Some(domain),
                    "lambda checking pops the domain it pushed"
                );
                push_value(values, declaration_index, Value::Checked)?;
            }
            Task::PairAfterFirst {
                first,
                second,
                codomain,
            } => {
                pop_checked(values);
                let second_type = substitute_top(arena, codomain, first)
                    .map_err(|error| from_transform_error(error, declaration_index))?;
                push_task(tasks, declaration_index, Task::PairAfterSecond)?;
                push_task(tasks, declaration_index, Task::Check(second, second_type))?;
            }
            Task::PairAfterSecond => {
                pop_checked(values);
                push_value(values, declaration_index, Value::Checked)?;
            }
            Task::CheckAfterSynthesis { source, expected } => {
                let synthesized = pop_term(values);
                if !convert(arena, globals, synthesized, expected)? {
                    return Err(CheckError::invalid_judgment(declaration_index, source));
                }
                push_value(values, declaration_index, Value::Checked)?;
            }
        }
    }

    Ok(())
}

#[derive(Clone, Copy)]
enum ProductKind {
    Pi,
    Sigma,
}

enum Task {
    Synthesize(TermId),
    Check(TermId, TermId),
    InferUniverse(TermId),
    InferUniverseAfterSynthesis {
        subject: TermId,
    },
    ProductAfterDomain {
        kind: ProductKind,
        domain: TermId,
        codomain: TermId,
    },
    ProductAfterCodomain {
        kind: ProductKind,
        domain: TermId,
        domain_level: Natural,
    },
    AppAfterFunction {
        source: TermId,
        argument: TermId,
    },
    AppAfterArgument {
        codomain: TermId,
        argument: TermId,
    },
    FstAfterPrincipal {
        source: TermId,
    },
    SndAfterPrincipal {
        source: TermId,
        principal: TermId,
    },
    IdAfterUniverse {
        ty: TermId,
        left: TermId,
        right: TermId,
    },
    IdAfterLeft {
        ty: TermId,
        right: TermId,
        level: Natural,
    },
    IdAfterRight {
        level: Natural,
    },
    ReflAfterValue {
        value: TermId,
    },
    SuccAfterCheck {
        nat: TermId,
    },
    AnnAfterUniverse {
        value: TermId,
        ty: TermId,
    },
    AnnAfterCheck {
        ty: TermId,
    },
    LamAfterBody {
        domain: TermId,
    },
    PairAfterFirst {
        first: TermId,
        second: TermId,
        codomain: TermId,
    },
    PairAfterSecond,
    JAfterUniverse {
        ty: TermId,
        base: TermId,
        motive: TermId,
        branch: TermId,
        endpoint: TermId,
        path: TermId,
    },
    JAfterBase {
        ty: TermId,
        base: TermId,
        motive: TermId,
        branch: TermId,
        endpoint: TermId,
        path: TermId,
    },
    JAfterMotive {
        ty: TermId,
        base: TermId,
        motive: TermId,
        branch: TermId,
        endpoint: TermId,
        path: TermId,
    },
    JAfterBranch {
        ty: TermId,
        base: TermId,
        motive: TermId,
        endpoint: TermId,
        path: TermId,
    },
    JAfterEndpoint {
        ty: TermId,
        base: TermId,
        motive: TermId,
        endpoint: TermId,
        path: TermId,
    },
    JAfterPath {
        motive: TermId,
        endpoint: TermId,
        path: TermId,
    },
    EmptyAfterMotive {
        motive: TermId,
        scrutinee: TermId,
    },
    EmptyAfterScrutinee {
        motive: TermId,
        scrutinee: TermId,
    },
    UnitAfterMotive {
        motive: TermId,
        branch: TermId,
        scrutinee: TermId,
    },
    UnitAfterBranch {
        motive: TermId,
        scrutinee: TermId,
        unit: TermId,
    },
    UnitAfterScrutinee {
        motive: TermId,
        scrutinee: TermId,
    },
    NatAfterMotive {
        motive: TermId,
        zero_case: TermId,
        step: TermId,
        scrutinee: TermId,
    },
    NatAfterZero {
        motive: TermId,
        step: TermId,
        scrutinee: TermId,
        nat: TermId,
    },
    NatAfterStep {
        motive: TermId,
        scrutinee: TermId,
        nat: TermId,
    },
    NatAfterScrutinee {
        motive: TermId,
        scrutinee: TermId,
    },
    CheckAfterSynthesis {
        source: TermId,
        expected: TermId,
    },
}

enum Value {
    Term(TermId),
    Level(Natural),
    Checked,
}

fn push_task(
    tasks: &mut Vec<Task>,
    declaration_index: usize,
    task: Task,
) -> Result<(), CheckError> {
    tasks
        .try_reserve(1)
        .map_err(|_| CheckError::resource_exhausted(declaration_index))?;
    tasks.push(task);
    Ok(())
}

fn push_value(
    values: &mut Vec<Value>,
    declaration_index: usize,
    value: Value,
) -> Result<(), CheckError> {
    values
        .try_reserve(1)
        .map_err(|_| CheckError::resource_exhausted(declaration_index))?;
    values.push(value);
    Ok(())
}

fn pop_term(values: &mut Vec<Value>) -> TermId {
    match values.pop().expect("typing continuation has a result") {
        Value::Term(term) => term,
        _ => unreachable!("typing continuation expected a synthesized type"),
    }
}

fn pop_level(values: &mut Vec<Value>) -> Natural {
    match values.pop().expect("universe inference has a result") {
        Value::Level(level) => level,
        _ => unreachable!("universe inference expected a literal level"),
    }
}

fn pop_checked(values: &mut Vec<Value>) {
    match values.pop().expect("checking continuation has a result") {
        Value::Checked => {}
        _ => unreachable!("checking continuation expected a checked marker"),
    }
}

fn restore_context(context: &mut LocalContext, checkpoint: usize) {
    assert!(
        context.len() >= checkpoint,
        "typing never removes pre-existing local entries"
    );
    while context.len() > checkpoint {
        context.pop().expect("extra local entry exists");
    }
}

fn append_universe_zero(arena: &mut Arena, declaration_index: usize) -> Result<TermId, CheckError> {
    let zero =
        Natural::from_decimal("0").map_err(|error| from_format_error(error, declaration_index))?;
    append(arena, declaration_index, Term::Universe(zero))
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
        _ => unreachable!("checker-derived terms preserve the arena invariant"),
    }
}
