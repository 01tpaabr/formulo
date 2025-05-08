mod syntax;

use crate::syntax::*;

fn main() {
    let p = Wff::new_atomic_wff("p");
    let q = Wff::new_atomic_wff("q");

    let not_p = Wff::build_wff(Operator::Not, Some(&p), None);
    let and = Wff::build_wff(Operator::And, Some(&not_p), Some(&q));
    let imp = Wff::build_wff(Operator::Imp, Some(&and), Some(&p));

    println!("{imp}");
}
