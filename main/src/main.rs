use core::borrow;
use std::rc::Rc;
use std::cell::{Ref, RefCell};
use std::ops::Deref;

#[derive(Clone)]
enum Connective {
    And,
    Or,
    Not,
    Imp,
    BiImp,
    Atomic,
}

fn represent_connective(c: &Connective) -> String {
    match c {
        Connective::And => r"/\".to_string(),
        Connective::Or => r"\/".to_string(),
        Connective::Not=> "~".to_string(),
        Connective::Imp => "->".to_string(),
        Connective::BiImp => "<->".to_string(),
        Connective::Atomic => "".to_string()
    }
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

    fn set_valuation (&self, value: bool) {
        let unwrapped = self.option();

        if let Some(f) = unwrapped {
            let mut borrowed = f.borrow_mut();
            borrowed.valuation = Some(value);
        }
    }

    fn print_wff(formula_ref: FormulaRef){
        let option = formula_ref.option();
        match option{
            Some(f) => {
                let borrowed = f.borrow();
                match borrowed.valuation{
                    Some(value) => {
                        println!("{} : {}", borrowed.repr, value.to_string());
                    }
                    None => {
                        println!("{} : `placeholder`", borrowed.repr);
                        
                    }
                }
            }
            None => {
                println!("None");
            }
        }
    }

    fn left_subformula(&self) -> FormulaRef {
        let option = self.option();
        match option{
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
        let option = self.option();
        match option{
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
    pub valuation: Option<bool>,
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
                return format!("({} {} {})", borrowed_left.repr, represent_connective(connective), borrowed_right.repr)
            }
            (Some(left), None) => {
                let borrowed_left = left.borrow();
                return format!("{}{}", represent_connective(connective), borrowed_left.repr) 
            }
            (None, Some(right)) => {
                let borrowed_right = right.borrow();
                return format!("{}{}", represent_connective(connective), borrowed_right.repr) 
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
            valuation: None,
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
            valuation: None, 
            atomic: false, 
            main_connective: connective, 
            left: left,
            right: right
        }
    }

    fn build_wrapped_wff(connective: Connective, left: FormulaRef, right: FormulaRef) -> FormulaRef {
        return Formula::wrap_atomic(Formula::build_wff(connective, left, right));
    }

    fn left_subformula(&self) -> FormulaRef {
        return self.left.clone();
    }

    fn right_subformula(&self) -> FormulaRef {
        return self.right.clone();
    }

}


fn main() {
    let p : FormulaRef = Formula::build_wrapped_atomic_wff("p".to_string());
    let q : FormulaRef  = Formula::build_wrapped_atomic_wff("q".to_string());
    let r : FormulaRef = Formula::build_wrapped_atomic_wff("r".to_string());
    let p_and_q : FormulaRef = Formula::build_wrapped_wff(Connective::And, p.clone(), q.clone());
    let r_implies_p_and_q : FormulaRef = Formula::build_wrapped_wff(Connective::Imp, r.clone(), p_and_q.clone());
    let neg_r_implies_p_and_q : FormulaRef = Formula::build_wrapped_wff(Connective::Not, r_implies_p_and_q.clone(), FormulaRef(None));
    
    FormulaRef::print_wff(p.clone());
    p_and_q.left_subformula().set_valuation(false);
    FormulaRef::print_wff(neg_r_implies_p_and_q.left_subformula().right_subformula().left_subformula());
}
