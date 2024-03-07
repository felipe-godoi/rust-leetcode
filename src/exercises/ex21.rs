#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    if list1.is_none() && list2.is_none() {
        return None;
    }

    if list1.is_none() || list2.is_none() {
        return list1.or(list2);
    }

    let mut head1 = &mut list1;
    let mut head2 = list2.clone().unwrap();
    let mut aux: Option<Box<ListNode>>;

    while head1.next.is_some() {
        if head1.val > head2.val {
            aux = head2.next;
            head2.next = Some(Box::from(head1.to_owned()));
            if aux.is_none() {
                break;
            }

            head1 = aux.as_mut().unwrap();
        } else {
            aux = head1.next;
            head1.next = Some(head2);
            if aux.is_none() {
                break;
            }

            head2 = aux.unwrap();
        }
    }

    Some(list1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_list(list: Vec<i32>) -> Box<ListNode> {
        let mut created = Box::from(ListNode::new(list[0]));
        let mut list_head = &mut created;

        for element in list[1..].into_iter() {
            let node = Box::from(ListNode::new(*element));

            list_head.next = Some(node);
            list_head = list_head.next.as_mut().unwrap();
        }

        created
    }

    #[test]
    fn test_case1() {
        let list1 = create_list(vec![1, 3, 4]);
        let list2 = create_list(vec![1, 3, 4]);
        let expected = create_list(vec![1, 1, 2, 3, 4, 4]);
        assert_eq!(merge_two_lists(Some(list1), Some(list2)), Some(expected));
    }
}
