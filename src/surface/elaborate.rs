use std::collections::HashMap;

use crate::error::FormatErrorClass;
use crate::surface::{SurfaceDeclaration, SurfaceError, SurfaceModule, SurfaceTerm, SurfaceTermId};
use crate::syntax::{Arena, Declaration, Module, Natural, Term, TermId};

pub fn elaborate_surface(module: &SurfaceModule) -> Result<Module, SurfaceError> {
    let mut arena = Arena::new();
    let mut declarations = Vec::new();
    declarations
        .try_reserve(module.declarations().len())
        .map_err(|_| SurfaceError::resource_exhausted())?;

    let mut globals = HashMap::<&str, usize>::new();
    globals
        .try_reserve(module.declarations().len())
        .map_err(|_| SurfaceError::resource_exhausted())?;

    for declaration in module.declarations() {
        let name = declaration.name().as_str();
        if globals.contains_key(name) {
            return Err(SurfaceError::duplicate_global());
        }

        let ty = elaborate_term(module, declaration.ty(), &globals, &mut arena)?;
        let body = match declaration.body() {
            Some(body) => Some(elaborate_term(module, body, &globals, &mut arena)?),
            None => None,
        };
        let core_name = clone_string(name)?;

        let core_declaration = match declaration {
            SurfaceDeclaration::Postulate { .. } => Declaration::Postulate {
                name: core_name,
                ty,
            },
            SurfaceDeclaration::Transparent { .. } => Declaration::Transparent {
                name: core_name,
                ty,
                body: body.expect("transparent declaration has a body"),
            },
            SurfaceDeclaration::Opaque { .. } => Declaration::Opaque {
                name: core_name,
                ty,
                body: body.expect("opaque declaration has a body"),
            },
        };

        declarations.push(core_declaration);
        globals.insert(name, declarations.len() - 1);
    }

    Ok(Module::new(arena, declarations))
}

fn elaborate_term(
    module: &SurfaceModule,
    root: SurfaceTermId,
    globals: &HashMap<&str, usize>,
    arena: &mut Arena,
) -> Result<TermId, SurfaceError> {
    let mut work = Vec::<Work<'_>>::new();
    let mut values = Vec::<TermId>::new();
    let mut locals = Vec::<&str>::new();

    push_work(&mut work, Work::Eval(root))?;

    while let Some(item) = work.pop() {
        match item {
            Work::Eval(id) => {
                let term = module
                    .arena()
                    .get(id)
                    .ok_or_else(|| SurfaceError::malformed(0))?;

                match term {
                    SurfaceTerm::Ref(name) => {
                        let text = name.as_str();
                        if let Some(depth) = locals.iter().rev().position(|local| *local == text) {
                            push_value(
                                &mut values,
                                push_core(arena, Term::Var(natural_from_usize(depth)?))?,
                            )?;
                        } else if let Some(index) = globals.get(text).copied() {
                            push_value(
                                &mut values,
                                push_core(arena, Term::Global(natural_from_usize(index)?))?,
                            )?;
                        } else {
                            return Err(SurfaceError::unknown_name());
                        }
                    }
                    SurfaceTerm::Universe(level) => {
                        push_value(
                            &mut values,
                            push_core(arena, Term::Universe(level.clone()))?,
                        )?;
                    }
                    SurfaceTerm::Pi {
                        binder,
                        domain,
                        codomain,
                    } => {
                        schedule_binding_binary(
                            &mut work,
                            BuildKind::Pi,
                            binder.as_str(),
                            *domain,
                            *codomain,
                        )?;
                    }
                    SurfaceTerm::Lam { binder, body } => {
                        push_work(&mut work, Work::Build(BuildKind::Lam))?;
                        push_work(&mut work, Work::PopLocal)?;
                        push_work(&mut work, Work::Eval(*body))?;
                        push_work(&mut work, Work::PushLocal(binder.as_str()))?;
                    }
                    SurfaceTerm::App(function, argument) => {
                        schedule_binary(&mut work, BuildKind::App, *function, *argument)?;
                    }
                    SurfaceTerm::Sigma {
                        binder,
                        domain,
                        codomain,
                    } => {
                        schedule_binding_binary(
                            &mut work,
                            BuildKind::Sigma,
                            binder.as_str(),
                            *domain,
                            *codomain,
                        )?;
                    }
                    SurfaceTerm::Pair(first, second) => {
                        schedule_binary(&mut work, BuildKind::Pair, *first, *second)?;
                    }
                    SurfaceTerm::Fst(pair) => {
                        schedule_unary(&mut work, BuildKind::Fst, *pair)?;
                    }
                    SurfaceTerm::Snd(pair) => {
                        schedule_unary(&mut work, BuildKind::Snd, *pair)?;
                    }
                    SurfaceTerm::Id(ty, left, right) => {
                        schedule_ternary(&mut work, BuildKind::Id, *ty, *left, *right)?;
                    }
                    SurfaceTerm::Refl(term) => {
                        schedule_unary(&mut work, BuildKind::Refl, *term)?;
                    }
                    SurfaceTerm::J(a, b, c, d, e, f) => {
                        schedule_six(&mut work, BuildKind::J, [*a, *b, *c, *d, *e, *f])?;
                    }
                    SurfaceTerm::Empty => {
                        push_value(&mut values, push_core(arena, Term::Empty)?)?;
                    }
                    SurfaceTerm::EmptyElim(motive, scrutinee) => {
                        schedule_binary(&mut work, BuildKind::EmptyElim, *motive, *scrutinee)?;
                    }
                    SurfaceTerm::Unit => {
                        push_value(&mut values, push_core(arena, Term::Unit)?)?;
                    }
                    SurfaceTerm::Star => {
                        push_value(&mut values, push_core(arena, Term::Star)?)?;
                    }
                    SurfaceTerm::UnitElim(motive, branch, scrutinee) => {
                        schedule_ternary(
                            &mut work,
                            BuildKind::UnitElim,
                            *motive,
                            *branch,
                            *scrutinee,
                        )?;
                    }
                    SurfaceTerm::Nat => {
                        push_value(&mut values, push_core(arena, Term::Nat)?)?;
                    }
                    SurfaceTerm::Zero => {
                        push_value(&mut values, push_core(arena, Term::Zero)?)?;
                    }
                    SurfaceTerm::Succ(predecessor) => {
                        schedule_unary(&mut work, BuildKind::Succ, *predecessor)?;
                    }
                    SurfaceTerm::NatElim(motive, zero_case, succ_case, scrutinee) => {
                        schedule_four(
                            &mut work,
                            BuildKind::NatElim,
                            [*motive, *zero_case, *succ_case, *scrutinee],
                        )?;
                    }
                    SurfaceTerm::Ann(term, ty) => {
                        schedule_binary(&mut work, BuildKind::Ann, *term, *ty)?;
                    }
                }
            }
            Work::PushLocal(name) => {
                locals
                    .try_reserve(1)
                    .map_err(|_| SurfaceError::resource_exhausted())?;
                locals.push(name);
            }
            Work::PopLocal => {
                locals.pop().expect("resolver local-stack invariant");
            }
            Work::Build(kind) => {
                let term = build_term(kind, &mut values);
                push_value(&mut values, push_core(arena, term)?)?;
            }
        }
    }

    if values.len() != 1 {
        return Err(SurfaceError::malformed(0));
    }
    Ok(values.pop().expect("one resolver result"))
}

