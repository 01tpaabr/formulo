use core::panic;
use std::fmt;

// Symbols to glue together less complex formulas
pub enum Operator {
    Not,
    And,
    Or,
    Imp,
    Bimp,
    Atomic
}

pub enum OperatorArity {
    Unary,
    Binary
}

// Ideally if new operator is to be added, the only changes that are need are in this section, along with adding one Invariant in Wff enum
impl Operator {
    pub fn arity(operator: &Operator) -> OperatorArity {
        match operator {
            Operator::Not => OperatorArity::Unary,
            _ => OperatorArity::Binary
        }
    }

    pub fn operator_repr(operator: &Operator) -> &str {
        match operator {
            Self::Atomic => {
                ""
            }
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

    pub fn build_formula_type_by_operator(operator: &Operator, repr: String) -> Wff {
        match operator {
            Operator::Atomic => {
                Wff::BimpFormula(repr)
            }
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
#[derive(Clone)]
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
    pub fn formula_type_to_operator(wff: &Wff) -> Operator {
        match wff {
            Wff::Atomic(_) => {
                Operator::Atomic
            }
            Wff::NotFormula(_) => {
                Operator::Not
            }
            Wff::AndFormula(_) => {
                Operator::And
            }
            Wff::OrFormula(_) => {
                Operator::Or
            }
            Wff::ImpFormula(_) => {
                Operator::Imp
            }
            Wff::BimpFormula(_) => {
                Operator::Bimp
            }
        }
    }

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

    pub fn build_wff(operator: Operator, left_formula: Option<&Wff>, right_formula: Option<&Wff>) -> Wff {
        let op_arity: OperatorArity = Operator::arity(&operator);

        match op_arity {
            OperatorArity::Unary => {
                match (left_formula, right_formula) {
                    (None, None) => panic!("Wff building error (None, None)"),
                    (None, _) => panic!("Wff building error (None, _)"),
                    (Some(left_repr), _) => {
                        let repr_string = Operator::wrap_operator(&operator, Some(left_repr.get_formula_repr()), None);
                        Operator::build_formula_type_by_operator(&operator, repr_string)
                    }
                }
            }
            OperatorArity::Binary => {
                match (left_formula, right_formula) {
                    (Some(left_repr), Some(right_repr)) => {
                        let repr_string = Operator::wrap_operator(&operator, Some(left_repr.get_formula_repr()), Some(right_repr.get_formula_repr()));
                        Operator::build_formula_type_by_operator(&operator, repr_string)
                    }
                    (_, _) => panic!("Operator wrapping error not (Some(), Some())"),
                }
            }
        }
    }

}

#[derive(Clone)]
pub struct SyntaxNode {
    content: Wff,
    left: Option<Box<SyntaxNode>>,
    right: Option<Box<SyntaxNode>>
}

impl SyntaxNode {
    pub fn get_wff(&self) -> Wff{
        self.content.clone()
    }

    pub fn build_syntax_node(operator: Operator, left_formula: Option<Box<SyntaxNode>>, right_formula: Option<Box<SyntaxNode>>) -> SyntaxNode {
        let op_arity: OperatorArity = Operator::arity(&operator);

        match op_arity {
            OperatorArity::Unary => {
                match (left_formula, right_formula) {
                    (None, None) => panic!("Wff building error (None, None)"),
                    (None, _) => panic!("Wff building error (None, _)"),
                    (Some(left_box), _) => {
                        let repr_string = Operator::wrap_operator(&operator, Some(left_box.get_wff().get_formula_repr()), None);
                        let new_wff = Operator::build_formula_type_by_operator(&operator, repr_string);
                        SyntaxNode{
                            content: new_wff,
                            left: Some(left_box.clone()),
                            right: None
                        }
                    }
                }
            }
            OperatorArity::Binary => {
                match (left_formula, right_formula) {
                    (Some(left_box), Some(right_box)) => {
                        let repr_string = Operator::wrap_operator(&operator, Some(left_box.get_wff().get_formula_repr()), Some(right_box.get_wff().get_formula_repr()));
                        let new_wff = Operator::build_formula_type_by_operator(&operator, repr_string);
                        SyntaxNode{
                            content: new_wff,
                            left: Some(left_box.clone()),
                            right: Some(right_box.clone())
                        }
                    }
                    (_, _) => panic!("Operator wrapping error not (Some(), Some())"),
                }
            }
        }
    }

    pub fn parse_string_into_syntax_node(input_string : String) -> SyntaxNode {
        panic!()
    }
}
