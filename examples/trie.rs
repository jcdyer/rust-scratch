// use std::slice::SliceExt;

#[derive(Clone, Debug)]
struct Trie {
    children: Vec<Option<Trie>>,
    value: bool,
}

impl Trie {
    pub fn empty() -> Trie {
        let children = vec![None; 256];
        let value = false;
        Trie { children, value }
    }

    pub fn insert(&mut self, value: &[u8]) {
        match value.split_first() {
            Some((first, rest)) => {
                if let None = self.children[*first as usize] {
                    let trie = Trie::empty();
                    self.children[*first as usize] = Some(trie);
                };
                if let Some(ref mut child) = self.children[*first as usize] {
                    child.insert(rest);
                }
            },
            None => self.value = true,
        };
    }

    pub fn contains(&self, value: &[u8]) -> bool {
        match value.split_first() {
            Some((first, rest)) => {
                if let Some(ref child) = self.children[*first as usize] {
                    child.contains(rest)
                } else {
                    false
                }
            },
            None => self.value,
        }
    }
}

fn main() {
    let words = vec![&b"peter"[..], &b"piper"[..], &b"picked"[..], &b"a"[..], &b"peck"[..], &b"of"[..], &b"pickled"[..], &b"peppers"[..]];
    let mut trie = Trie::empty();
    for word in &words {
        trie.insert(&word);
    }
    let trie = trie;
    for word in &words {
        println!("Trie {} value {}", if trie.contains(&word) { "contains" } else { "doesn't contain" }, ::std::str::from_utf8(&word).unwrap());
    }

    for word in vec![&b"these"[..], &b"words"[..], &b"aren't"[..], &b"in"[..], &b"there"[..], &b"pi"[..]] {
        println!("Trie {} value {}", if trie.contains(&word) { "contains" } else { "doesn't contain" }, ::std::str::from_utf8(&word).unwrap());
    }
}