fn schedule_unary(
    work: &mut Vec<Work<'_>>,
    kind: BuildKind,
    child: SurfaceTermId,
) -> Result<(), SurfaceError> {
    reserve_work(work, 2)?;
    work.push(Work::Build(kind));
    work.push(Work::Eval(child));
    Ok(())
}

fn schedule_binary(
    work: &mut Vec<Work<'_>>,
    kind: BuildKind,
    first: SurfaceTermId,
    second: SurfaceTermId,
) -> Result<(), SurfaceError> {
    reserve_work(work, 3)?;
    work.push(Work::Build(kind));
    work.push(Work::Eval(second));
    work.push(Work::Eval(first));
    Ok(())
}

fn schedule_binding_binary<'a>(
    work: &mut Vec<Work<'a>>,
    kind: BuildKind,
    binder: &'a str,
    domain: SurfaceTermId,
    codomain: SurfaceTermId,
) -> Result<(), SurfaceError> {
    reserve_work(work, 5)?;
    work.push(Work::Build(kind));
    work.push(Work::PopLocal);
    work.push(Work::Eval(codomain));
    work.push(Work::PushLocal(binder));
    work.push(Work::Eval(domain));
    Ok(())
}

fn schedule_ternary(
    work: &mut Vec<Work<'_>>,
    kind: BuildKind,
    first: SurfaceTermId,
    second: SurfaceTermId,
    third: SurfaceTermId,
) -> Result<(), SurfaceError> {
    reserve_work(work, 4)?;
    work.push(Work::Build(kind));
    work.push(Work::Eval(third));
    work.push(Work::Eval(second));
    work.push(Work::Eval(first));
    Ok(())
}

fn schedule_four(
    work: &mut Vec<Work<'_>>,
    kind: BuildKind,
    children: [SurfaceTermId; 4],
) -> Result<(), SurfaceError> {
    reserve_work(work, 5)?;
    work.push(Work::Build(kind));
    for child in children.into_iter().rev() {
        work.push(Work::Eval(child));
    }
    Ok(())
}

fn schedule_six(
    work: &mut Vec<Work<'_>>,
    kind: BuildKind,
    children: [SurfaceTermId; 6],
) -> Result<(), SurfaceError> {
    reserve_work(work, 7)?;
    work.push(Work::Build(kind));
    for child in children.into_iter().rev() {
        work.push(Work::Eval(child));
    }
    Ok(())
}

