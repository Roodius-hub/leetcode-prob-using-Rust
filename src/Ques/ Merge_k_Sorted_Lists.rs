use std::vec;


#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val:i32,
    pub next: Option<Box<ListNode>>    
}

impl ListNode {
    #[inline]
    fn new(val:i32) -> Self {
        ListNode {
            val,
            next:None
        }
    }
}

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut vals = Vec::new();

    for mut list in lists {
        while let Some(node) = list {
            vals.push(node.val);
            list = node.next;
        }
    }

    vals.sort();

    make_list(vals)

}

pub fn make_list(nums:Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;

    for n in nums.into_iter().rev() {
        let mut node = Box::new(ListNode::new(n));
        node.next = head;
        head = Some(node);
    }
    head
}

fn main(){
    let nums:Vec<i32> = vec![1,2,2,4,5,6,5];
    let lists:Vec<Option<Box<ListNode>>> = vec![make_list(vec![1,2,3,4,5]),make_list(vec![2,3,4,5,6,6])];
    let  ans  = merge_k_lists(lists);
    
    let mut curr = ans.as_ref();

    while let Some(node) = curr {
        print!(" {} ", node.val);
        curr = node.next.as_ref();
    }

}