use itertools::Itertools;

fn calculate_combinations(nums: &[i32], operators: &[char]) -> Vec<String> {
    let mut results = Vec::new();

    // Generare tutte le combinazioni di operatori
    let operator_combinations = (0..=operators.len())
        .map(|_| operators)
        .multi_cartesian_product();

    for combination in operator_combinations {
        let mut expression = nums[0].to_string(); // Inizia dal primo numero
        for (num, op) in nums.iter().skip(1).zip(&combination) {
            expression.push(**op); // Aggiungi l'operatore
            expression.push_str(&num.to_string()); // Aggiungi il numero
        }
        results.push(expression);
    }

    results
}

fn main() 
{
    let nums = vec![9, 7, 18, 13];
    let operators = vec!['+', '*'];

    let combinations = calculate_combinations(&nums, &operators);
    for comb in &combinations {
        println!("{}", comb);
    }
}