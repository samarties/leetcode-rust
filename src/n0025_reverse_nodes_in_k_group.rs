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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        return Solution::reverse_k_group_without_vec(head, k);
    }

    pub fn reverse_k_group_without_vec(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k < 2 {
            return head;
        }

        let mut new_head: Option<Box<ListNode>> = None;
        let mut tail = &mut new_head;
        let mut k_group: Option<Box<ListNode>> = None;
        loop {
            for k_len in 0..k {
                if let Some(mut node) = head {
                    head = node.next.take();
                    node.next = k_group;
                    k_group = Some(node);
                } else {
                    // k not reached

                    let mut rev_k_group: Option<Box<ListNode>> = None;
                    for _ in 0..k_len {
                        let mut node = k_group.unwrap();
                        k_group = node.next.take();
                        node.next = rev_k_group;
                        rev_k_group = Some(node);
                    }
                    *tail = rev_k_group;
                    return new_head;
                }
            }

            *tail = k_group;
            for _ in 0..k {
                tail = &mut tail.as_mut().unwrap().next;
            }
            k_group = None;
        }
    }

    pub fn reverse_k_group_using_vec(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k < 2 {
            return head;
        }

        let mut swapped: Vec<i32> = vec![];

        let mut node = head;
        let mut previous_vals: Vec<i32> = vec![];

        let k_as_usize = k as u32 as usize;

        while node.is_some() {
            previous_vals.insert(0, node.as_ref().unwrap().val);

            if previous_vals.len() == k_as_usize {
                swapped.append(&mut previous_vals);
            }

            node = node.and_then(|n| n.next);
        }

        if previous_vals.len() > 0 {
            previous_vals.reverse();
            swapped.append(&mut previous_vals);
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
    fn when_empty() {
        assert_eq!(
            Solution::reverse_k_group(
                Solution::build_list(&vec![]),
                2,
            ),
            Solution::build_list(&vec![]),
        );
    }

    #[test]
    fn when_short() {
        assert_eq!(
            Solution::reverse_k_group(
                Solution::build_list(&vec![1, 2, 3, 4]),
                5,
            ),
            Solution::build_list(&vec![1, 2, 3, 4]),
        );
    }

    #[test]
    fn when_exact() {
        assert_eq!(
            Solution::reverse_k_group(
                Solution::build_list(&vec![1, 2, 3, 4, 5]),
                5,
            ),
            Solution::build_list(&vec![5, 4, 3, 2, 1]),
        );
    }

    #[test]
    fn when_one_group() {
        assert_eq!(
            Solution::reverse_k_group(
                Solution::build_list(&vec![1, 2, 3, 4, 5]),
                1,
            ),
            Solution::build_list(&vec![1, 2, 3, 4, 5]),
        );
    }

    #[test]
    fn when_two_group() {
        assert_eq!(
            Solution::reverse_k_group(
                Solution::build_list(&vec![1, 2, 3, 4, 5]),
                2,
            ),
            Solution::build_list(&vec![2, 1, 4, 3, 5]),
        );
    }

    #[test]
    fn when_three_group() {
        assert_eq!(
            Solution::reverse_k_group(
                Solution::build_list(&vec![1, 2, 3, 4, 5]),
                3,
            ),
            Solution::build_list(&vec![3, 2, 1, 4, 5]),
        );
    }
}
