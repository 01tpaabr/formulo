use std::fmt;

#[derive(Clone, Copy)]
pub enum TruthValue {
    T,
    F,
    Unassigned,
}

impl fmt::Display for TruthValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TruthValue::T => "T",
            TruthValue::F => "F",
            TruthValue::Unassigned => "Unassigned",
        };
        write!(f, "{}", s)
    }
}

impl TruthValue {
    pub fn generate_possible_options_aux(number_of_formulas: i32) -> Vec<Vec<TruthValue>>{
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
