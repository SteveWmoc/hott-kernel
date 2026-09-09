use super::CheckError;
use super::convert::convert;
use super::reduce::{expose_pi, expose_sigma, whnf};
use super::state::{CheckedGlobals, LocalContext};
use super::transform::{TransformError, substitute_top};
use crate::error::{FormatError, FormatErrorClass};
use crate::syntax::{Arena, Natural, Term, TermId};

/// Synthesize a type for the Core v0.1 fragment that does not require motive
/// decomposition.
///
/// `Ok(None)` is deliberately not a rejection. It means that the requested
/// derivation encountered one of the four motive-driven eliminators (`J`,
/// empty elimination, unit elimination, or natural-number elimination), whose
/// exact motive recognition is staged for the next trusted slice. On `Err` or
/// `Ok(None)`, all arena and local-context changes made by this operation are
/// rolled back.
pub(super) fn synthesize_motive_free(
    arena: &mut Arena,
    globals: &CheckedGlobals,
    context: &mut LocalContext,
    term: TermId,
) -> Result<Option<TermId>, CheckError> {
    match run(arena, globals, context, Goal::Synthesize(term))? {
        Some(Value::Term(ty)) => Ok(Some(ty)),
        None => Ok(None),
        Some(_) => unreachable!("synthesis produces exactly one synthesized type"),
    }
}

/// Check a term against an already validated expected type in the motive-free
/// Core v0.1 fragment.
///
/// As with [`synthesize_motive_free`], `Ok(None)` means only that motive-driven
/// eliminator support is required; it carries no logical verdict. Successful
/// checking is arena-neutral because no derived `TermId` escapes.
pub(super) fn check_motive_free(
    arena: &mut Arena,
    globals: &CheckedGlobals,
    context: &mut LocalContext,
    term: TermId,
    expected: TermId,
) -> Result<Option<()>, CheckError> {
    match run(arena, globals, context, Goal::Check(term, expected))? {
        Some(Value::Checked) => Ok(Some(())),
        None => Ok(None),
        Some(_) => unreachable!("checking produces exactly one checked marker"),
    }
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
) -> Result<Option<Value>, CheckError> {
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
        Ok(MachineOutcome::Complete) => {
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
            Ok(Some(value))
        }
        Ok(MachineOutcome::Deferred) => {
            restore_context(context, context_checkpoint);
            arena.truncate(arena_checkpoint);
            Ok(None)
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
) -> Result<MachineOutcome, CheckError> {
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
                    Term::J(_, _, _, _, _, _)
                    | Term::EmptyElim(_, _)
                    | Term::UnitElim(_, _, _)
                    | Term::NatElim(_, _, _, _) => return Ok(MachineOutcome::Deferred),
                }
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

    Ok(MachineOutcome::Complete)
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

#[derive(Clone, Copy)]
enum MachineOutcome {
    Complete,
    Deferred,
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
