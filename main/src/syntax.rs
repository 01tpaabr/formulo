use std::fmt;

// Symbols to glue together less complex formulas
pub enum Operator {
    Not,
    And,
    Or,
    Imp,
    Bimp
}

pub enum OperatorArity {
    Unary,
    Binary
}

impl Operator {
    pub fn arity(operator: &Operator) -> OperatorArity {
        match operator {
            Operator::Not => OperatorArity::Unary,
            _ => OperatorArity::Binary
        }
    }

    pub fn operator_repr(operator: &Operator) -> &str {
        match operator {
            Self::Not => {
                "~"
            }
            Self::And => {
                r"/\"
            }
            Self::Or => {
                r"\/"
            }
            Self::Imp => {
                "->"
            }
            Self::Bimp => {
                "<->"
            }
        }
    }

    pub fn wrap_operator(operator: &Operator, left_formula_repr : Option<String>, right_formula_repr : Option<String>) -> String {
            let op_arity: OperatorArity = Operator::arity(operator);
            let operator_repr = Operator::operator_repr(operator);

            match op_arity {
                OperatorArity::Unary => {
                    match (left_formula_repr, right_formula_repr) {
                        (None, None) => panic!("Operator wrapping error"),
                        (None, _) => panic!("Operator wrapping error"),
                        (Some(left_repr), _) => {
                            format!("{operator_repr}{left_repr}")
                        }
                    }
                }
                OperatorArity::Binary => {
                    match (left_formula_repr, right_formula_repr) {
                        (Some(left_repr), Some(right_repr)) => {
                            format!("({left_repr} {operator_repr} {right_repr})")
                        }
                        (_, _) => panic!("Operator wrapping error"),
                    }
                }
            }
    }
}

// wff separated by their main connective
pub enum Wff {
    Atomic(String),
    NotFormula(String),
    AndFormula(String),
    OrFormula(String),
    ImpFormula(String),
    BimpFormula(String)
}

impl fmt::Display for Wff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_formula_repr())
    }
}


impl Wff {
    pub fn get_formula_repr(&self) -> String{
        match self {
            Wff::Atomic(s)
            | Wff::NotFormula(s)
            | Wff::AndFormula(s)
            | Wff::OrFormula(s)
            | Wff::ImpFormula(s)
            | Wff::BimpFormula(s) => s.to_string()
        }
    }

    pub fn new_atomic_wff(placeholder: &str) -> Wff{
        Wff::Atomic(placeholder.to_string())
    }

    pub fn operator_formula_type(operator: &Operator, repr: String) -> Wff {
        match operator {
            Operator::Not => {
                Wff::NotFormula(repr)
            }
            Operator::And => {
                Wff::AndFormula(repr)
            }
            Operator::Or => {
                Wff::OrFormula(repr)
            }
            Operator::Imp => {
                Wff::ImpFormula(repr)
            }
            Operator::Bimp => {
                Wff::BimpFormula(repr)
            }
        }
    }

    pub fn build_wff(operator: Operator, left_formula: Option<&Wff>, right_formula: Option<&Wff>) -> Wff {
        let op_arity: OperatorArity = Operator::arity(&operator);

        match op_arity {
            OperatorArity::Unary => {
                match (left_formula, right_formula) {
                    (None, None) => panic!("Wff building erorr (None, None)"),
                    (None, _) => panic!("Wff building error (None, _)"),
                    (Some(left_repr), _) => {
                        let repr_string = Operator::wrap_operator(&operator, Some(left_repr.get_formula_repr()), None);
                        Wff::operator_formula_type(&operator, repr_string)
                    }
                }
            }
            OperatorArity::Binary => {
                match (left_formula, right_formula) {
                    (Some(left_repr), Some(right_repr)) => {
                        let repr_string = Operator::wrap_operator(&operator, Some(left_repr.get_formula_repr()), Some(right_repr.get_formula_repr()));
                        Wff::operator_formula_type(&operator, repr_string)
                    }
                    (_, _) => panic!("Operator wrapping error not (Some(), Some())"),
                }
            }
        }
    
    }


}