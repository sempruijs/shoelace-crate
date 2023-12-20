pub struct Point {
    x: i64,
    y: i64,
}

impl From<(i64, i64)> for Point {
    fn from(pair: (i64, i64)) -> Self {
        Point {
            x: pair.0,
            y: pair.1,
        }
    }
}

pub struct Area(f64);

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
            dbg!(&area);
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
}
