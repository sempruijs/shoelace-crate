use std::fmt::Display;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn origin() -> Self {
        Point { x: 0, y: 0 }
    }
}

impl From<(i64, i64)> for Point {
    fn from(pair: (i64, i64)) -> Self {
        Point {
            x: pair.0,
            y: pair.1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Area(pub f64);

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl From<Vec<Point>> for Area {
    fn from(points: Vec<Point>) -> Self {
        let mut result = 0;
        let points = points.into_iter().rev().collect::<Vec<Point>>();
        let start_point = &points[0];
        for (i, point) in points.iter().enumerate() {
            let next_point = match points.len() - 1 == i {
                true => &start_point,
                false => &points[i + 1],
            };
            let area = (point.x * next_point.y) - (point.y * next_point.x);
            result += area;
        }
        let total_area = result as f64 * 0.5;
        Area(total_area)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_points_to_area() {
        let points = vec![
            Point::from((4, 4)),
            Point::from((5, -2)),
            Point::from((-1, -4)),
            Point::from((-6, 0)),
            Point::from((-2, 5)),
            Point::from((0, 1)),
        ];

        let r_1 = Area::from(points).0;
        let expected_1: f64 = 55.0;
        assert_eq!(r_1, expected_1);
    }

    #[test]
    fn test_origin() {
        let result = Point::origin();
        let expected = Point { x: 0, y: 0 };
        assert_eq!(expected, result);
    }

    #[test]
    fn test_display_point() {
        let result = format!("{}", Point::origin());
        let expected = "(0,0)";
        assert_eq!(result, expected);
    }
}
