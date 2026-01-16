// Definition for singly-linked list.
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

fn add_two_num(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    
    let mut carry = 0;
    let mut result = Box::new(ListNode::new(0));
    let mut curr = &mut result;

    while l1.is_some() || l2.is_some() || carry != 0 {
        let x = l1.as_ref().map_or(0, |n| n.val);
        let y = l2.as_ref().map_or(0, |n| n.val);

        let sum = x + y + carry;
        carry = sum / 10;

        curr.next = Some(Box::new(ListNode::new(sum % 10)));
        curr = curr.next.as_mut().unwrap();

        l1 = l1.and_then(|n| n.next);
        l2 = l2.and_then(|n| n.next);
    }
    result.next
}

// for create linked list 
fn vec_list(nums:Vec<i32>) -> Option<Box<ListNode>> {
    let mut head  = None;
    let mut curr  = &mut head;
        
    for val in nums {
        *curr = Some(Box::new(ListNode::new(val)));
        curr = &mut curr.as_mut().unwrap().next;
    }
    head
}

// Helper: Print linked list
fn print_list(mut head:Option<Box<ListNode>>) {
    while let Some(node) = head {
        print!("{}",node.val);
        
        head = node.next;
        
        if head.is_some() {
            print!( " -> ");
        }
    }
    // println!();
}

fn main() {
    let l1  =  vec_list(vec![5,4,3]);
    let l2 = vec_list(vec![5,6,4]);
            
    print!("l1: ");
    print_list(l1.clone());
    
    println!();
    
    print!("l2: ");
    print_list(l2.clone());
    
    println!();
    
    let result = add_two_num(l1, l2);
    
    print!("Res: ");
    print_list(result);
    
}
