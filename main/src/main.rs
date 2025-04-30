use std::rc::Rc;
use std::cell::{Ref, RefCell};
use std::ops::Deref;
use std::fmt;

#[derive(Clone)]
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



#[derive(Clone)]
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
    // fn generate_possible_options_aux(number_of_formulas: i32) -> Vec<Vec<TruthValue>>{
    //     let mut all_valuations : Vec<TruthValue> = vec![TruthValue::T, TruthValue::F];
    //     let mut results_vector : Vec<Vec<TruthValue>> = vec![vec![TruthValue::T], vec![TruthValue::F]];

    //     for i in 1..number_of_formulas{
    //         let mut added_combinations_vector: Vec<Vec<TruthValue>> = Vec::new();

    //         for path in results_vector.iter() {
    //             for valuation in all_valuations.iter(){
    //                 let mut new_combination : Vec<TruthValue> = path.extend(valuation);
    //             }
    //         }
        
        
    //     }

    //     return results_vector;

        
    // }
}

#[derive(Clone)]
struct FormulaRef(Option<Rc<RefCell<Formula>>>);

impl Deref for FormulaRef {
    type Target = Option<Rc<RefCell<Formula>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FormulaRef {
    fn option(&self) -> Option<Rc<RefCell<Formula>>> {
        return self.clone().0;
    }

    fn set_valuation (&self, value: TruthValue) {
        let unwrapped = self.option();

        if let Some(f) = unwrapped {
            let mut borrowed = f.borrow_mut();
            borrowed.valuation = value;
        }
    }

    fn print_wff(formula_ref: FormulaRef){
        let formula_option = formula_ref.option();
        match formula_option{
            Some(f) => {
                let borrowed = f.borrow();
                match &borrowed.valuation{
                    value => {
                        println!("{} : {}", borrowed.repr, value.to_string());
                    }
                }
            }
            None => {
                println!("None");
            }
        }
    }

    fn left_subformula(&self) -> FormulaRef {
        let formula_option = self.option();
        match formula_option{
            Some(rc_refcell_formula) => {
                let borrowed = rc_refcell_formula.borrow();
                return borrowed.left.clone();
            }
            None => {
                return FormulaRef(None)
            }
        }
    }

    fn right_subformula(&self) -> FormulaRef {
        let formula_option = self.option();
        match formula_option{
            Some(rc_refcell_formula) => {
                let borrowed = rc_refcell_formula.borrow();
                return borrowed.right.clone();
            }
            None => {
                return FormulaRef(None)
            }
        }
    }
}


#[derive(Clone)]
struct Formula {
    repr: String,
    pub valuation: TruthValue,
    atomic: bool,
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
                let borrowed_left = left.borrow();
                let borrowed_right = right.borrow();
                return format!("({} {} {})", borrowed_left.repr, connective.to_string(), borrowed_right.repr)
            }
            (Some(left), None) => {
                let borrowed_left = left.borrow();
                return format!("{}{}", connective.to_string(), borrowed_left.repr) 
            }
            (None, Some(right)) => {
                let borrowed_right = right.borrow();
                return format!("{}{}", connective.to_string(), borrowed_right.repr) 
            }
            (None, None) => {
                return "".to_string()
            }
        }
           
    }

    fn wrap_atomic(formula: Formula) -> FormulaRef{
        return FormulaRef(Some(Rc::new(RefCell::new(formula))));
    }

    fn build_atomic_wff(repr: String) -> Formula {
        return Formula {
            repr : repr,
            valuation: TruthValue::unassigned,
            atomic: true,
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
            valuation: TruthValue::unassigned, 
            atomic: false, 
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

    fn add_compound_formula(&mut self, compound_formula: FormulaRef){
        self.compound_formulas.push(compound_formula);
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
                    Some(rc_refcell_formula) => rc_refcell_formula.borrow().repr.clone(),
                    None => "".to_string(),
                }
            }
        ).collect::<Vec<String>>();
    }

    fn columns_heads_string(&self) -> String {
        return self.columns_heads_repr().join(" | ");
    }

    fn amount_of_columns(&self) -> i32 {
        return self.columns_heads().len().try_into().unwrap();
    }

}


fn main() {
    let p : FormulaRef = Formula::build_wrapped_atomic_wff("p".to_string());
    let q : FormulaRef  = Formula::build_wrapped_atomic_wff("q".to_string());
    let r : FormulaRef = Formula::build_wrapped_atomic_wff("r".to_string());
    let p_and_q : FormulaRef = Formula::build_wrapped_wff(Connective::And, p.clone(), q.clone());
    let r_implies_p_and_q : FormulaRef = Formula::build_wrapped_wff(Connective::Imp, r.clone(), p_and_q.clone());
    let neg_r_implies_p_and_q : FormulaRef = Formula::build_wrapped_wff(Connective::Not, r_implies_p_and_q.clone(), FormulaRef(None));
    let r_iff_p : FormulaRef = Formula::build_wrapped_wff(Connective::BiImp, r.clone(), p.clone());
    let neg_r_implies_p_and_q_or_r_and_p : FormulaRef = Formula::build_wrapped_wff(Connective::Or, neg_r_implies_p_and_q.clone(), r_iff_p.clone());

    let base_atoms: Vec<FormulaRef> = vec![p.clone(), q.clone(), r.clone()];

    let mut truth_table : TruthTable = TruthTable::build_base_truth_table(base_atoms);
    truth_table.add_compound_formula(p_and_q.clone());
    let columns_heads_string = truth_table.columns_heads_repr().join(" | ");

    println!("{}", columns_heads_string);



    
    
    FormulaRef::print_wff(p.clone());
    p_and_q.left_subformula().set_valuation(TruthValue::F);
    FormulaRef::print_wff(p.clone());
}
