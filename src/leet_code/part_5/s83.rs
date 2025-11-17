#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }

    #[inline]
    pub fn to_vec(self) -> Vec<i32> {
        let mut items = vec![self.val];
        let mut current = self;

        while let Some(next) = current.next {
            current = *next;
            items.push(current.val);
        }

        items
    }

    #[inline]
    pub fn from_vec(items: Vec<i32>) -> Option<ListNode> {
        let mut dummy_head = Box::new(ListNode::new(0));
        let mut tail = &mut dummy_head;

        for &val in items.iter() {
            tail.next = Some(Box::new(ListNode::new(val)));
            tail = tail.next.as_mut().unwrap();
        }

        Some(*dummy_head.next?.to_owned())
    }
}
