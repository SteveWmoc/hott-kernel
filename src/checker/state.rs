use super::transform::shift;
use super::{CheckError, ReferenceKind};
use crate::syntax::{Arena, Declaration, Natural, TermId};
use std::collections::VecDeque;

/// Newest-first local telescope used by the bidirectional checker.
///
/// Stored types are the types validated immediately before their entries were
/// introduced. Looking up local index `n` therefore weakens the stored type by
/// exactly `n + 1`, as frozen in Core v0.1 Section 4.
#[derive(Debug, Default)]
pub(super) struct LocalContext {
    entries: VecDeque<TermId>,
}

impl LocalContext {
    pub(super) const fn new() -> Self {
        Self {
            entries: VecDeque::new(),
        }
    }

    pub(super) fn len(&self) -> usize {
        self.entries.len()
    }

    pub(super) fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Extend the context with a type that has already been validated in the
    /// current context.
    pub(super) fn push_checked(
        &mut self,
        declaration_index: usize,
        ty: TermId,
    ) -> Result<(), CheckError> {
        self.entries
            .try_reserve(1)
            .map_err(|_| CheckError::resource_exhausted(declaration_index))?;
        self.entries.push_front(ty);
        Ok(())
    }

    pub(super) fn pop(&mut self) -> Option<TermId> {
        self.entries.pop_front()
    }

    /// Return the type of a local variable at its current de Bruijn depth.
    ///
    /// `use_site` is retained only for stable diagnostic attribution if the
    /// index is outside the current telescope.
    pub(super) fn lookup(
        &self,
        arena: &mut Arena,
        declaration_index: usize,
        use_site: TermId,
        index: &Natural,
    ) -> Result<TermId, CheckError> {
        let index = index
            .to_usize()
            .filter(|index| *index < self.entries.len())
            .ok_or_else(|| {
                CheckError::invalid_reference(declaration_index, use_site, ReferenceKind::Local)
            })?;
        let stored = self.entries[index];
        let increment = index
            .checked_add(1)
            .ok_or_else(|| CheckError::resource_exhausted(declaration_index))?;
        shift(arena, stored, increment, 0)
            .map_err(|_| CheckError::resource_exhausted(declaration_index))
    }
}

/// Declaration kind retained by the checked global environment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum GlobalKind {
    Postulate,
    Transparent,
    Opaque,
}

/// One declaration that has already passed all checker obligations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct GlobalEntry {
    kind: GlobalKind,
    ty: TermId,
    body: Option<TermId>,
}

impl GlobalEntry {
    fn from_declaration(declaration: &Declaration) -> Self {
        match declaration {
            Declaration::Postulate { ty, .. } => Self {
                kind: GlobalKind::Postulate,
                ty: *ty,
                body: None,
            },
            Declaration::Transparent { ty, body, .. } => Self {
                kind: GlobalKind::Transparent,
                ty: *ty,
                body: Some(*body),
            },
            Declaration::Opaque { ty, body, .. } => Self {
                kind: GlobalKind::Opaque,
                ty: *ty,
                body: Some(*body),
            },
        }
    }

    pub(super) const fn kind(self) -> GlobalKind {
        self.kind
    }

    pub(super) const fn ty(self) -> TermId {
        self.ty
    }

    pub(super) const fn body(self) -> Option<TermId> {
        self.body
    }

    /// Body available to delta reduction.
    ///
    /// Opaque bodies remain retained for checking and audit extraction but are
    /// never exposed operationally.
    pub(super) const fn unfolding_body(self) -> Option<TermId> {
        match self.kind {
            GlobalKind::Transparent => self.body,
            GlobalKind::Postulate | GlobalKind::Opaque => None,
        }
    }
}

/// Prefix of declarations that have successfully passed checker validation.
///
/// The current declaration is absent until [`CheckedGlobals::commit`] is
/// called, making self and forward references unavailable by construction.
#[derive(Debug, Default)]
pub(super) struct CheckedGlobals {
    entries: Vec<GlobalEntry>,
}

impl CheckedGlobals {
    pub(super) const fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Zero-based index of the declaration currently being checked.
    pub(super) fn next_declaration_index(&self) -> usize {
        self.entries.len()
    }

    /// Insert a declaration only after its type and optional body have passed
    /// all checker obligations.
    pub(super) fn commit(&mut self, declaration: &Declaration) -> Result<(), CheckError> {
        let declaration_index = self.next_declaration_index();
        self.entries
            .try_reserve(1)
            .map_err(|_| CheckError::resource_exhausted(declaration_index))?;
        self.entries
            .push(GlobalEntry::from_declaration(declaration));
        Ok(())
    }

