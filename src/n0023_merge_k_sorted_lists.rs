use std::cmp::Ordering;
use std::collections::BinaryHeap;

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

impl PartialOrd<ListNode> for ListNode {
    fn partial_cmp(&self, other: &ListNode) -> Option<Ordering> {
        other.val.partial_cmp(&self.val)
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

pub struct Solution {}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        // return Solution::merge_lists_by_destructuring(&lists);
        return Solution::merge_lists_by_binary_heap(&lists);
    }

    pub fn merge_lists_by_binary_heap(lists: &Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();
        for node in lists {
            if node.is_some() {
                heap.push(node.to_owned().take()?)
            }
        }
        let mut head = heap.pop()?;
        let mut pointer = &mut head;
        while !heap.is_empty() {
            if pointer.next.is_some() {
                heap.push(pointer.next.take()?);
            }
            pointer.next = Some(heap.pop()?);
            pointer = pointer.next.as_mut()?;
        }
        Some(head)
    }

    fn merge_lists_by_destructuring(lists: &Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut nodes: Vec<i32> = vec![];

        for list in lists {
            nodes.append(&mut Solution::destructure_list(&list));
        }

        nodes.sort();

        return Solution::build_list(&nodes);
    }

    fn destructure_list(list: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut nodes: Vec<i32> = vec![];

        let mut maybe_head = list;
        while maybe_head.is_some() {
            if maybe_head.is_some() {
                let head = maybe_head.as_ref().unwrap();
                nodes.push(head.val);
                maybe_head = &head.next;
            }
        }

        return nodes;
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
    fn no_lists() {
        assert_eq!(
            Solution::merge_k_lists(
                vec![]
            ),
            Solution::build_list(&vec![]),
        );
    }

    #[test]
    fn one_list() {
        assert_eq!(
            Solution::merge_k_lists(
                vec![
                    Solution::build_list(&vec![1, 4, 5]),
                ]
            ),
            Solution::build_list(&vec![1, 4, 5]),
        );
    }

    #[test]
    fn two_list() {
        assert_eq!(
            Solution::merge_k_lists(
                vec![
                    Solution::build_list(&vec![1, 4, 5]),
                    Solution::build_list(&vec![1, 3, 4]),
                ]
            ),
            Solution::build_list(&vec![1, 1, 3, 4, 4, 5]),
        );
    }

    #[test]
    fn three_lists() {
        assert_eq!(
            Solution::merge_k_lists(
                vec![
                    Solution::build_list(&vec![1, 4, 5]),
                    Solution::build_list(&vec![1, 3, 4]),
                    Solution::build_list(&vec![2, 6]),
                ]
            ),
            Solution::build_list(&vec![1, 1, 2, 3, 4, 4, 5, 6]),
        );
    }
}
