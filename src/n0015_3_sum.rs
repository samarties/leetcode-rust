pub struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }

        let sorted_nums = Solution::sort_nums(nums.clone());
        let mut triplets = vec![];

        let mut index: usize = 0;
        while index < (nums.len() - 2) {
            triplets.append(&mut Solution::two_sum(&sorted_nums, index));
            index = Solution::next_index(&sorted_nums, index);
        }

        return triplets;
    }

    fn two_sum(sorted_nums: &Vec<i32>, index: usize) -> Vec<Vec<i32>> {
        let mut triplets = vec![];
        let mut start: usize = index + 1;
        let mut end: usize = sorted_nums.len() - 1;
        while start < end {
            let sum = sorted_nums[index] + sorted_nums[start] + sorted_nums[end];
            if sum > 0 {
                end = Solution::prev_index(&sorted_nums, end);
            } else if sum < 0 {
                start = Solution::next_index(&sorted_nums, start);
            } else {
                let triplet = vec![sorted_nums[index], sorted_nums[start], sorted_nums[end]];
                triplets.push(triplet);
                start = Solution::next_index(&sorted_nums, start);
                end = Solution::prev_index(&sorted_nums, end);
            }
        }

        return triplets;
    }

    fn next_index(sorted_nums: &Vec<i32>, mut index: usize) -> usize {
        let num = sorted_nums[index];
        while index < sorted_nums.len() && sorted_nums[index] == num {
            index = index + 1;
        }
        return index;
    }

    fn prev_index(sorted_nums: &Vec<i32>, mut index: usize) -> usize {
        let num = sorted_nums[index];
        while index > 0 && index < sorted_nums.len() && sorted_nums[index] == num {
            index = index - 1;
        }
        return index;
    }

    fn sort_nums(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();
        return nums;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_vec() -> Vec<Vec<i32>> {
        return Vec::new();
    }

    #[test]
    fn test_empty() {
        assert_eq!(
            empty_vec(),
            Solution::three_sum(vec![])
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            empty_vec(),
            Solution::three_sum(vec![0])
        );
    }

    #[test]
    fn test_three_zeros() {
        assert_eq!(
            vec![
                vec![0, 0, 0],
            ],
            Solution::three_sum(vec![0, 0, 0])
        );
    }

    #[test]
    fn test_two_nums() {
        assert_eq!(
            empty_vec(),
            Solution::three_sum(vec![9, -9])
        );
    }

    #[test]
    fn test_sort() {
        assert_eq!(
            vec![
                vec![-1, -1, 2],
            ],
            Solution::three_sum(vec![2, -1, -1])
        );
    }

    #[test]
    fn test_no_triplets() {
        assert_eq!(
            empty_vec(),
            Solution::three_sum(vec![0, -1, -2, 10])
        )
    }

    #[test]
    fn test_no_duplicates() {
        assert_eq!(
            vec![
                vec![-1, -1, 2],
            ],
            Solution::three_sum(vec![-1, -1, 2, -1, 2, -1])
        )
    }

    #[test]
    fn test_reuse_nums() {
        assert_eq!(
            vec![
                vec![-2,0,2],
                vec![-2,1,1],
            ],
            Solution::three_sum(vec![-2,0,1,1,2])
        );
    }
}
