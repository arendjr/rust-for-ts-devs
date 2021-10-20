// Generics look similar to how they look in TypeScript...
// ... but only at a first glance.

pub struct Point<T> {
    timestamp: time::PrimitiveDateTime,
    value: T,
}

pub struct Series {
    points: Vec<Point<f64>>,
}
