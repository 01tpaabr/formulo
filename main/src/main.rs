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
    valuation: Option<bool>,
    atomic: bool,
    main_connective: Connective,
    subformulas: Option<Box<(Formula, Option<Formula>)>>
}

impl Formula {
    fn print_wff(formula: Option<&Formula>){
        match formula{
            Some(f) => {
                println!("{}", f.repr);
            }
            None => {
                println!("None");
            }
        }
    }

    fn build_atomic_wff(repr: String) -> Formula {
        return Formula {
            repr : repr,
            valuation: None,
            atomic: true,
            main_connective: Connective::Atomic,
            subformulas: None
        }
    }

    fn repr_wff(connective: &Connective, subformulas: &Option<Box<(Formula, Option<Formula>)>>) -> String {
        match subformulas{
            Some(boxed) => {
                let (left, right) = boxed.as_ref();
                match right {
                    Some(right) => {
                        return format!("({} {} {})", left.repr, represent_connective(connective), right.repr)
                    }
                    None =>{
                        return format!("{}{}", represent_connective(connective), left.repr) 
                    }
                }
            }
            None => {
                return "".to_string();
            }   
        }
    }

    fn wrap_atomic_pair(left: Formula, right: Option<Formula>) -> Option<Box<(Formula, Option<Formula>)>> {
        return Some(Box::new((left, right))); 
    }

    fn internal_build_wff(connective: Connective, subformulas: Option<Box<(Formula, Option<Formula>)>>) -> Formula{
        return Formula { 
            repr: Formula::repr_wff(&connective, &subformulas), 
            valuation: None, 
            atomic: false, 
            main_connective: connective, 
            subformulas: subformulas
        }
    }

    fn build_wff(connective: Connective, left: Formula, right: Option<Formula>) -> Formula {
        return Formula::internal_build_wff(connective, Formula::wrap_atomic_pair(left, right));
    }

    fn left_subformula(&self) -> Option<&Formula> {
        self.subformulas.as_ref().map(|boxed| &boxed.0)
    }

    fn right_subformula(&self) -> Option<&Formula> {
        self.subformulas.as_ref()
            .and_then(|boxed| boxed.1.as_ref())
    }

}


fn main() {
    let p : Formula = Formula::build_atomic_wff("p".to_string());
    let q : Formula = Formula::build_atomic_wff("q".to_string()); 
    let r : Formula = Formula::build_atomic_wff("r".to_string());
    let p_and_q : Formula = Formula::build_wff(Connective::And, p, Some(q));
    let r_implies_p_and_q : Formula = Formula::build_wff(Connective::Imp, r, Some(p_and_q));
    let neg_r_implies_p_and_q : Formula = Formula::build_wff(Connective::Not, r_implies_p_and_q, None);
    
    
    Formula::print_wff(Some(&neg_r_implies_p_and_q));
    
}
