// What if our Point struct wanted to store *references* instead?
#[derive(Clone)]
pub struct Point<'a, T> {
    timestamp: time::PrimitiveDateTime,
    value: &'a T, // We now require a lifetime specifier.
}

pub struct Series<'a> {
    points: Vec<Point<'a, f64>>,
}

impl<'a> Series<'a> {
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }

    pub fn from_points(points: &'a [Point<f64>]) -> Self {
        Self {
            points: points.to_owned(),
        }
    }
}
