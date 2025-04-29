use core::borrow;
use std::rc::Rc;
use std::cell::{Ref, RefCell};

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
struct Formula {
    repr: String,
    pub valuation: Option<bool>,
    atomic: bool,
    main_connective: Connective,
    left: Option<Rc<RefCell<Formula>>>,
    right: Option<Rc<RefCell<Formula>>>
}

#[derive(Clone)]
struct FormulaRef(Option<Rc<RefCell<Formula>>>);

impl FormulaRef{
    
}

impl Formula {
    fn print_wff(formula: Option<Rc<RefCell<Formula>>>,){
        match formula{
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

    fn repr_wff(connective: &Connective, left: Option<Rc<RefCell<Formula>>>, right: Option<Rc<RefCell<Formula>>>) -> String {
        match (left, right) {
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

    fn wrap_atomic(formula: Formula) -> Option<Rc<RefCell<Formula>>>{
        return Some(Rc::new(RefCell::new(formula)));
    }

    fn build_atomic_wff(repr: String) -> Formula {
        return Formula {
            repr : repr,
            valuation: None,
            atomic: true,
            main_connective: Connective::Atomic,
            left: None,
            right: None,
        }
    }

    fn build_wrapped_atomic_wff(repr: String) -> Option<Rc<RefCell<Formula>>> {
        return Formula::wrap_atomic(Formula::build_atomic_wff(repr));
    }

    fn build_wff(connective: Connective, left: Option<Rc<RefCell<Formula>>>, right: Option<Rc<RefCell<Formula>>>) -> Formula {
        return Formula { 
            repr: Formula::repr_wff(&connective, left.clone(), right.clone()), 
            valuation: None, 
            atomic: false, 
            main_connective: connective, 
            left: left,
            right: right
        }
    }

    fn build_wrapped_wff(connective: Connective, left: Option<Rc<RefCell<Formula>>>, right: Option<Rc<RefCell<Formula>>>) -> Option<Rc<RefCell<Formula>>> {
        return Formula::wrap_atomic(Formula::build_wff(connective, left, right));
    }

    fn left_subformula(&self) -> Option<Rc<RefCell<Formula>>> {
        return self.left.clone();
    }

    fn right_subformula(&self) -> Option<Rc<RefCell<Formula>>> {
        return self.right.clone();
    }

    fn set_valuation (formula: Option<Rc<RefCell<Formula>>>, value: bool) {
        if let Some(f) = formula {
            let mut borrowed = f.borrow_mut();
            borrowed.valuation = Some(value);
        }
    }

}


fn main() {
    let p : Option<Rc<RefCell<Formula>>> = Formula::build_wrapped_atomic_wff("p".to_string());
    let q : Option<Rc<RefCell<Formula>>> = Formula::build_wrapped_atomic_wff("q".to_string());
    let r : Option<Rc<RefCell<Formula>>> = Formula::build_wrapped_atomic_wff("r".to_string());
    let p_and_q : Option<Rc<RefCell<Formula>>> = Formula::build_wrapped_wff(Connective::And, p.clone(), q.clone());
    let r_implies_p_and_q : Option<Rc<RefCell<Formula>>> = Formula::build_wrapped_wff(Connective::Imp, r.clone(), p_and_q.clone());
    let neg_r_implies_p_and_q : Option<Rc<RefCell<Formula>>> = Formula::build_wrapped_wff(Connective::Not, r_implies_p_and_q.clone(), None);
    
    Formula::print_wff(p.clone());
    Formula::set_valuation(p.clone(), true);
    Formula::print_wff(p_and_q);

    

    
}
