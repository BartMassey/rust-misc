/// A count. Deliberately does not implement most traits.
#[derive(Default)]
pub struct Count(u64);

impl Count {
    /// Increase the count by one.
    pub fn incr(&mut self) {
        self.0 += 1;
    }

    /// Return the current count.
    pub fn value(&self) -> u64 {
        self.0
    }
}
