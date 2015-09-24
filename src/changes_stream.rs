use std::io::{BufRead,BufReader,Read,Lines};
use std::marker::PhantomData;
use serde::de::Deserialize;
use types::change::{Change};
use types::changes_lines::{ChangesLines};

pub struct ChangesStream<T: Read, D: Deserialize> {
    source: Lines<BufReader<T>>,
    documents: PhantomData<D>
}

pub struct Full<T: Read, D: Deserialize> {
    stream: ChangesStream<T, D>
}

pub struct Changes<T: Read, D: Deserialize> {
    stream: Full<T, D>
}

impl<T: Read, D: Deserialize> ChangesStream<T,D> {
    pub fn new(source: T) -> ChangesStream<T,D> {
        ChangesStream { source: BufReader::new(source).lines(), documents: PhantomData }
    }

    pub fn full(self) -> Full<T, D> {
        Full { stream: self }
    }

    pub fn changes(self) -> Changes<T, D> {
        Changes { stream: self.full() }
    }
}

impl<T: Read, D: Deserialize> Iterator for Full<T, D> {
    type Item = ChangesLines<D>;

    #[inline]
    fn next(&mut self) -> Option<ChangesLines<D>> {
        if let Some(elem) = self.stream.source.next() {
            elem.ok().iter()
                .filter_map(|line| {
                    ChangesLines::parse(line).ok()
                }).nth(0)
        } else {
            None
        }
        
    }
}

impl<T: Read, D: Deserialize> Iterator for Changes<T, D> {
    type Item = Change<D>;

    #[inline]
    fn next(&mut self) -> Option<Change<D>> {
        if let Some(next) = self.stream.next() {
            next.to_change()
        } else {
            None
        }
    }
}
