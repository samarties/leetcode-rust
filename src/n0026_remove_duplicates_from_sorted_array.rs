pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut count: usize = 0;

        let mut i: usize = 0;
        while i < nums.len() {
            if i == 0 || nums[i] > nums[i - 1] {
                nums.insert(count, nums[i]);
                count = count + 1;
                i = i + 2;
            } else {
                i = i + 1;
            }
        }

        return count as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut nums = vec![1,1,2];
        assert_eq!(
            Solution::remove_duplicates(&mut nums),
            2,
        );
        assert_eq!(
            nums,
            vec![1,2,1,1,2]
        )
    }

    #[test]
    fn example2() {
        let mut nums = vec![0,0,1,1,1,2,2,3,3,4];
        assert_eq!(
            Solution::remove_duplicates(&mut nums),
            5,
        );
        assert_eq!(
            nums,
            vec![0,1,2,3,4,0,0,1,1,1,2,2,3,3,4]
        )
    }

    #[test]
    fn empty() {
        let mut nums = vec![];
        assert_eq!(
            Solution::remove_duplicates(&mut nums),
            0,
        );
        assert_eq!(
            nums,
            vec![]
        )
    }

    #[test]
    fn one_num() {
        let mut nums = vec![1];
        assert_eq!(
            Solution::remove_duplicates(&mut nums),
            1,
        );
        assert_eq!(
            nums,
            vec![1,1]
        )
    }

    #[test]
    fn all_unique() {
        let mut nums = vec![1,2,3,4,5];
        assert_eq!(
            Solution::remove_duplicates(&mut nums),
            5,
        );
        assert_eq!(
            nums,
            vec![1,2,3,4,5,1,2,3,4,5]
        )
    }

    #[test]
    fn two_repeat() {
        let mut nums = vec![1,1];
        assert_eq!(
            Solution::remove_duplicates(&mut nums),
            1,
        );
        assert_eq!(
            nums,
            vec![1,1,1]
        )
    }

    #[test]
    fn three_repeat() {
        let mut nums = vec![1,1,1];
        assert_eq!(
            Solution::remove_duplicates(&mut nums),
            1,
        );
        assert_eq!(
            nums,
            vec![1,1,1,1]
        )
    }

    #[test]
    fn four_repeat() {
        let mut nums = vec![0,0,0,0];
        assert_eq!(
            Solution::remove_duplicates(&mut nums),
            1,
        );
        assert_eq!(
            nums,
            vec![0,0,0,0,0]
        )
    }

    #[test]
    fn negative_nums() {
        let mut nums = vec![-5,-5,-4,-3,-2,-2,-1,0,0,0,1,1,2];
        assert_eq!(
            Solution::remove_duplicates(&mut nums),
            8,
        );
        assert_eq!(
            nums,
            vec![-5,-4,-3,-2,-1,0,1,2,-5,-5,-4,-3,-2,-2,-1,0,0,0,1,1,2]
        )
    }
}