fn reserve_work(work: &mut Vec<Work<'_>>, additional: usize) -> Result<(), SurfaceError> {
    work.try_reserve(additional)
        .map_err(|_| SurfaceError::resource_exhausted())
}

fn push_work<'a>(work: &mut Vec<Work<'a>>, item: Work<'a>) -> Result<(), SurfaceError> {
    work.try_reserve(1)
        .map_err(|_| SurfaceError::resource_exhausted())?;
    work.push(item);
    Ok(())
}

fn push_value(values: &mut Vec<TermId>, value: TermId) -> Result<(), SurfaceError> {
    values
        .try_reserve(1)
        .map_err(|_| SurfaceError::resource_exhausted())?;
    values.push(value);
    Ok(())
}

fn build_term(kind: BuildKind, values: &mut Vec<TermId>) -> Term {
    match kind {
        BuildKind::Pi => {
            let [domain, codomain] = pop_values::<2>(values);
            Term::Pi(domain, codomain)
        }
        BuildKind::Lam => {
            let [body] = pop_values::<1>(values);
            Term::Lam(body)
        }
        BuildKind::App => {
            let [function, argument] = pop_values::<2>(values);
            Term::App(function, argument)
        }
        BuildKind::Sigma => {
            let [domain, codomain] = pop_values::<2>(values);
            Term::Sigma(domain, codomain)
        }
        BuildKind::Pair => {
            let [first, second] = pop_values::<2>(values);
            Term::Pair(first, second)
        }
        BuildKind::Fst => {
            let [pair] = pop_values::<1>(values);
            Term::Fst(pair)
        }
        BuildKind::Snd => {
            let [pair] = pop_values::<1>(values);
            Term::Snd(pair)
        }
        BuildKind::Id => {
            let [ty, left, right] = pop_values::<3>(values);
            Term::Id(ty, left, right)
        }
        BuildKind::Refl => {
            let [term] = pop_values::<1>(values);
            Term::Refl(term)
        }
        BuildKind::J => {
            let [a, b, c, d, e, f] = pop_values::<6>(values);
            Term::J(a, b, c, d, e, f)
        }
        BuildKind::EmptyElim => {
            let [motive, scrutinee] = pop_values::<2>(values);
            Term::EmptyElim(motive, scrutinee)
        }
        BuildKind::UnitElim => {
            let [motive, branch, scrutinee] = pop_values::<3>(values);
            Term::UnitElim(motive, branch, scrutinee)
        }
        BuildKind::Succ => {
            let [predecessor] = pop_values::<1>(values);
            Term::Succ(predecessor)
        }
        BuildKind::NatElim => {
            let [motive, zero_case, succ_case, scrutinee] = pop_values::<4>(values);
            Term::NatElim(motive, zero_case, succ_case, scrutinee)
        }
        BuildKind::Ann => {
            let [term, ty] = pop_values::<2>(values);
            Term::Ann(term, ty)
        }
    }
}

fn pop_values<const N: usize>(values: &mut Vec<TermId>) -> [TermId; N] {
    let start = values
        .len()
        .checked_sub(N)
        .expect("resolver value-stack invariant");
    let array = std::array::from_fn(|offset| values[start + offset]);
    values.truncate(start);
    array
}

fn push_core(arena: &mut Arena, term: Term) -> Result<TermId, SurfaceError> {
    arena.push(term).map_err(|error| match error.class() {
        FormatErrorClass::ResourceExhausted => SurfaceError::resource_exhausted(),
        _ => SurfaceError::malformed(0),
    })
}

fn natural_from_usize(mut value: usize) -> Result<Natural, SurfaceError> {
    let mut digits = [0_u8; 3 * core::mem::size_of::<usize>()];
    let mut cursor = digits.len();

    loop {
        cursor -= 1;
        digits[cursor] = b'0' + u8::try_from(value % 10).expect("decimal digit");
        value /= 10;
        if value == 0 {
            break;
        }
    }

    let text = std::str::from_utf8(&digits[cursor..]).expect("decimal digits are ASCII");
    Natural::from_decimal(text).map_err(|error| match error.class() {
        FormatErrorClass::ResourceExhausted => SurfaceError::resource_exhausted(),
        _ => SurfaceError::malformed(0),
    })
}

fn clone_string(text: &str) -> Result<String, SurfaceError> {
    let mut owned = String::new();
    owned
        .try_reserve_exact(text.len())
        .map_err(|_| SurfaceError::resource_exhausted())?;
    owned.push_str(text);
    Ok(owned)
}

enum Work<'a> {
    Eval(SurfaceTermId),
    PushLocal(&'a str),
    PopLocal,
    Build(BuildKind),
}

#[derive(Clone, Copy)]
enum BuildKind {
    Pi,
    Lam,
    App,
    Sigma,
    Pair,
    Fst,
    Snd,
    Id,
    Refl,
    J,
    EmptyElim,
    UnitElim,
    Succ,
    NatElim,
    Ann,
}
