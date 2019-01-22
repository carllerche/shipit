/// Iterate multiple times
pub struct Replay<T>
where
    T: Iterator,
{
    iter: T,
    items: Vec<T::Item>,
}

pub struct Iter<'a, T>
where
    T: Iterator,
{
    replay: &'a mut Replay<T>,
    pos: usize,
}

impl<T> Replay<T>
where
    T: Iterator,
{
    pub fn new(iter: T) -> Replay<T> {
        Replay {
            iter,
            items: vec![],
        }
    }

    pub fn iter(&mut self) -> Iter<T> {
        Iter {
            replay: self,
            pos: 0,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Iterator,
    T::Item: Clone,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.replay.items.len() {
            if let Some(value) = self.replay.iter.next() {
                self.replay.items.push(value);
            } else {
                return None;
            }
        }

        let ret = self.replay.items[self.pos].clone();
        self.pos += 1;

        Some(ret)
    }
}
