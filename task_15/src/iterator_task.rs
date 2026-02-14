use std::cell::Cell;
use std::iter::FromIterator;

#[derive(Debug, PartialEq)]
pub struct SimpleLog {
    entries: Vec<String>,
}

impl SimpleLog {
   pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn add(&mut self, entry: impl Into<String>) {
        self.entries.push(entry.into());
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

pub struct SimpleLogIter<'a> {
    log: &'a SimpleLog,
    front: Cell<usize>,
    back: Cell<usize>,
}

impl Iterator for SimpleLogIter<'_> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.front.get() < self.log.len() {
            let index = self.front.get();
            let entry = self.log.entries[index].clone();
            self.front.set(index + 1);
            Some(entry)
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a SimpleLog {
    type Item = String;
    type IntoIter = SimpleLogIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SimpleLogIter { log: &self, front: Cell::new(0), back: Cell::new(self.len()) }
    }
}

impl<'a> DoubleEndedIterator for SimpleLogIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.back.get() > self.front.get() {
            let index = self.back.get() - 1;
            self.back.set(index);
            Some(self.log.entries[index].clone())
        } else {
            None
        }
    }
}

impl IntoIterator for SimpleLog {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

impl FromIterator<String> for SimpleLog {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        Self { entries: iter.into_iter().collect::<Vec<String>>()}
    }
}
