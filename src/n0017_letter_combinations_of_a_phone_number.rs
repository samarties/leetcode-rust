use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut digit_map = HashMap::new();
        digit_map.insert("2".to_string(), vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        digit_map.insert("3".to_string(), vec!["d".to_string(), "e".to_string(), "f".to_string()]);
        digit_map.insert("4".to_string(), vec!["g".to_string(), "h".to_string(), "i".to_string()]);
        digit_map.insert("5".to_string(), vec!["j".to_string(), "k".to_string(), "l".to_string()]);
        digit_map.insert("6".to_string(), vec!["m".to_string(), "n".to_string(), "o".to_string()]);
        digit_map.insert("7".to_string(), vec!["p".to_string(), "q".to_string(), "r".to_string(), "s".to_string()]);
        digit_map.insert("8".to_string(), vec!["t".to_string(), "u".to_string(), "v".to_string()]);
        digit_map.insert("9".to_string(), vec!["w".to_string(), "x".to_string(), "y".to_string(), "z".to_string()]);

        let mut combinations: Vec<String> = Vec::new();

        for digit in digits.chars() {
            let combinations_len = combinations.len();
            let mapped_letters = digit_map.get(&digit.to_string()).unwrap();

            if combinations_len < 1 {
                combinations = mapped_letters.to_owned();
                continue;
            }

            let mut updated_combinations: Vec<String> = Vec::new();
            for combination in combinations {
                for mapped_letter in mapped_letters {
                    let mut updated_combination = combination.to_string();
                    updated_combination.push_str(mapped_letter);
                    updated_combinations.push(updated_combination);
                }
            }

            combinations = updated_combinations;
        }

        return combinations;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_digit() {
        assert_eq!(
            Solution::letter_combinations("4".to_string()),
            vec!["g", "h", "i"]
        );
    }

    #[test]
    fn test_two_digits() {
        assert_eq!(
            Solution::letter_combinations("23".to_string()),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
    }

    #[test]
    fn test_three_digits() {
        assert_eq!(
            Solution::letter_combinations("482".to_string()),
            vec!["gta", "gtb", "gtc", "gua", "gub", "guc", "gva", "gvb", "gvc", "hta", "htb", "htc", "hua", "hub", "huc", "hva", "hvb", "hvc", "ita", "itb", "itc", "iua", "iub", "iuc", "iva", "ivb", "ivc"]
        );
    }

    #[test]
    fn test_same_digits() {
        assert_eq!(
            Solution::letter_combinations("22".to_string()),
            vec!["aa", "ab", "ac", "ba", "bb", "bc", "ca", "cb", "cc"]
        );
    }
}
