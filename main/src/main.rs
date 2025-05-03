mod connective;
mod truth_value;
mod base_formula;
mod truth_table;

use base_formula::{Formula, FormulaRef};
use connective::Connective;
use truth_value::TruthValue;
use truth_table::TruthTable;


fn main() {
    let p : FormulaRef = Formula::build_wrapped_atomic_wff("p".to_string());
    let q : FormulaRef  = Formula::build_wrapped_atomic_wff("q".to_string());
    let r : FormulaRef = Formula::build_wrapped_atomic_wff("r".to_string());
    let s : FormulaRef = Formula::build_wrapped_atomic_wff("s".to_string());
    let p_and_q : FormulaRef = Formula::build_wrapped_wff(Connective::And, p.clone(), q.clone());
    let r_implies_p_and_q : FormulaRef = Formula::build_wrapped_wff(Connective::Imp, r.clone(), p_and_q.clone());
    let neg_r_implies_p_and_q : FormulaRef = Formula::build_wrapped_wff(Connective::Not, r_implies_p_and_q.clone(), FormulaRef(None));
    let r_iff_p : FormulaRef = Formula::build_wrapped_wff(Connective::BiImp, r.clone(), p.clone());
    let neg_r_implies_p_and_q_or_r_iff_p : FormulaRef = Formula::build_wrapped_wff(Connective::Or, neg_r_implies_p_and_q.clone(), r_iff_p.clone());
    let neg_p : FormulaRef = Formula::build_wrapped_wff(Connective::Not, p.clone(), FormulaRef(None));
    let p_or_neg_p : FormulaRef = Formula::build_wrapped_wff(Connective::Or, p.clone(), neg_p.clone());

    let base_atoms: Vec<FormulaRef> = vec![p.clone(), q.clone(), r.clone()];

    let mut truth_table : TruthTable = TruthTable::build_base_truth_table(base_atoms);
    truth_table.add_compound_formula(p_and_q.clone());
    truth_table.add_compound_formula(r_implies_p_and_q.clone());
    truth_table.add_compound_formula(neg_r_implies_p_and_q.clone());
    truth_table.add_compound_formula(r_iff_p.clone());
    truth_table.add_compound_formula(neg_r_implies_p_and_q_or_r_iff_p.clone());


    truth_table.print_final_truth_table();

}
