///
/// An Iterator that will skip a given range
/// of the underlying Iterator
///
struct SkipIterator<T> where T: Iterator {
    iter: T,        // the wrapped iterator
    start: usize,   // item index to start skipping
    end: usize,     // item index to stop skipping
    count: usize,   // number of items iterated
}
impl <T> Iterator for SkipIterator<T> where T: Iterator {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.next();
        while item.is_some() {
            self.count += 1;
            if self.count > self.start && self.count <= self.end {
                // do not emit item in skip range
                continue
            }
            return item
        }
        None
    }

}
pub fn skip_iterator<T>(list: impl IntoIterator<Item = T>, skip_start: usize, skip_end: usize) -> impl Iterator<Item = T> {
    SkipIterator{iter: list.into_iter(), start: skip_start, end: skip_end, count: 0}
}
