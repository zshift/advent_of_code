use std::ops::Range;

pub trait Overlap {
    fn overlaps(&self, other: &Self) -> bool;
    fn merge(&self, other: &Self) -> Self;
}

impl Overlap for Range<u64> {
    fn overlaps(&self, other: &Self) -> bool {
        self.start <= other.end && other.start <= self.end
    }

    fn merge(&self, other: &Self) -> Self {
        self.start.min(other.start)..self.end.max(other.end)
    }
}
