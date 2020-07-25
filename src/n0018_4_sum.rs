pub struct Solution {}

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return vec![];
        }

        let sorted_nums = &Solution::sort_nums(&nums);

        let mut quadruplets = vec![];

        let mut index: usize = 0;
        while index < (sorted_nums.len() - 3) {
            let triplets = Solution::three_sum(
                sorted_nums,
                index + 1,
                target - sorted_nums[index]
            );
            for triplet in triplets {
                quadruplets.push(vec![sorted_nums[index], triplet[0], triplet[1], triplet[2]]);
            }

            index = Solution::next_index(&sorted_nums, index);
        }

        return quadruplets;
    }

    fn three_sum(sorted_nums: &Vec<i32>, initial_index: usize, target: i32) -> Vec<Vec<i32>> {
        let mut triplets = vec![];

        let mut index: usize = initial_index;
        while index < (sorted_nums.len() - 2) {
            let pairs = Solution::two_sum(
                sorted_nums,
                index + 1,
                target - sorted_nums[index]
            );
            for pair in pairs {
                triplets.push(vec![sorted_nums[index], pair[0], pair[1]]);
            }

            index = Solution::next_index(&sorted_nums, index);
        }

        return triplets;
    }

    fn two_sum(sorted_nums: &Vec<i32>, initial_index: usize, target: i32) -> Vec<Vec<i32>> {
        let mut pairs = vec![];

        let mut start: usize = initial_index;
        let mut end: usize = sorted_nums.len() - 1;
        while start < end {
            let sum = sorted_nums[start] + sorted_nums[end];
            if sum > target {
                end = Solution::prev_index(sorted_nums, end);
            } else if sum < target {
                start = Solution::next_index(sorted_nums, start);
            } else {
                let pair = vec![sorted_nums[start], sorted_nums[end]];
                pairs.push(pair);
                start = Solution::next_index(sorted_nums, start);
                end = Solution::prev_index(sorted_nums, end);
            }
        }

        return pairs;
    }

    fn next_index(sorted_nums: &Vec<i32>, current_index: usize) -> usize {
        let mut index: usize = current_index + 1;
        while index < sorted_nums.len() && sorted_nums[index] == sorted_nums[current_index] {
            index = index + 1;
        }
        return index;
    }

    fn prev_index(sorted_nums: &Vec<i32>, current_index: usize) -> usize {
        let mut index: usize = current_index - 1;
        while index > 0 && sorted_nums[index] == sorted_nums[current_index] {
            index = index - 1;
        }
        return index;
    }

    fn sort_nums(nums: &Vec<i32>) -> Vec<i32> {
        let mut nums_copy = nums.to_owned();
        nums_copy.sort();
        return nums_copy;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_vec() -> Vec<Vec<i32>> {
        return Vec::new();
    }

    #[test]
    fn test_too_few_nums() {
        assert_eq!(
            Solution::four_sum(
                vec![1, 0, -1],
                0
            ),
            empty_vec(),
        );
    }

    #[test]
    fn test_match() {
        assert_eq!(
            Solution::four_sum(
                vec![1, 0, -1, 0, -2, 2],
                0
            ),
            vec![
                vec![-2, -1, 1, 2],
                vec![-2,  0, 0, 2],
                vec![-1, 0, 0, 1],
            ],
        );
    }

    #[test]
    fn test_no_matches() {
        assert_eq!(
            Solution::four_sum(
                vec![1, 0, -1, 0, -2, 2],
                9
            ),
            empty_vec(),
        );
    }

    #[test]
    fn test_repeated_numbers() {
        assert_eq!(
            Solution::four_sum(
                vec![1, 2, 3, 4, 1, 2, 3, 4],
                10
            ),
            vec![
                [1, 1, 4, 4],
                [1, 2, 3, 4],
                [2, 2, 3, 3],
            ],
        );
    }
}
