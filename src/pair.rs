pub struct Pair<T> {
    first: T,
    second: T,
}

impl <T> Pair<T> {
    pub fn from(first: T, second: T) -> Pair<T> {
        Pair {
            first,
            second,
        }
    }

    pub fn first(&self) -> &T {
        &self.first
    }

    pub fn second(&self) -> &T {
        &self.second
    }

    pub fn get(&self, index: i32) -> &T {
        match index {
            1 => self.first(),
            2 => self.second(),
            _ => panic!("Invalid index for pair: {}", index),
        }
    }
}