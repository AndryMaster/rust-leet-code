pub struct Trie {
    children: [Option<Box<Trie>>; 2]
}

impl Trie {
    pub fn new() -> Self {
        Self {
            children: [None, None]
        }
    }

    pub fn insert(&mut self, num: i32) {
        let mut cur_node = self;

        for bit_pos in (0..31).rev() {
            let bit = ((num >> bit_pos) & 1) as usize;
            if (cur_node.children[bit]).is_none() {
                cur_node.children[bit] = Some(Box::new(Trie::new()));
            }
            cur_node = cur_node.children[bit].as_mut().unwrap();
        }
    }

    pub fn search(&mut self, num: i32) -> i32 {
        let mut cur_node = self;
        let mut max_xor = 0;

        for bit_pos in (0..31).rev() {
            let bit = ((num >> bit_pos) & 1) as usize;
            if cur_node.children[bit^1].is_some() {
                cur_node = cur_node.children[bit^1].as_mut().unwrap();
                max_xor |= 1 << bit_pos;
            }
            else {
                cur_node = cur_node.children[bit].as_mut().unwrap();
            }
        }
        
        max_xor
    }
}
