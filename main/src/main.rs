use std::rc::Rc;
use std::ops::Deref;
use std::fmt;
use std::collections::HashMap;


#[derive(Clone, Copy)]
enum Connective {
    And,
    Or,
    Not,
    Imp,
    BiImp,
    Atomic,
}

impl fmt::Display for Connective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Connective::And => r"/\",
            Connective::Or => r"\/",
            Connective::Not=> "~",
            Connective::Imp => "->",
            Connective::BiImp => "<->",
            Connective::Atomic => ""
        };
        write!(f, "{}", s)
    }
}

impl Connective {
    fn and_rules(val1: TruthValue, val2: TruthValue) -> TruthValue {
        match (val1, val2){
            (TruthValue::unassigned, _) | (_, TruthValue::unassigned) => {
                return TruthValue::unassigned;
            },
            (TruthValue::T, TruthValue::T) => {
                return TruthValue::T;
            },
            (_, _) => {
                return TruthValue::F;
            }
        }
    }

    fn or_rules(val1: TruthValue, val2: TruthValue) -> TruthValue {
        match (val1, val2){
            (TruthValue::unassigned, _) | (_, TruthValue::unassigned) => {
                return TruthValue::unassigned;
            },
            (TruthValue::F, TruthValue::F) => {
                return TruthValue::F;
            },
            (_, _) => {
                return TruthValue::T;
            }
        }
    }

    fn imp_rules(val1: TruthValue, val2: TruthValue) -> TruthValue {
        match (val1, val2){
            (TruthValue::unassigned, _) | (_, TruthValue::unassigned) => {
                return TruthValue::unassigned;
            },
            (TruthValue::T, TruthValue::F) => {
                return TruthValue::F;
            },
            (_, _) => {
                return TruthValue::T;
            }
        }
    }

    fn bi_imp_rules(val1: TruthValue, val2: TruthValue) -> TruthValue {
        match (val1, val2){
            (TruthValue::unassigned, _) | (_, TruthValue::unassigned) => {
                return TruthValue::unassigned;
            },
            (TruthValue::T, TruthValue::T) | (TruthValue::F, TruthValue::F) => {
                return TruthValue::T;
            },
            (_, _) => {
                return TruthValue::F;
            }
        }
    }

    fn not_rules(val: TruthValue) -> TruthValue {
        match val{
            TruthValue::unassigned => {
                return TruthValue::unassigned;
            },
            TruthValue::T => {
                return TruthValue::F;
            },
            TruthValue::F => {
                return TruthValue::T;
            }
        }
    }

    fn atomic_rules(val : TruthValue) -> TruthValue{
        return val;
    }

    fn apply_connective(val1: TruthValue, val2: TruthValue, connective: Connective) -> TruthValue{
        match connective{
            Connective::And => {
                return Connective::and_rules(val1, val2);
            },
            Connective::Or => {
                return Connective::or_rules(val1, val2);
            },
            Connective::Imp => {
                return Connective::imp_rules(val1, val2);
            },
            Connective::BiImp => {
                return Connective::bi_imp_rules(val1, val2);
            },
            Connective::Not => {
                return TruthValue::unassigned;
            },
            Connective::Atomic => {
                return TruthValue::unassigned;
            }
        }
    }
}



#[derive(Clone, Copy)]
enum TruthValue {
    T,
    F,
    unassigned,
}

impl fmt::Display for TruthValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TruthValue::T => "T",
            TruthValue::F => "F",
            TruthValue::unassigned => "unassigned",
        };
        write!(f, "{}", s)
    }
}

impl TruthValue {
    fn generate_possible_options_aux(number_of_formulas: i32) -> Vec<Vec<TruthValue>>{
        let all_valuations : Vec<TruthValue> = vec![TruthValue::T, TruthValue::F];
        let mut results_vector : Vec<Vec<TruthValue>> = vec![vec![TruthValue::T], vec![TruthValue::F]];

        for _ in 1..number_of_formulas{
            let mut added_combinations_vector: Vec<Vec<TruthValue>> = Vec::new();

            for path in results_vector.iter() {
                for valuation in all_valuations.iter(){
                    let mut new_combination : Vec<TruthValue> = path.clone();
                    new_combination.push(*valuation);

                    added_combinations_vector.push(new_combination);
                }
            }

            results_vector = added_combinations_vector;
        
        }
        return results_vector;
    }
}

#[derive(Clone)]
struct FormulaRef(Option<Rc<Formula>>);

impl Deref for FormulaRef {
    type Target = Option<Rc<Formula>>;

    fn deref(&self) -> &Self::Target {
        &self
    }
}

impl FormulaRef {
    fn option(&self) -> Option<Rc<Formula>> {
        return self.clone().0;
    }

