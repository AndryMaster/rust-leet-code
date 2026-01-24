#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}


impl FromIterator<i32> for ListNode {
    fn from_iter<I: IntoIterator<Item=i32>>(iter: I) -> Self {
        let mut arr = Vec::new();
        for i in iter {
            arr.push(i)
        }
        if arr.is_empty() {
            panic!("empty list");
        }

        let mut head: ListNode = ListNode::new(arr.pop().unwrap());
        for mut node in arr.into_iter().map(|x| ListNode::new(x)).rev() {
            node.next = Some(Box::new(head));  // old head
            head = node;  // new head
        }

        head
    }
}


impl<'a> IntoIterator for &'a ListNode {
    type Item = i32;
    type IntoIter = ListNodeIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ListNodeIntoIterator {
            node: Some(&self)
        }
    }
}

pub struct ListNodeIntoIterator<'a> {
    node: Option<&'a ListNode>,
}

impl<'a> Iterator for ListNodeIntoIterator<'a> {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.node {
            self.node = node.next.as_deref();
            Some(node.val)
        } else {
            None
        }
    }
}