    pub(super) fn lookup(
        &self,
        use_site: TermId,
        index: &Natural,
    ) -> Result<GlobalEntry, CheckError> {
        let declaration_index = self.next_declaration_index();
        index
            .to_usize()
            .and_then(|index| self.entries.get(index).copied())
            .ok_or_else(|| {
                CheckError::invalid_reference(declaration_index, use_site, ReferenceKind::Global)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::{CheckedGlobals, GlobalKind, LocalContext};
    use crate::checker::{CheckErrorClass, ReferenceKind};
    use crate::syntax::{Arena, Declaration, Natural, Term};

    fn natural(text: &str) -> Natural {
        Natural::from_decimal(text).expect("test natural is canonical")
    }

    #[test]
    fn local_lookup_is_newest_first_and_applies_frozen_weakening() {
        let mut arena = Arena::new();
        let nat = arena.push(Term::Nat).unwrap();
        let var_zero = arena.push(Term::Var(natural("0"))).unwrap();
        let dependent = arena.push(Term::Id(nat, var_zero, var_zero)).unwrap();
        let use_site = arena.push(Term::Var(natural("0"))).unwrap();

        let mut context = LocalContext::new();
        context.push_checked(0, nat).unwrap();
        context.push_checked(0, dependent).unwrap();

        assert_eq!(context.len(), 2);
        let newest = context
            .lookup(&mut arena, 0, use_site, &natural("0"))
            .unwrap();
        let Term::Id(newest_ty, left, right) = arena.get(newest).unwrap() else {
            panic!("dependent lookup must remain an identity type");
        };
        assert_eq!(*newest_ty, nat);
        for endpoint in [*left, *right] {
            let Term::Var(index) = arena.get(endpoint).unwrap() else {
                panic!("weakened endpoint must remain a variable");
            };
            assert_eq!(index.as_str(), "1");
        }

        let older = context
            .lookup(&mut arena, 0, use_site, &natural("1"))
            .unwrap();
        assert_eq!(older, nat, "closed stored types are reused after weakening");

        assert_eq!(context.pop(), Some(dependent));
        assert_eq!(context.pop(), Some(nat));
        assert!(context.is_empty());
    }

    #[test]
    fn local_lookup_rejects_out_of_scope_and_unbounded_indices() {
        let mut arena = Arena::new();
        let nat = arena.push(Term::Nat).unwrap();
        let use_site = arena.push(Term::Var(natural("1"))).unwrap();
        let mut context = LocalContext::new();
        context.push_checked(7, nat).unwrap();

        for index in [natural("1"), natural(&"9".repeat(512))] {
            let error = context.lookup(&mut arena, 7, use_site, &index).unwrap_err();
            assert_eq!(error.class(), CheckErrorClass::InvalidJudgment);
            assert_eq!(error.declaration_index(), 7);
            assert_eq!(error.term_id(), Some(use_site));
            assert_eq!(error.reference_kind(), Some(ReferenceKind::Local));
        }
    }

    #[test]
    fn globals_become_visible_only_after_commit() {
        let mut arena = Arena::new();
        let nat = arena.push(Term::Nat).unwrap();
        let zero = arena.push(Term::Zero).unwrap();
        let use_site = arena.push(Term::Global(natural("0"))).unwrap();
        let declaration = Declaration::Transparent {
            name: "z".to_owned(),
            ty: nat,
            body: zero,
        };
        let mut globals = CheckedGlobals::new();

        let error = globals.lookup(use_site, &natural("0")).unwrap_err();
        assert_eq!(error.class(), CheckErrorClass::InvalidJudgment);
        assert_eq!(error.declaration_index(), 0);
        assert_eq!(error.reference_kind(), Some(ReferenceKind::Global));

        globals.commit(&declaration).unwrap();
        assert_eq!(globals.next_declaration_index(), 1);
        let entry = globals.lookup(use_site, &natural("0")).unwrap();
        assert_eq!(entry.kind(), GlobalKind::Transparent);
        assert_eq!(entry.ty(), nat);
        assert_eq!(entry.body(), Some(zero));
        assert_eq!(entry.unfolding_body(), Some(zero));

        let self_reference = globals.lookup(use_site, &natural("1")).unwrap_err();
        assert_eq!(self_reference.declaration_index(), 1);
        assert_eq!(self_reference.reference_kind(), Some(ReferenceKind::Global));
    }

    #[test]
    fn only_transparent_globals_expose_delta_bodies() {
        let mut arena = Arena::new();
        let nat = arena.push(Term::Nat).unwrap();
        let zero = arena.push(Term::Zero).unwrap();
        let use_site = arena.push(Term::Global(natural("0"))).unwrap();
        let declarations = [
            Declaration::Postulate {
                name: "p".to_owned(),
                ty: nat,
            },
            Declaration::Opaque {
                name: "o".to_owned(),
                ty: nat,
                body: zero,
            },
            Declaration::Transparent {
                name: "t".to_owned(),
                ty: nat,
                body: zero,
            },
        ];
        let mut globals = CheckedGlobals::new();
        for declaration in &declarations {
            globals.commit(declaration).unwrap();
        }

        let postulate = globals.lookup(use_site, &natural("0")).unwrap();
        assert_eq!(postulate.kind(), GlobalKind::Postulate);
        assert_eq!(postulate.body(), None);
        assert_eq!(postulate.unfolding_body(), None);

        let opaque = globals.lookup(use_site, &natural("1")).unwrap();
        assert_eq!(opaque.kind(), GlobalKind::Opaque);
        assert_eq!(opaque.body(), Some(zero));
        assert_eq!(opaque.unfolding_body(), None);

        let transparent = globals.lookup(use_site, &natural("2")).unwrap();
        assert_eq!(transparent.kind(), GlobalKind::Transparent);
        assert_eq!(transparent.unfolding_body(), Some(zero));
    }

    #[test]
    fn global_lookup_rejects_machine_unbounded_indices() {
        let mut arena = Arena::new();
        let nat = arena.push(Term::Nat).unwrap();
        let use_site = arena.push(Term::Global(natural("0"))).unwrap();
        let declaration = Declaration::Postulate {
            name: "p".to_owned(),
            ty: nat,
        };
        let mut globals = CheckedGlobals::new();
        globals.commit(&declaration).unwrap();

        let huge = natural(&"9".repeat(512));
        let error = globals.lookup(use_site, &huge).unwrap_err();
        assert_eq!(error.class(), CheckErrorClass::InvalidJudgment);
        assert_eq!(error.declaration_index(), 1);
        assert_eq!(error.term_id(), Some(use_site));
        assert_eq!(error.reference_kind(), Some(ReferenceKind::Global));
    }
}
