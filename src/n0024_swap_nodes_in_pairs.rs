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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut swapped: Vec<i32> = vec![];

        let mut node = head;
        let mut previous_val: Option<i32> = None;

        while node.is_some() {
            if previous_val.is_some() {
                swapped.push(node.as_ref().unwrap().val);
                swapped.push(previous_val.take().unwrap());
            } else {
                previous_val = node.as_ref().map(|n| n.val);
            }

            node = node.and_then(|n| n.next);
        }

        if previous_val.is_some() {
            swapped.push(previous_val.unwrap());
        }

        return Solution::build_list(&swapped);
    }

    fn build_list(nodes: &Vec<i32>) -> Option<Box<ListNode>> {
        if nodes.len() < 1 {
            return None;
        }

        let mut node = ListNode::new(nodes[0]);
        node.next = Solution::build_list(&nodes.to_owned().split_off(1));

        return Some(Box::new(node));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(
            Solution::swap_pairs(
                Solution::build_list(&vec![]),
            ),
            Solution::build_list(&vec![]),
        );
    }

    #[test]
    fn single_node() {
        assert_eq!(
            Solution::swap_pairs(
                Solution::build_list(&vec![1]),
            ),
            Solution::build_list(&vec![1]),
        );
    }

    #[test]
    fn one_pair() {
        assert_eq!(
            Solution::swap_pairs(
                Solution::build_list(&vec![1, 2]),
            ),
            Solution::build_list(&vec![2, 1]),
        );
    }

    #[test]
    fn two_pairs() {
        assert_eq!(
            Solution::swap_pairs(
                Solution::build_list(&vec![1, 2, 3, 4]),
            ),
            Solution::build_list(&vec![2, 1, 4, 3]),
        );
    }

    #[test]
    fn three_pairs() {
        assert_eq!(
            Solution::swap_pairs(
                Solution::build_list(&vec![1, 2, 3, 4, 5, 6]),
            ),
            Solution::build_list(&vec![2, 1, 4, 3, 6, 5]),
        );
    }

    #[test]
    fn odd_pairs() {
        assert_eq!(
            Solution::swap_pairs(
                Solution::build_list(&vec![1, 2, 3, 4, 5]),
            ),
            Solution::build_list(&vec![2, 1, 4, 3, 5]),
        );
    }
}
