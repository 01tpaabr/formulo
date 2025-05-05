use crate::connective::{Connective};

#[derive(Clone)]
pub struct FormulaRef (
    pub Option<Box<Formula>>
);


impl FormulaRef {
    pub fn option(&self) -> Option<Box<Formula>> {
        return self.clone().0;
    }

    #[allow(dead_code)]
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

    pub fn main_connective(&self) -> Connective {
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

    pub fn repr(&self) -> String{
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

    pub fn left_subformula(&self) -> FormulaRef {
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

    pub fn right_subformula(&self) -> FormulaRef {
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

#[derive(Clone)]
pub struct Formula{
    pub repr: String,
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
        return FormulaRef(Some(Box::new(formula)));
    }

    fn build_atomic_wff(repr: String) -> Formula {
        return Formula {
            repr : repr,
            main_connective: Connective::Atomic,
            left: FormulaRef(None),
            right: FormulaRef(None),
        }
    }

    pub fn build_wrapped_atomic_wff(repr: String) -> FormulaRef {
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

    pub fn build_wrapped_wff(connective: Connective, left: &FormulaRef, right: &FormulaRef) -> FormulaRef {
        let cloned_left: FormulaRef = left.clone();
        let cloned_right: FormulaRef = right.clone();
        return Formula::wrap_atomic(Formula::build_wff(connective, cloned_left, cloned_right));
    }

}