pub struct Solution {}

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let sorted_nums = Solution::sort_nums(nums.clone());

        let mut closest_difference: i32 = 0;
        let mut closest_sum: i32 = 0;

        let mut index_a: usize = 0;
        while index_a < (nums.len() - 2) {
            let mut index_b: usize = index_a + 1;
            while index_b < (nums.len() - 1) {
                let mut index_c: usize = index_b + 1;
                while index_c < nums.len() {

                    let sum = sorted_nums[index_a] + sorted_nums[index_b] + sorted_nums[index_c];

                    println!(
                        "sum of {} + {} + {} is {}",
                        sorted_nums[index_a],
                        sorted_nums[index_b],
                        sorted_nums[index_c],
                        sum
                    );

                    if sum == target {
                        return sum;
                    }

                    let difference = (sum - target).abs();
                    if closest_difference == 0 || difference < closest_difference {
                        closest_difference = difference;
                        closest_sum = sum;
                    }

                    if sum > target {
                        break;
                    }

                    index_c = Solution::find_next_index(&sorted_nums, index_c);
                }

                index_b = Solution::find_next_index(&sorted_nums, index_b);
            }

            index_a = Solution::find_next_index(&sorted_nums, index_a);
        }

        return closest_sum;
    }

    fn find_next_index(nums: &Vec<i32>, current_index: usize) -> usize {
        let mut index = current_index + 1;
        while index < nums.len() {
            if nums[current_index] != nums[index] {
                return index;
            }
            index = index + 1;
        }

        return nums.len();
    }

    fn sort_nums(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();
        return nums;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_not_reached() {
        assert_eq!(
            2,
            Solution::three_sum_closest(vec![-1, 2, 1, -4], 1)
        );
    }

    #[test]
    fn test_target_reached() {
        assert_eq!(
            2,
            Solution::three_sum_closest(vec![-1, 2, 1, -4], 2)
        );
    }

    #[test]
    fn test_negative_target_not_reached() {
        assert_eq!(
            -4,
            Solution::three_sum_closest(vec![-1, 2, 1, -4], -7)
        );
    }

    #[test]
    fn test_negative_target_reached() {
        assert_eq!(
            -2,
            Solution::three_sum_closest(vec![-1, 2, 3, -4], -2)
        );
    }

    #[test]
    fn test_repeated_numbers() {
        assert_eq!(
            3,
            Solution::three_sum_closest(vec![1,1,-1,-1,3], 3)
        );
    }

    #[test]
    fn test_doubles() {
        assert_eq!(
            82, // 2 + 16 + 64
            Solution::three_sum_closest(vec![1,2,4,8,16,32,64,128], 82)
        );
    }
}
