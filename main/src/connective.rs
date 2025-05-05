use crate::truth_value::TruthValue;
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub enum Connective {
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
            (TruthValue::Unassigned, _) | (_, TruthValue::Unassigned) => {
                return TruthValue::Unassigned;
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
            (TruthValue::Unassigned, _) | (_, TruthValue::Unassigned) => {
                return TruthValue::Unassigned;
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
            (TruthValue::Unassigned, _) | (_, TruthValue::Unassigned) => {
                return TruthValue::Unassigned;
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
            (TruthValue::Unassigned, _) | (_, TruthValue::Unassigned) => {
                return TruthValue::Unassigned;
            },
            (TruthValue::T, TruthValue::T) | (TruthValue::F, TruthValue::F) => {
                return TruthValue::T;
            },
            (_, _) => {
                return TruthValue::F;
            }
        }
    }

    pub fn not_rules(val: TruthValue) -> TruthValue {
        match val{
            TruthValue::Unassigned => {
                return TruthValue::Unassigned;
            },
            TruthValue::T => {
                return TruthValue::F;
            },
            TruthValue::F => {
                return TruthValue::T;
            }
        }
    }

    pub fn apply_connective(val1: TruthValue, val2: TruthValue, connective: Connective) -> TruthValue{
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
                return TruthValue::Unassigned;
            },
            Connective::Atomic => {
                return TruthValue::Unassigned;
            }
        }
    }
}

pub trait ConnectiveType {
    const CONNECTIVE: Connective;
}

pub struct AndType;
pub struct OrType;
pub struct NotType;
pub struct ImpType;
pub struct BiImpType;
pub struct AtomicType;

impl ConnectiveType for AndType {
    const CONNECTIVE: Connective = Connective::And;
}

impl ConnectiveType for OrType {
    const CONNECTIVE: Connective = Connective::Or;
}

impl ConnectiveType for NotType {
    const CONNECTIVE: Connective = Connective::Not;
}

impl ConnectiveType for ImpType {
    const CONNECTIVE: Connective = Connective::Imp;
}

impl ConnectiveType for BiImpType {
    const CONNECTIVE: Connective = Connective::BiImp;
}

impl ConnectiveType for AtomicType {
    const CONNECTIVE: Connective = Connective::Atomic;
}