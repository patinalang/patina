//! Delete-free planning: obtain the goal facts with the cheapest set of actions.
//!
//! The implementation uses a solver to solve a linear problem for the cheapest set.
//!
//! The linear problem is constructed mostly as defined in Rankooh, Rintanen (2022) as IP-TL in
//! "Efficient Computation and Informative Estimation of h+ by Integer and Linear Programming".
//! Even though that paper highlights the vertex-elimination model, we don't do that here;
//! our problems are likely small enough that it doesn't matter.
//!
//! The other paper used for this is Imai, Fukunaga (2015)
//! "On a Practical, Integer-Linear Programming Model for Delete-Free Tasks and its Use
//!  as a Heuristic for Cost-Optimal Planning".
//! mostly for pre-processing: relevance analysis and dominated action elimination.
//! Here, we don't use landmarks, so our pruning is weaker.

use good_lp::{
    Expression, ProblemVariables, Solution, SolverModel, Variable, constraint, microlp, variable,
};
use rustc_data_structures::fx::{FxHashMap, FxHashSet, FxIndexMap};
use rustc_index::{IndexVec, newtype_index};
use rustc_middle::bug;

newtype_index! {
    #[orderable]
    #[debug_format = "fact{}"]
    pub(super) struct FactId {}
}

newtype_index! {
    #[orderable]
    #[debug_format = "action{}"]
    pub(super) struct ActionId {}
}

#[derive(Default)]
struct Fact {
    /// Any of these actions individually can obtain this `Fact`.
    achievers: Vec<ActionId>,
}

/// Obtains all of its `effects` for `cost` once all of `preconditions` are obtained.
struct Action {
    preconditions: Box<[FactId]>,
    effects: Vec<FactId>,
    cost: u32,
}

/// A delete-free STRIPS planning task.
/// Delete-free implies that once a fact is obtained, it stays.
pub(super) struct Problem {
    facts: IndexVec<FactId, Fact>,
    actions: IndexVec<ActionId, Action>,
}

impl Problem {
    pub(super) fn new() -> Self {
        Problem { facts: IndexVec::new(), actions: IndexVec::new() }
    }

    pub(super) fn push_fact(&mut self) -> FactId {
        self.facts.push(Fact::default())
    }

    pub(super) fn push_action(
        &mut self,
        mut preconditions: Vec<FactId>,
        effects: Vec<FactId>,
        cost: u32,
    ) -> ActionId {
        preconditions.sort_unstable();
        preconditions.dedup();

        let action = self.actions.push(Action {
            preconditions: preconditions.into_boxed_slice(),
            effects,
            cost,
        });
        for &fact in &self.actions[action].effects {
            self.facts[fact].achievers.push(action);
        }
        action
    }

    /// Makes `action` also obtain `effects`, which may include facts it obtains already.
    pub(super) fn add_effects(
        &mut self,
        action: ActionId,
        effects: impl IntoIterator<Item = FactId>,
    ) {
        for fact in effects {
            self.actions[action].effects.push(fact);
            self.facts[fact].achievers.push(action);
        }
    }

