#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub struct Solution {}

impl Solution {
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut nodes: Vec<i32> = vec![];

        let mut maybe_l1_head = &l1;
        let mut maybe_l2_head = &l2;

        while maybe_l1_head.is_some() || maybe_l2_head.is_some() {
            if maybe_l1_head.is_some() {
                let l1_head = maybe_l1_head.as_ref().unwrap();
                nodes.push(l1_head.val);
                maybe_l1_head = &l1_head.next;
            }
            if maybe_l2_head.is_some() {
                let l2_head = maybe_l2_head.as_ref().unwrap();
                nodes.push(l2_head.val);
                maybe_l2_head = &l2_head.next;
            }
        }

        nodes.sort();

        return Solution::build_list(&nodes);
    }

    fn build_list(nodes: &Vec<i32>) -> Option<Box<ListNode>> {
        if nodes.len() < 1 {
            return None;
        }

        let mut list = ListNode::new(nodes[0]);
        list.next = Solution::build_list(&nodes.to_owned().split_off(1));

        return Some(Box::new(list));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_of_equal_length() {
        assert_eq!(
            Solution::merge_two_lists(
                Solution::build_list(&vec![1, 2, 4]),
                Solution::build_list(&vec![1, 3, 4]),
            ),
            Solution::build_list(&vec![1, 1, 2, 3, 4, 4]),
        );
    }

    #[test]
    fn lists_of_different_lengths() {
        assert_eq!(
            Solution::merge_two_lists(
                Solution::build_list(&vec![1, 3]),
                Solution::build_list(&vec![1, 2, 4]),
            ),
            Solution::build_list(&vec![1, 1, 2, 3, 4]),
        );
    }

    #[test]
    fn left_list_only() {
        assert_eq!(
            Solution::merge_two_lists(
                Solution::build_list(&vec![1, 2, 4]),
                Solution::build_list(&vec![]),
            ),
            Solution::build_list(&vec![1, 2, 4]),
        );
    }

    #[test]
    fn right_list_only() {
        assert_eq!(
            Solution::merge_two_lists(
                Solution::build_list(&vec![]),
                Solution::build_list(&vec![1, 2, 4]),
            ),
            Solution::build_list(&vec![1, 2, 4]),
        );
    }

    #[test]
    fn both_empty() {
        assert_eq!(
            Solution::merge_two_lists(
                Solution::build_list(&vec![]),
                Solution::build_list(&vec![]),
            ),
            Solution::build_list(&vec![]),
        );
    }

    #[test]
    fn not_initially_sorted() {
        assert_eq!(
            Solution::merge_two_lists(
                Solution::build_list(&vec![2]),
                Solution::build_list(&vec![1]),
            ),
            Solution::build_list(&vec![1, 2]),
        );
    }

    #[test]
    fn longer_not_initially_sorted() {
        assert_eq!(
            Solution::merge_two_lists(
                Solution::build_list(&vec![2, 4, 6]),
                Solution::build_list(&vec![1, 3, 9]),
            ),
            Solution::build_list(&vec![1, 2, 3, 4, 6, 9]),
        );
    }
}
