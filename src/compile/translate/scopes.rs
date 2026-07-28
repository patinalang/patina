use rustc_data_structures::fx::FxIndexMap;
use rustc_data_structures::smallvec::SmallVec;
use rustc_middle::bug;
use rustc_middle::thir::LocalVarId;
use rustc_span::Symbol;

use crate::compile::translate::BodyCx;

#[derive(Clone)]
pub(super) struct Scope(FxIndexMap<LocalVarId, PatinaVar>);

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) struct PatinaVar {
    pub name: Symbol,
    pub var_num: u32,
    pub vsn: u32,
    pub erl_name: Symbol,
    /// How many scope levels deep are we compared to the var's declaration?
    /// A depth of 0 means we're at the declaration scope.
    pub decl_depth: u32,
    /// How many scope levels deep are we compared to the var's initialization?
    /// A depth of 0 means we're at the initialization scope.
    /// `initialization_depth` is always less than or equal to `decl_depth`.
    /// `None` means the var is uninitialized.
    pub initialization_depth: Option<u32>,
}

impl PatinaVar {
    fn new(name: Symbol, var_num: u32, initialized: bool) -> Self {
        Self {
            name,
            var_num,
            vsn: 0,
            erl_name: Self::make_erl_name(name, var_num, 0),
            decl_depth: 0,
            initialization_depth: if initialized { Some(0) } else { None },
        }
    }

    fn with_bumped_vsn(&self, from_scope_depth: u32, initialized: bool) -> Self {
        Self {
            vsn: self.vsn + 1,
            erl_name: Self::make_erl_name(self.name, self.var_num, self.vsn + 1),
            decl_depth: self.decl_depth + from_scope_depth,
            initialization_depth: self
                .initialization_depth
                .map(|depth| depth + from_scope_depth)
                .or(initialized.then_some(0)),
            ..*self
        }
    }

    pub(super) fn initialized(&self) -> bool {
        self.initialization_depth.is_some()
    }

    fn make_erl_name(name: Symbol, var_num: u32, vsn: u32) -> Symbol {
        Symbol::intern(&format!("_{}@{}_{}", name, var_num, vsn))
    }
}

impl Scope {
    pub(super) fn new() -> Self {
        Self(FxIndexMap::default())
    }

    pub(super) fn get_var(&self, id: LocalVarId) -> Option<&PatinaVar> {
        self.0.get(&id)
    }

    fn bind_var(&mut self, id: LocalVarId, var: PatinaVar) -> &PatinaVar {
        self.0.entry(id).insert_entry(var).into_mut()
    }
}

pub(crate) struct Scopes {
    scopes: SmallVec<[Scope; 8]>,
    next_var_num: u32,
}

impl Scopes {
    pub(super) fn new() -> Self {
        let mut scopes = SmallVec::new();
        scopes.push(Scope::new());
        Self { scopes, next_var_num: 0 }
    }

    pub(super) fn get_var(&self, id: LocalVarId) -> Option<&PatinaVar> {
        self.scopes.iter().rev().find_map(|scope| scope.get_var(id))
    }

    pub(super) fn fetch_var(&self, id: LocalVarId) -> &PatinaVar {
        self.get_var(id).unwrap_or_else(|| bug!("variable {:?} is not in scope", id))
    }

    pub(super) fn fresh_var(&mut self, name: Symbol, initialized: bool) -> PatinaVar {
        let var = PatinaVar::new(name, self.next_var_num, initialized);
        self.next_var_num += 1;
        var
    }

    pub(super) fn bind_var(
        &mut self,
        id: LocalVarId,
        name: Symbol,
        initialized: bool,
    ) -> &PatinaVar {
        let var = match self
            .scopes
            .iter()
            .rev()
            .enumerate()
            .find_map(|(idx, scope)| scope.get_var(id).map(|var| (idx, var)))
        {
            Some((idx, var)) => var.with_bumped_vsn(idx as u32, initialized),
            None => self.fresh_var(name, initialized),
        };

        self.last_scope_mut().bind_var(id, var)
    }

    pub(super) fn rebind_var(&mut self, id: LocalVarId, initialized: bool) -> &PatinaVar {
        let name = self.fetch_var(id).name;
        self.bind_var(id, name, initialized)
    }

    pub(super) fn push(&mut self) {
        self.scopes.push(Scope::new());
    }

    pub(super) fn pop(&mut self) -> Scope {
        self.scopes.pop().expect("scopes should never be empty")
    }

    /// Returns the union of all outer variables that are modified by the branch scopes.
    /// The map value is the latest version of the variable between all branches.
    pub(super) fn modified_outer_vars<'a>(
        &mut self,
        branches: impl IntoIterator<Item = (&'a Scope, bool)>,
    ) -> FxIndexMap<LocalVarId, PatinaVar> {
        let mut modified_outer_vars = FxIndexMap::default();
        let mut num_used = FxIndexMap::default();
        let mut num_non_diverging_branches = 0;

        for (branch, always_diverges) in branches {
            if !always_diverges {
                num_non_diverging_branches += 1;
            }

            for (id, branch_var) in &branch.0 {
                if branch_var.decl_depth == 0 {
                    continue;
                }

                let decl_depth = branch_var.decl_depth - 1;
                let initialization_depth = match branch_var.initialization_depth {
                    Some(0) => {
                        if !always_diverges {
                            num_used.entry(*id).and_modify(|count| *count += 1).or_insert(1);
                        }
                        None
                    }
                    Some(depth) => Some(depth - 1),
                    None => bug!("outer var {:?} is not initialized in branch", id),
                };

                let branch_var = PatinaVar { decl_depth, initialization_depth, ..*branch_var };

                modified_outer_vars
                    .entry(*id)
                    .and_modify(|var: &mut PatinaVar| {
                        if var.vsn < branch_var.vsn {
                            *var = branch_var;
                        }
                    })
                    .or_insert(branch_var);
            }
        }

        for (id, count) in num_used {
            if count == num_non_diverging_branches {
                // Initialized in all non-diverging branches, so it's fully initialized outside of the branches.
                let var = modified_outer_vars.get_mut(&id).unwrap_or_else(|| {
                    bug!("variable {id:?} present in num_used but not in modified_outer_vars")
                });
                if var.initialization_depth.is_some() {
                    bug!("variable {id:?} already initialized");
                }
                var.initialization_depth = Some(0);
            }
        }

        modified_outer_vars
    }

    /// Merges the scopes of the branches into a single after-branch scope.
    pub(super) fn join_branches(
        &mut self,
        modified_outer_vars: &FxIndexMap<LocalVarId, PatinaVar>,
    ) {
        for (id, var) in modified_outer_vars {
            self.last_scope_mut().0.insert(*id, *var);
        }
    }

    fn last_scope_mut(&mut self) -> &mut Scope {
        self.scopes.last_mut().expect("scopes should never be empty")
    }
}

impl<'a, 'tcx: 'arena, 'arena> BodyCx<'a, 'tcx, 'arena> {
    pub(super) fn with_scope<R>(&mut self, f: impl FnOnce(&mut Self) -> R) -> (R, Scope) {
        self.scopes.push();
        let result = f(self);
        let scope = self.scopes.pop();
        (result, scope)
    }
}
