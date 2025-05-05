mod connective;
mod truth_value;
mod base_formula;
mod truth_table;
mod tableau;

use base_formula::{Formula, FormulaRef};
use connective::Connective;
use truth_table::TruthTable;


fn main() {
    let p : FormulaRef = Formula::build_wrapped_atomic_wff("p".to_string());
    let q : FormulaRef  = Formula::build_wrapped_atomic_wff("q".to_string());
    let r : FormulaRef = Formula::build_wrapped_atomic_wff("r".to_string());
    let s : FormulaRef = Formula::build_wrapped_atomic_wff("s".to_string());
    let p_and_q : FormulaRef = Formula::build_wrapped_wff(Connective::And, &p, &q);
    let r_implies_p_and_q : FormulaRef = Formula::build_wrapped_wff(Connective::Imp, &r, &p_and_q);
    let neg_r_implies_p_and_q : FormulaRef = Formula::build_wrapped_wff(Connective::Not, &r_implies_p_and_q, &FormulaRef(None));
    let r_iff_p : FormulaRef = Formula::build_wrapped_wff(Connective::BiImp, &r, &p);
    let neg_r_implies_p_and_q_or_r_iff_p : FormulaRef = Formula::build_wrapped_wff(Connective::Or, &neg_r_implies_p_and_q, &r_iff_p);
    let neg_p : FormulaRef = Formula::build_wrapped_wff(Connective::Not, &p, &FormulaRef(None));
    let p_or_neg_p : FormulaRef = Formula::build_wrapped_wff(Connective::Or, &p, &neg_p);

    let base_atoms: Vec<&FormulaRef> = vec![&p, &q, &r];

    let mut truth_table : TruthTable = TruthTable::build_base_truth_table(base_atoms);
    truth_table.add_compound_formula(&p_and_q);
    truth_table.add_compound_formula(&r_implies_p_and_q);
    truth_table.add_compound_formula(&neg_r_implies_p_and_q);
    truth_table.add_compound_formula(&r_iff_p);
    truth_table.add_compound_formula(&neg_r_implies_p_and_q_or_r_iff_p);


    truth_table.print_final_truth_table();

}
