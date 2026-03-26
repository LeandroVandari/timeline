use crate::when::When;

struct LevelTracker<T> {
    thing: T,
    level: u8,
}

trait Leveled<F: Fn() -> i16> {
    fn get_level_function(&self) -> F;
}

impl<T> LevelTracker<T> {
    pub fn lower_level(&mut self) {
        self.level -= 1
    }

    pub fn increase_level(&mut self) {
        self.level += 1
    }
}

pub struct DateIterator {
    curr: LevelTracker<When>,
    end: When,
}
