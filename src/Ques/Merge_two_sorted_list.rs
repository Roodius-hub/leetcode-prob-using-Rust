pub struct ListNode {
    pub val:i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val:i32) -> Self {
        ListNode {
            next:None,
            val
        }
    }
}


pub fn merge_two_lists(mut list1: Option<Box<ListNode>>,mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

    let mut dummy = Box::new(ListNode::new(0));
    let mut tail  = &mut dummy;

    while list1.is_some() && list2.is_some() {
        // compare
        let val1 = list1.as_ref().unwrap().val;
        let val2 = list2.as_ref().unwrap().val;

        if val1 <= val2 {
            let mut node = list1.take().unwrap();
            list1 = node.next.take();
            tail.next = Some(node);
        } else {
            let mut node = list2.take().unwrap();
            list2 = node.next.take();
            tail.next = Some(node);
        }
        tail = tail.next.as_mut().unwrap();
    }
    tail.next = if list1.is_some() {list1.take()} else {list2.take()};

    dummy.next
}

fn from_vec(vals: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for val in vals.into_iter().rev() {
        let mut node = Box::new(ListNode::new(val));
        node.next = head;
        head = Some(node);
    }
    head
}

fn to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    while let Some(node) = head {
        result.push(node.val);
        head = node.next;
    }
    result
}

fn main () {
    let list1 = from_vec(vec![1, 2, 4]);
    let list2 = from_vec(vec![1, 3, 4]);

    let merged = merge_two_lists(list1, list2);

    println!("Merged list: {:?}", to_vec(merged));

}