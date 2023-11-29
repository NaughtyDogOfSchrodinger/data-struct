#[derive(Default)]
struct Node {
    end: bool,
    children: [Option<Box<Node>>; 26],
}

#[derive(Default)]
struct Trie {
    root: Node,
}

#[allow(unused)]
impl Trie {
    pub fn insert(&mut self, word: &str) {
        println!("{word}");
        let mut node = &mut self.root;
        for char in word.as_bytes() {
            let index = (char - b'a') as usize;
            let next = &mut node.children[index];
            node = next.get_or_insert(Box::<Node>::default());
        }
        node.end = true;
    }

    pub fn contains(&mut self, word: &str) -> bool {
        if word.is_empty() {
            false
        } else {
            let mut node = &mut self.root;
            for char in word.as_bytes() {
                let index = (char - b'a') as usize;
                let next = &mut node.children[index];
                if let Some(n) = next {
                    node = n;
                } else {
                    return false;
                }
            }
            node.end
        }
    }
}

#[cfg(test)]
mod test {
    use crate::trie::Trie;
    use rand::Rng;
    use std::collections::BinaryHeap;

    #[test]
    fn test_insert_and_query() {
        let mut trie = Trie::default();
        trie.insert("hj");
        assert!(trie.contains("hj"));
        assert!(!trie.contains("hjb"));
    }

    fn random_string(n: usize) -> String {
        const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

        let mut rng = rand::thread_rng();
        (0..n)
            .map(|_| {
                let idx = rng.gen_range(0..CHARS.len());
                CHARS[idx] as char
            })
            .collect()
    }

    #[test]
    fn print_use_of_mem() {
        let mut trie = Trie::default();
        let cap = 1_000_000;
        for _ in 0..cap {
            let rand_str: String = random_string(8);
            trie.insert(&rand_str);
        }
    }
}
