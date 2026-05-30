#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}


pub fn merge_two_lists_rec(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> { 
    match (list1, list2) {
        (None, None) => None,
        (Some(v), None) | (None, Some(v)) => Some(v),
        (Some(mut l1), Some(mut l2)) => {
            if l1.val <= l2.val {
                l1.next = merge_two_lists_rec(l1.next.take(), Some(l2));
                Some(l1)
            } else {
                l2.next = merge_two_lists_rec(Some(l1), l2.next.take());
                Some(l2)
            }
        }
    }
}

// Helper: build a list from a vector
fn from_vec(vals: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for val in vals.into_iter().rev() {
        let mut node = Box::new(ListNode::new(val));
        node.next = head;
        head = Some(node);
    }
    head
}

// Helper: convert a list back to a vector for printing
fn to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    while let Some(node) = head {
        result.push(node.val);
        head = node.next;
    }
    result
}

fn main() {
    let list1 = from_vec(vec![1, 2, 4]);
    let list2 = from_vec(vec![1, 3, 4]);

    let merged = merge_two_lists_rec(list1, list2);
    println!("Merged list: {:?}", to_vec(merged));
}
