use std::io::{BufRead,BufReader,Read,Lines};
use types::change::{Change};
use types::changes_lines::{ChangesLines};

pub struct ChangesStream<T: Read> {
    source: Lines<BufReader<T>>
}

pub struct Full<T: Read> {
    stream: ChangesStream<T>
}

pub struct Changes<T: Read> {
    stream: Full<T>
}

impl<T: Read> ChangesStream<T> {
    pub fn new(source: T) -> ChangesStream<T> {
        ChangesStream { source: BufReader::new(source).lines() }
    }

    pub fn full(self) -> Full<T> {
        Full { stream: self }
    }

    pub fn changes(self) -> Changes<T> {
        Changes { stream: self.full() }
    }
}

impl<T: Read> Iterator for Full<T> {
    type Item = ChangesLines;

    #[inline]
    fn next(&mut self) -> Option<ChangesLines> {
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

impl<T: Read> Iterator for Changes<T> {
    type Item = Change;

    #[inline]
    fn next(&mut self) -> Option<Change> {
        if let Some(next) = self.stream.next() {
            next.to_change()
        } else {
            None
        }
    }
}
