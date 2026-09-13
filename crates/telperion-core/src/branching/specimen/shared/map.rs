//! Persistent ordered radix pages. Editing a key copies its path's index pages;
//! unchanged payloads remain shared, including across structural insertions.
use std::sync::Arc;
const BITS: u32 = 5;
const SIZE: usize = 1 << BITS;
#[derive(Clone)]
enum Page<V: Clone> {
    Branch([Option<Arc<Page<V>>>; SIZE]),
    Leaf([Option<Arc<V>>; SIZE]),
}
#[derive(Clone)]
pub(super) struct Map<V: Clone> {
    root: Arc<Page<V>>,
    level: u32,
    len: usize,
}
impl<V: Clone> Default for Map<V> {
    fn default() -> Self {
        Self {
            root: Arc::new(Page::Leaf(std::array::from_fn(|_| None))),
            level: 0,
            len: 0,
        }
    }
}
impl<V: Clone> Map<V> {
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn get(&self, key: u64) -> Option<&V> {
        if key.checked_shr(BITS * (self.level + 1)).unwrap_or(0) != 0 {
            return None;
        }
        let mut page = self.root.as_ref();
        let mut level = self.level;
        loop {
            let index = ((key >> (level * BITS)) & (SIZE as u64 - 1)) as usize;
            match page {
                Page::Leaf(values) => return values[index].as_deref(),
                Page::Branch(children) => {
                    page = children[index].as_deref()?;
                    level -= 1;
                }
            }
        }
    }
    pub fn set(&mut self, key: u64, value: Option<V>) {
        let existed = self.get(key).is_some();
        if value.is_none() && !existed {
            return;
        }
        self.len = self.len + usize::from(value.is_some()) - usize::from(existed);
        while key.checked_shr(BITS * (self.level + 1)).unwrap_or(0) != 0 {
            let mut children = std::array::from_fn(|_| None);
            children[0] = Some(self.root.clone());
            self.root = Arc::new(Page::Branch(children));
            self.level += 1;
        }
        write(&mut self.root, self.level, key, value.map(Arc::new));
    }
    pub fn iter(&self) -> Iter<'_, V> {
        Iter {
            stack: vec![(&self.root, 0)],
        }
    }
}
fn write<V: Clone>(page: &mut Arc<Page<V>>, level: u32, key: u64, value: Option<Arc<V>>) {
    let index = ((key >> (level * BITS)) & (SIZE as u64 - 1)) as usize;
    match Arc::make_mut(page) {
        Page::Leaf(values) => values[index] = value,
        Page::Branch(children) => {
            let child = children[index].get_or_insert_with(|| {
                Arc::new(if level == 1 {
                    Page::Leaf(std::array::from_fn(|_| None))
                } else {
                    Page::Branch(std::array::from_fn(|_| None))
                })
            });
            write(child, level - 1, key, value);
        }
    }
}
pub(super) struct Iter<'a, V: Clone> {
    stack: Vec<(&'a Page<V>, usize)>,
}
impl<'a, V: Clone> Iterator for Iter<'a, V> {
    type Item = &'a V;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((page, at)) = self.stack.last_mut() {
            if *at == SIZE {
                self.stack.pop();
                continue;
            }
            let index = *at;
            *at += 1;
            match page {
                Page::Leaf(values) => {
                    if let Some(value) = &values[index] {
                        return Some(value);
                    }
                }
                Page::Branch(children) => {
                    if let Some(child) = &children[index] {
                        self.stack.push((child, 0));
                    }
                }
            }
        }
        None
    }
}
