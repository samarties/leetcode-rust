pub struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n == 0 {
            return vec![];
        }

        return Solution::calculate_variations(vec!["()".to_string()], n - 1);
    }

    fn calculate_variations(strings: Vec<String>, repeat: i32) -> Vec<String> {
        if repeat == 0 {
            return strings;
        }

        let mut variants: Vec<String> = vec![];
        for str in strings {
            variants.append(&mut Solution::add_parantheses(&str));
        }

        variants.sort();
        variants.dedup();

        return Solution::calculate_variations(variants, repeat - 1);
    }

    fn add_parantheses(str: &String) -> Vec<String> {
        let mut variants: Vec<String> = vec![];

        let mut pos: usize = 1;
        while pos <= str.len() {
            let mut variant = str.to_owned();
            variant.insert_str(pos, "()");
            variants.push(variant);

            pos = pos + 1;
        }

        return variants;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_vec() -> Vec<String> {
        return Vec::new();
    }

    #[test]
    fn zero() {
        assert_eq!(
            Solution::generate_parenthesis(0),
            empty_vec(),
        );
    }

    #[test]
    fn one() {
        assert_eq!(
            Solution::generate_parenthesis(1),
            vec![
                "()",
            ],
        );

        assert_eq!(
            Solution::generate_parenthesis(2),
            vec![
                "(())",
                "()()",
            ],
        );

        assert_eq!(
            Solution::generate_parenthesis(3),
            vec![
                "((()))",
                "(()())",
                "(())()",
                "()(())",
                "()()()",
            ],
        );
    }

    #[test]
    fn two() {
        assert_eq!(
            Solution::generate_parenthesis(2),
            vec![
                "(())",
                "()()",
            ],
        );

        assert_eq!(
            Solution::generate_parenthesis(3),
            vec![
                "((()))",
                "(()())",
                "(())()",
                "()(())",
                "()()()",
            ],
        );
    }

    #[test]
    fn three() {
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec![
                "((()))",
                "(()())",
                "(())()",
                "()(())",
                "()()()",
            ],
        );
    }
}
