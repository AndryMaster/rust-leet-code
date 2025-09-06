// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn collect(&self) -> Vec<i32> {
        let mut result = vec!(self.val);

        let mut next = self;
        while next.next.is_some() {
            next = next.next.as_ref().unwrap();
            result.push(next.val);
        }

        result
    }
}
