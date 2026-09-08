//! A half-open interval calendar for conflict detection and availability queries.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Slot { pub start: u32, pub end: u32 }

impl Slot {
    pub fn new(start: u32, end: u32) -> Option<Self> {
        (start < end).then_some(Self { start, end })
    }
    pub fn overlaps(self, other: Slot) -> bool {
        self.start < other.end && other.start < self.end
    }
}

#[derive(Debug, Default)]
pub struct Calendar { bookings: Vec<Slot> }

impl Calendar {
    pub fn book(&mut self, slot: Slot) -> bool {
        if self.bookings.iter().copied().any(|existing| existing.overlaps(slot)) { return false; }
        self.bookings.push(slot);
        self.bookings.sort_by_key(|item| item.start);
        true
    }

    pub fn bookings(&self) -> &[Slot] { &self.bookings }

    pub fn next_available(&self, requested: Slot, duration: u32) -> Option<Slot> {
        if duration == 0 || requested.start.saturating_add(duration) > requested.end { return None; }
        let mut candidate = requested.start;
        for booking in self.bookings.iter().copied().filter(|slot| slot.end > requested.start) {
            if booking.start > candidate && candidate.saturating_add(duration) <= booking.start {
                return Slot::new(candidate, candidate + duration);
            }
            candidate = candidate.max(booking.end);
            if candidate.saturating_add(duration) > requested.end { return None; }
        }
        Slot::new(candidate, candidate + duration)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn finds_gap_between_bookings() {
        let mut calendar = Calendar::default();
        calendar.book(Slot::new(9, 10).unwrap());
        calendar.book(Slot::new(12, 13).unwrap());
        assert_eq!(calendar.next_available(Slot::new(8, 14).unwrap(), 2), Slot::new(10, 12));
    }
}
