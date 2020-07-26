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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let n_as_usize =  n as u32 as usize;
        if n_as_usize < 1 {
            return head;
        }

        let mut nodes: Vec<i32> = vec![];

        let mut maybe_moving_head = &head;
        while maybe_moving_head.is_some() {
            let moving_head = maybe_moving_head.as_ref().unwrap();
            nodes.push(moving_head.val);
            maybe_moving_head = &moving_head.next;
        }

        nodes.remove(nodes.len() - n_as_usize);

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
    fn remove_penultimate() {
        assert_eq!(
            Solution::remove_nth_from_end(
                Solution::build_list(&vec![1, 2, 3, 4, 5]),
                2
            ),
            Solution::build_list(&vec![1, 2, 3, 5])
        );
    }

    #[test]
    fn remove_last() {
        assert_eq!(
            Solution::remove_nth_from_end(
                Solution::build_list(&vec![1, 2, 3, 4, 5]),
                1
            ),
            Solution::build_list(&vec![1, 2, 3, 4])
        );
    }

    #[test]
    fn remove_second() {
        assert_eq!(
            Solution::remove_nth_from_end(
                Solution::build_list(&vec![1, 2, 3, 4, 5]),
                4
            ),
            Solution::build_list(&vec![1, 3, 4, 5])
        );
    }

    #[test]
    fn remove_first() {
        assert_eq!(
            Solution::remove_nth_from_end(
                Solution::build_list(&vec![1, 2, 3, 4, 5]),
                5
            ),
            Solution::build_list(&vec![2, 3, 4, 5])
        );
    }

    #[test]
    fn remove_zero() {
        assert_eq!(
            Solution::remove_nth_from_end(
                Solution::build_list(&vec![1, 2, 3, 4, 5]),
                0
            ),
            Solution::build_list(&vec![1, 2, 3, 4, 5])
        );
    }

    #[test]
    fn single_node() {
        assert_eq!(
            Solution::remove_nth_from_end(
                Solution::build_list(&vec![1]),
                1
            ),
            Solution::build_list(&vec![])
        );
    }
}
