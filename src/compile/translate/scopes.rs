use rustc_data_structures::fx::FxIndexMap;
use rustc_data_structures::smallvec::SmallVec;
use rustc_middle::bug;
use rustc_middle::erl::etir::VarId;
use rustc_span::Symbol;

use crate::compile::translate::BodyCx;

#[derive(Clone)]
pub(super) struct Scope(FxIndexMap<VarId, PatinaVar>);

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) struct PatinaVar {
    pub name: Symbol,
    pub var_num: u32,
    pub vsn: u32,
    pub erl_name: Symbol,
    /// How many scope levels deep are we compared to the var's declaration?
    /// A depth of 0 means we're at the declaration scope.
    pub decl_depth: u32,
}

impl PatinaVar {
    fn new(name: Symbol, var_num: u32) -> Self {
        Self {
            name,
            var_num,
            vsn: 0,
            erl_name: Self::make_erl_name(name, var_num, 0),
            decl_depth: 0,
        }
    }

    fn with_bumped_vsn(&self, from_scope_depth: u32) -> Self {
        Self {
            vsn: self.vsn + 1,
            erl_name: Self::make_erl_name(self.name, self.var_num, self.vsn + 1),
            decl_depth: self.decl_depth + from_scope_depth,
            ..*self
        }
    }

    fn make_erl_name(name: Symbol, var_num: u32, vsn: u32) -> Symbol {
        Symbol::intern(&format!("_{}@{}_{}", name, var_num, vsn))
    }
}

impl Scope {
    pub(super) fn new() -> Self {
        Self(FxIndexMap::default())
    }

    pub(super) fn get_var(&self, id: VarId) -> Option<&PatinaVar> {
        self.0.get(&id)
    }

    fn bind_var(&mut self, id: VarId, var: PatinaVar) -> &PatinaVar {
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

    pub(super) fn get_var(&self, id: VarId) -> Option<&PatinaVar> {
        self.scopes.iter().rev().find_map(|scope| scope.get_var(id))
    }

    pub(super) fn fetch_var(&self, id: VarId) -> &PatinaVar {
        self.get_var(id).unwrap_or_else(|| bug!("variable {:?} is not in scope", id))
    }

    pub(super) fn fresh_var(&mut self, name: Symbol) -> PatinaVar {
        let var = PatinaVar::new(name, self.next_var_num);
        self.next_var_num += 1;
        var
    }

    pub(super) fn bind_var(&mut self, id: VarId, name: Symbol) -> &PatinaVar {
        let var = match self
            .scopes
            .iter()
            .rev()
            .enumerate()
            .find_map(|(idx, scope)| scope.get_var(id).map(|var| (idx, var)))
        {
            Some((idx, var)) => var.with_bumped_vsn(idx as u32),
            None => self.fresh_var(name),
        };

        self.last_scope_mut().bind_var(id, var)
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
        branches: impl IntoIterator<Item = &'a Scope>,
    ) -> FxIndexMap<VarId, PatinaVar> {
        let mut modified_outer_vars = FxIndexMap::default();
        for branch in branches {
            for (id, branch_var) in &branch.0 {
                if branch_var.decl_depth == 0 {
                    continue;
                }

                let decl_depth = branch_var.decl_depth - 1;
                let branch_var = PatinaVar { decl_depth, ..*branch_var };

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

        modified_outer_vars
    }

    /// Merges the scopes of the branches into a single after-branch scope.
    pub(super) fn join_branches(&mut self, modified_outer_vars: &FxIndexMap<VarId, PatinaVar>) {
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
