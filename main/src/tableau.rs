use crate::truth_value::{self, TruthValue};
use crate::connective::Connective;
use crate::base_formula::FormulaRef;
use std::collections::HashMap;

// Provide proof tree for formulas
// Proof by contradiction, if all branches closes, input is contradictory

// Starts with a vector of formulas to be deconstructed
// Apply rules untill only atoms are left

// Rules can branch out 

struct ProofNode {
    compound_formulas : Vec<FormulaRef>,
    model : HashMap<String, TruthValue>,
    left_branch : Option<Box<ProofNode>>,
    right_branch : Option<Box<ProofNode>>,
}

impl ProofNode {
    fn build_starting_node(starting_formulas: Vec<FormulaRef>, starting_truth_values: Vec<TruthValue>) -> ProofNode {
        if starting_formulas.len() != starting_truth_values.len() {
            panic!("Wrong assignment of truth values")
        }

        let mut new_model : HashMap<String, TruthValue> = HashMap::new();

        for (index, formula) in starting_formulas.iter().enumerate(){
            let current_formula_repr = formula.repr();

            new_model.insert(current_formula_repr, starting_truth_values[index]);
        }

        return ProofNode { 
            compound_formulas: starting_formulas,
            model: new_model,
            left_branch: None,
            right_branch: None 
        }
    }
}

struct Tableau {
    starting_node : ProofNode
}

impl Tableau {
    
}