    fn print_wff(formula_ref: FormulaRef){
        let formula_option = formula_ref.option();
        match formula_option{
            Some(f) => {
                println!("{}", f.repr);
            }
            None => {
                println!("None");
            }
        }
    }

    fn main_connective(&self) -> Connective {
        let formula_option = self.option();
        match formula_option{
            Some(rc_f) => {
                return rc_f.clone().main_connective;
            }
            None => {
                return Connective::Atomic;
            }
        }
    }

    fn repr(&self) -> String{
        let formula_option = self.option();
        match formula_option{
            Some(rc_f) => {
                return rc_f.repr.clone();
            }
            None => {
                return "".to_string();
            }
        }
    }

    fn left_subformula(&self) -> FormulaRef {
        let formula_option = self.option();
        match formula_option{
            Some(rc_f) => {
                return rc_f.left.clone();
            }
            None => {
                return FormulaRef(None)
            }
        }
    }

    fn right_subformula(&self) -> FormulaRef {
        let formula_option = self.option();
        match formula_option{
            Some(rc_f) => {
                return rc_f.right.clone();
            }
            None => {
                return FormulaRef(None)
            }
        }
    }
}


struct Formula {
    repr: String,
    main_connective: Connective,
    left: FormulaRef,
    right: FormulaRef
}

impl Formula {
    fn repr_wff(connective: &Connective, left: FormulaRef, right: FormulaRef) -> String {
        let left_option = left.option();
        let right_option = right.option();
        match (left_option, right_option) {
            (Some(left), Some(right)) => {
                return format!("({} {} {})", left.repr, connective.to_string(), right.repr)
            }
            (Some(left), None) => {
                return format!("{}{}", connective.to_string(), left.repr) 
            }
            (None, Some(right)) => {
                return format!("{}{}", connective.to_string(), right.repr) 
            }
            (None, None) => {
                return "".to_string()
            }
        }
           
    }

    fn wrap_atomic(formula: Formula) -> FormulaRef{
        return FormulaRef(Some(Rc::new(formula)));
    }

    fn build_atomic_wff(repr: String) -> Formula {
        return Formula {
            repr : repr,
            main_connective: Connective::Atomic,
            left: FormulaRef(None),
            right: FormulaRef(None),
        }
    }

    fn build_wrapped_atomic_wff(repr: String) -> FormulaRef {
        return Formula::wrap_atomic(Formula::build_atomic_wff(repr));
    }

    fn build_wff(connective: Connective, left: FormulaRef, right: FormulaRef) -> Formula {
        return Formula { 
            repr: Formula::repr_wff(&connective, left.clone(), right.clone()), 
            main_connective: connective, 
            left: left,
            right: right
        }
    }

    fn build_wrapped_wff(connective: Connective, left: FormulaRef, right: FormulaRef) -> FormulaRef {
        return Formula::wrap_atomic(Formula::build_wff(connective, left, right));
    }

}

struct TruthTable {
    atoms: Vec<FormulaRef>,
    compound_formulas: Vec<FormulaRef>, 
}

impl TruthTable {
    fn build_base_truth_table(atoms: Vec<FormulaRef>) -> TruthTable{
        return TruthTable{
            atoms: atoms, 
            compound_formulas: Vec::new()
        }; 
    }

    fn amount_of_atoms(&self) -> i32 {
        return self.atoms.len().try_into().unwrap();
    }

    fn amount_of_compound_formulas(&self) -> i32 {
        return self.compound_formulas.len().try_into().unwrap();
    }

    fn amount_of_value_rows(&self) -> i32 {
        return 2i32.pow(self.atoms.len().try_into().unwrap());
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

    fn add_compound_formula(&mut self, compound_formula: FormulaRef){
        let formulas_positions_in_columns: HashMap<String, i32> = self.formulas_positions_in_columns();

        match compound_formula.main_connective(){
            Connective::Not => {
                if None == formulas_positions_in_columns.get(&compound_formula.left_subformula().repr()){
                    panic!("Subformulas of Formula [{}] are not present in this TruthTable", compound_formula.repr());
                }
            },
            _ => {
                if None == formulas_positions_in_columns.get(&compound_formula.left_subformula().repr())
                || None == formulas_positions_in_columns.get(&compound_formula.right_subformula().repr()){
                    panic!("Subformulas of Formula [{}] are not present in this TruthTable", compound_formula.repr());
                }
            }
        }
        

        self.compound_formulas.push(compound_formula);
    }

   

    fn columns_heads_string(&self) -> String {
        return self.columns_heads_repr().join(" | ");
    }

    fn amount_of_columns(&self) -> i32 {
        return self.columns_heads().len().try_into().unwrap();
    }

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

    fn print_final_truth_table(&self) {
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
