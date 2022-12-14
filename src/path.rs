use std::{
    collections::{BTreeMap, HashMap},
    fmt::Display,
};

use anyhow::Result;

use crate::{
    executor_state::SymType,
    expr::{Expr, ExprNode},
    probability::Prob,
};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Path {
    conds: Vec<Expr>,
    prob: Prob,
    terminated: bool,
    sigma: BTreeMap<String, ExprNode>,
    observations: Vec<Expr>,
}

impl Path {
    pub fn new() -> Self {
        Path {
            conds: Vec::new(),
            prob: Prob::init_dist(),
            terminated: false,
            sigma: BTreeMap::new(),
            observations: Vec::new(),
        }
    }

    pub fn branch(&mut self, cond: Expr, sigma: &HashMap<String, ExprNode>) {
        self.conds.push(cond);
        self.merge_sigma(sigma);
    }

    pub fn observe(&mut self, observation: Expr) {
        self.observations.push(observation);
    }

    pub fn merge_sigma(&mut self, sigma: &HashMap<String, ExprNode>) {
        self.sigma
            .extend(sigma.into_iter().map(|(k, v)| (k.clone(), v.clone())));
    }

    pub fn get_conds(&self) -> &Vec<Expr> {
        &self.conds
    }

    pub fn calculate_prob(&mut self, sym_vars: &HashMap<String, SymType>) -> Result<()> {
        self.prob = Prob::new(self, sym_vars)?;
        Ok(())
    }

    pub fn mark_terminated(&mut self) {
        self.terminated = true;
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sigma_str = self
            .sigma
            .iter()
            .map(|(k, v)| format!("σ({k}) = {v}"))
            .collect::<Vec<String>>()
            .join(", ");
        if self.terminated {
            write!(
                f,
                "Path Condition: {}\n\tObservations: {}\n\tProbability: {}\n\tSigma: {}\n\tTerminated: Yes",
                self.conds
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(" ∧ "),
                self.observations
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(" ∧ "),
                self.prob,
                sigma_str
            )
        } else {
            write!(
                f,
                "Path Condition: {}\n\tObservations: {}\n\tProbability: {}\n\tSigma: {}\n\tTerminated: No",
                self.conds
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(" ∧ "),
                self.observations
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(" ∧ "),
                self.prob,
                sigma_str
            )
        }
    }
}
