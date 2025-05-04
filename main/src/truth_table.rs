use crate::truth_value::TruthValue;
use crate::connective::Connective;
use crate::base_formula::FormulaRef;
use std::collections::HashMap;

pub struct TruthTable {
    atoms: Vec<FormulaRef>,
    compound_formulas: Vec<FormulaRef>, 
}

impl TruthTable {
    pub fn build_base_truth_table(atoms: Vec<&FormulaRef>) -> TruthTable{
        let cloned_atoms: Vec<FormulaRef> = atoms.iter().map(|atom_ref: &&FormulaRef| (*atom_ref).clone()).collect();

        return TruthTable{
            atoms: cloned_atoms, 
            compound_formulas: Vec::new()
        }; 
    }

    fn amount_of_atoms(&self) -> i32 {
        return self.atoms.len().try_into().unwrap();
    }

    fn amount_of_compound_formulas(&self) -> i32 {
        return self.compound_formulas.len().try_into().unwrap();
    }

    fn columns_heads(&self) -> Vec<FormulaRef>{
        let mut columns_refs = self.atoms.clone();
        columns_refs.extend(self.compound_formulas.clone());

        return columns_refs;
    }

    fn columns_heads_repr(&self) -> Vec<String>{
        let mut columns_refs = self.atoms.clone();
        columns_refs.extend(self.compound_formulas.clone());

        return columns_refs.iter().map(
            |formula_ref| {
                match formula_ref.option() {
                    Some(rc_f) => rc_f.repr.clone(),
                    None => "".to_string(),
                }
            }
        ).collect::<Vec<String>>();
    }

    fn formulas_positions_in_columns(&self) -> HashMap<String, i32> {
        let mut formulas_positions_in_columns : HashMap<String, i32> = HashMap::new();

        for (i, formula) in self.columns_heads().iter().enumerate(){
            let formula_repr = formula.repr();
            let formula_position : i32 = i.try_into().unwrap();

            formulas_positions_in_columns.insert(formula_repr, formula_position);
        }

        return formulas_positions_in_columns;
    }

    pub fn add_compound_formula(&mut self, compound_formula: &FormulaRef){
        let formulas_positions_in_columns: HashMap<String, i32> = self.formulas_positions_in_columns();
        let cloned_coumpond_formula = compound_formula.clone();

        match cloned_coumpond_formula.main_connective(){
            Connective::Not => {
                if None == formulas_positions_in_columns.get(&cloned_coumpond_formula.left_subformula().repr()){
                    panic!("Subformulas of Formula [{}] are not present in this TruthTable", cloned_coumpond_formula.repr());
                }
            },
            _ => {
                if None == formulas_positions_in_columns.get(&cloned_coumpond_formula.left_subformula().repr())
                || None == formulas_positions_in_columns.get(&cloned_coumpond_formula.right_subformula().repr()){
                    panic!("Subformulas of Formula [{}] are not present in this TruthTable", cloned_coumpond_formula.repr());
                }
            }
        }
        

        self.compound_formulas.push(cloned_coumpond_formula);
    }

    // fn amount_of_value_rows(&self) -> i32 {
    //     return 2i32.pow(self.atoms.len().try_into().unwrap());
    // }

    // fn columns_heads_string(&self) -> String {
    //     return self.columns_heads_repr().join(" | ");
    // }

    // fn amount_of_columns(&self) -> i32 {
    //     return self.columns_heads().len().try_into().unwrap();
    // }

    fn build_final_truth_table(&self) -> (Vec<FormulaRef>, Vec<Vec<TruthValue>>){
        let amount_of_atoms = self.amount_of_atoms();

        let columns_heads : Vec<FormulaRef> = self.columns_heads();
        let mut truth_values : Vec<Vec<TruthValue>> = TruthValue::generate_possible_options_aux(amount_of_atoms);

        // (Formula repr, position in columns)
        let formulas_positions_in_columns : HashMap<String, i32> = self.formulas_positions_in_columns();
        
        // Compound formulas are ordered in less_complex -> most_complex (left -> right)
        let starting_compound_formulas_position = amount_of_atoms;
        for i in 0..self.amount_of_compound_formulas(){
            let index_in_columns_heads = starting_compound_formulas_position + i;

            let current_compound_formula: FormulaRef = columns_heads[index_in_columns_heads as usize].clone();
            let current_main_connective: Connective = current_compound_formula.main_connective();


            // Add the value of compound formula in each value_row
            for truth_row in truth_values.iter_mut(){
                match current_main_connective{
                    // Can't be atomic formula, so just checks for Not connective
                    Connective::Not => {
                        let left_subformula: FormulaRef = current_compound_formula.left_subformula();
                        let left_subformula_repr: String = left_subformula.repr();
                        let left_subformula_column: i32 = *(formulas_positions_in_columns.get(&left_subformula_repr).unwrap());

                        let left_subformula_value_in_row : TruthValue = truth_row[left_subformula_column as usize];

                        let current_formula_value: TruthValue = Connective::not_rules(left_subformula_value_in_row);

                        truth_row.push(current_formula_value);
                    },
                    _ => {
                        // Get values of subformulas in this row
                        let left_subformula: FormulaRef = current_compound_formula.left_subformula();
                        let right_subformula: FormulaRef = current_compound_formula.right_subformula();
                        let left_subformula_repr: String = left_subformula.repr();
                        let right_subformula_repr: String = right_subformula.repr();

                        let left_subformula_column: i32 = *(formulas_positions_in_columns.get(&left_subformula_repr).unwrap());
                        let right_subformula_column: i32 = *(formulas_positions_in_columns.get(&right_subformula_repr).unwrap());

                        let left_subformula_value_in_row : TruthValue = truth_row[left_subformula_column as usize];
                        let right_subformula_value_in_row : TruthValue = truth_row[right_subformula_column as usize];

                        // Calculate current_formula value
                        let current_formula_value : TruthValue = Connective::apply_connective(left_subformula_value_in_row, right_subformula_value_in_row, current_main_connective);
                        
                        // push to truth_row end
                        truth_row.push(current_formula_value);
                    }
                }
            }
        }

        return (columns_heads, truth_values);
    }


    fn center_truth_value(val: TruthValue, reference: String) -> String{
        let mut centered_string: String = String::new();

        let reference_size : i32 = reference.len().try_into().unwrap();

        let correct_value_position : i32 = reference_size/2;

        for i in 0..reference_size{
            if i == correct_value_position{
                centered_string.push(val.to_string().chars().next().unwrap());
            }
            centered_string.push(' ');
        }

        return centered_string
    }

    #[allow(dead_code)]
    pub fn print_final_truth_table(&self) {
        let (_, truth_values) = self.build_final_truth_table();
        let columns_heads_repr = self.columns_heads_repr();
        // Priting heads
        println!("{}", columns_heads_repr.join(" | "));

        // let formulas_positions_in_columns: HashMap<String, i32> = self.formulas_positions_in_columns();

        for row in truth_values.iter(){
            let mut row_string : Vec<String> = vec![];
            
            for (formula_position_in_column, truth_value) in row.iter().enumerate(){
                row_string.push(TruthTable::center_truth_value(*truth_value, columns_heads_repr[formula_position_in_column].clone()))
            }

            println!("{}", row_string.join("| "));
        }
    }
}