    /// For given `goals`, return the cheapest plan of how to fulfill it.
    /// The plan comprises goals and intermediate facts, with information on how each is achieved.
    /// A fact missing from the plan wasn't needed to fulfill `goals`.
    /// Alternatively, it was zero-cost and the solver picked it because why not.
    ///
    /// We have no tie-breaks, and we rely on the solver returning a deterministic output
    /// within the solver's version. Best I can tell, `microlp` gives us that that as of writing.
    ///
    /// Solves IP-TL linear problem as defined by Rankooh, Rintanen (2022)
    /// "Efficient Computation and Informative Estimation of h+ by Integer and Linear Programming".
    /// Every fact gets a time, and every action in the plan must obtain its effects later than
    /// its preconditions.
    /// Even though that paper highlights the vertex-elimination model, we don't do that here;
    /// our problems are likely small enough that it doesn't matter.
    pub(super) fn solve(&mut self, goals: &[FactId]) -> FxIndexMap<FactId, ActionId> {
        for action in &mut self.actions {
            action.effects.sort_unstable();
            action.effects.dedup();
        }
        for fact in &mut self.facts {
            fact.achievers.sort_unstable();
            fact.achievers.dedup();
        }

        let relevant = self.relevant_achievers(goals);

        // All facts can only be obtained one way, so there's nothing to choose.
        // This also covers `goals: []`, if we would ever produce it.
        if relevant.values().all(|achievers| achievers.len() == 1) {
            return relevant.into_iter().map(|(fact, achievers)| (fact, achievers[0])).collect();
        }

        let mut num_relevant_effects = FxIndexMap::<ActionId, usize>::default();
        for achievers in relevant.values() {
            for &action in achievers {
                *num_relevant_effects.entry(action).or_default() += 1;
            }
        }

        let mut problem_vars = ProblemVariables::new();
        // Rankooh, Rintanen (2022): `|P|`
        let num_facts = relevant.len() as f64;

        // Rankooh, Rintanen (2022): `f_p`
        let supported: FxIndexMap<FactId, Variable> =
            relevant.keys().map(|&fact| (fact, problem_vars.add(variable().binary()))).collect();

        // Rankooh, Rintanen (2022): `t_i`
        let time: FxIndexMap<FactId, Variable> = relevant
            .keys()
            .map(|&fact| (fact, problem_vars.add(variable().integer().min(1).max(num_facts))))
            .collect();

        // Rankooh, Rintanen (2022): `f_a`
        let used: FxIndexMap<ActionId, Variable> = num_relevant_effects
            .keys()
            .map(|&action| (action, problem_vars.add(variable().binary())))
            .collect();

        // Whether each achiever obtains its fact in the plan.
        // Rankooh, Rintanen (2022): `f_{p,a}`
        let achieves: FxHashMap<(FactId, ActionId), Variable> = relevant
            .iter()
            .flat_map(|(&fact, achievers)| achievers.iter().map(move |&action| (fact, action)))
            .map(|(fact, action)| {
                if num_relevant_effects[&action] == 1 {
                    ((fact, action), used[&action])
                } else {
                    ((fact, action), problem_vars.add(variable().binary()))
                }
            })
            .collect();

        // Rankooh, Rintanen (2022): Equation (1)
        let cost_expr: Expression =
            used.iter().map(|(&action, &used)| f64::from(self.actions[action].cost) * used).sum();

        let mut model = problem_vars.minimise(cost_expr).using(microlp);

        // Rankooh, Rintanen (2022): Equation (4)
        for goal in goals {
            model.add_constraint(constraint!(supported[goal] == 1));
        }
        for (&fact, achievers) in &relevant {
            // A fact is obtained by exactly one achiever if it is obtained at all.
            // Rankooh, Rintanen (2022): Equation (2)
            let obtained: Expression =
                achievers.iter().map(|&action| achieves[&(fact, action)]).sum();
            model.add_constraint(constraint!(supported[&fact] == obtained));

            // The achievers obtaining `fact` that need each precondition.
            let mut users = FxIndexMap::<FactId, Expression>::default();
            for &action in achievers {
                let achieves = achieves[&(fact, action)];

                if num_relevant_effects[&action] > 1 {
                    // An achiever obtains its fact only if its action is used.
                    // Rankooh, Rintanen (2022): Equation (5)
                    model.add_constraint(constraint!(achieves <= used[&action]));
                }

                for precondition in &self.actions[action].preconditions {
                    *users.entry(*precondition).or_default() += achieves;

                    // When `achieves`, `time[precondition] < time[fact]`.
                    // Rankooh, Rintanen (2022): Equation (9)
                    model.add_constraint(constraint!(
                        time[precondition] - time[&fact] + 1 <= num_facts * (1 - achieves)
                    ));
                }
            }

            for (precondition, users) in users {
                // If `users` is 1, the precondition must also be 1
                // Rankooh, Rintanen (2022): Equation (3)
                model.add_constraint(constraint!(users <= supported[&precondition]));
            }
        }

        let solution = model.solve().unwrap_or_else(|err| bug!("failed to plan: {err}"));
        relevant
            .into_iter()
            .filter_map(|(fact, achievers)| {
                let &action = achievers
                    .iter()
                    .find(|&&action: &&ActionId| solution.value(achieves[&(fact, action)]) > 0.5)?;
                Some((fact, action))
            })
            .collect()
    }

    fn relevant_achievers(&self, goals: &[FactId]) -> FxIndexMap<FactId, Box<[ActionId]>> {
        let relevant = self.backchain(goals, |_| true);

        let mut dominated = FxHashSet::default();
        for achievers in relevant.values() {
            for &action in achievers {
                if achievers.iter().any(|&other| self.dominates(other, action, &relevant)) {
                    dominated.insert(action);
                }
            }
        }

        self.backchain(goals, |action| !dominated.contains(&action))
    }

    fn backchain(
        &self,
        goals: &[FactId],
        keep: impl Fn(ActionId) -> bool,
    ) -> FxIndexMap<FactId, Box<[ActionId]>> {
        let mut relevant = FxIndexMap::default();
        let mut pending = goals.to_vec();
        while let Some(fact) = pending.pop() {
            if relevant.contains_key(&fact) {
                continue;
            }

            let achievers: Box<[_]> =
                self.facts[fact].achievers.iter().copied().filter(|&action| keep(action)).collect();

            for &action in &achievers {
                pending.extend(&self.actions[action].preconditions);
            }

            relevant.insert(fact, achievers);
        }
        relevant
    }

    /// Check if one action A dominates another B, i.e. A obtains all `relevant` facts that B
    /// obtains, while B requires the same preconditions as A (and more?), and costs at least
    /// as much. The id acts as a tiebreak if actions dominate each other.
    ///
    /// A weaker version of a check from Imai, Fukunaga (2015).
    fn dominates(
        &self,
        action_id: ActionId,
        other_id: ActionId,
        relevant: &FxIndexMap<FactId, Box<[ActionId]>>,
    ) -> bool {
        fn replaces(
            a: &Action,
            b: &Action,
            relevant: &FxIndexMap<FactId, Box<[ActionId]>>,
        ) -> bool {
            a.cost <= b.cost
                && a.preconditions.iter().all(|fact| b.preconditions.contains(fact))
                && b.effects
                    .iter()
                    .all(|fact| !relevant.contains_key(fact) || a.effects.contains(fact))
        }

        let (action, other) = (&self.actions[action_id], &self.actions[other_id]);
        action_id != other_id
            && replaces(action, other, relevant)
            && (action_id < other_id || !replaces(other, action, relevant))
    }
}
