use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        return Solution::is_valid_via_replace(&s);
    }

    fn is_valid_via_replace(s: &String) -> bool {
        let mut mutated_s = s.to_owned();

        while
            mutated_s.contains("{}")
            || mutated_s.contains("[]")
            || mutated_s.contains("()")
        {
            mutated_s = mutated_s
                .replace("{}", "")
                .replace("[]", "")
                .replace("()", "");
        }

        return mutated_s.len() == 0;
    }

    fn is_valid_via_stack(s: &String) -> bool {
        let mut parantheses_map = HashMap::new();
        parantheses_map.insert(")".to_string(), "(".to_string());
        parantheses_map.insert("]".to_string(), "[".to_string());
        parantheses_map.insert("}".to_string(), "{".to_string());

        let mut open_parantheses: Vec<String> = vec![];

        for char in s.chars() {
            let char_as_string = char.to_string();
            let char_is_closing = parantheses_map.contains_key(&char_as_string) == true;
            if char_is_closing {
                let required_last_opened = parantheses_map.get(&char_as_string);
                let actual_last_opened = open_parantheses.last();
                let last_opened_match = required_last_opened.eq(&actual_last_opened);

                if last_opened_match == false {
                    return false;
                }

                open_parantheses.pop();
                continue;
            }
            open_parantheses.push(char_as_string);
        }

        return open_parantheses.len() == 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_empty() {
        assert_eq!(
            Solution::is_valid("".to_string()),
            true
        );
    }

    #[test]
    fn valid_round() {
        assert_eq!(
            Solution::is_valid("()".to_string()),
            true
        );
    }

    #[test]
    fn valid_square() {
        assert_eq!(
            Solution::is_valid("[]".to_string()),
            true
        );
    }

    #[test]
    fn valid_curly() {
        assert_eq!(
            Solution::is_valid("{}".to_string()),
            true
        );
    }

    #[test]
    fn valid_list() {
        assert_eq!(
            Solution::is_valid("()[]{}".to_string()),
            true
        );
    }

    #[test]
    fn valid_nested_pair() {
        assert_eq!(
            Solution::is_valid("([{}])".to_string()),
            true
        );
    }

    #[test]
    fn valid_nested_list() {
        assert_eq!(
            Solution::is_valid("([{}[]])".to_string()),
            true
        );
    }

    #[test]
    fn valid_repeated_nested() {
        assert_eq!(
            Solution::is_valid("([(())])".to_string()),
            true
        );
    }

    #[test]
    fn invalid_backwards() {
        assert_eq!(
            Solution::is_valid("][".to_string()),
            false
        );
    }

    #[test]
    fn invalid_pair() {
        assert_eq!(
            Solution::is_valid("(}".to_string()),
            false
        );
    }

    #[test]
    fn invalid_nested() {
        assert_eq!(
            Solution::is_valid("({])".to_string()),
            false
        );
    }

    #[test]
    fn invalid_never_closed() {
        assert_eq!(
            Solution::is_valid("()[".to_string()),
            false
        );
    }
}
