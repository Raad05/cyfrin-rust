pub fn first<X, Y>(t: (X, Y)) -> X {
    t.0
}

pub fn last<X, Y>(t: (X, Y)) -> Y {
    t.1
}

#[derive(Debug)]
pub struct Rectangle<T> {
    pub top: T,
    pub left: T,
    pub width: T,
    pub height: T,
}
