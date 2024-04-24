use std::fmt::Display;

/// A Point represents an [x, y] location on a grid
#[derive(PartialEq, Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ x: {}, y: {} ]", &self.x, &self.y)
    }
}

impl Point {
    /// Adds two Points and returns the result as a new Point
    pub fn add(point1: &Self, point2: &Self) -> Self {
        Self {
            x: point1.x + point2.x,
            y: point1.y + point2.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Point;

    #[test]
    fn add_points() {
        let point1 = Point { x: 1, y: 2 };
        let point2 = Point { x: 2, ..point1 };

        let result = Point::add(&point1, &point2);
        assert_eq!(result, Point { x: 3, y: 4 });
    }

    #[test]
    fn format_point() {
        let point1 = Point { x: 1, y: 2 };
        let result = format!("{}", point1);
        assert_eq!(result, "[ x: 1, y: 2 ]")
    }
}
