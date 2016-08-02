

// distance formula
pub fn distance(start: &(f64, f64), end: &(f64, f64)) -> f64 {
    ((start.0 - end.0).powf(2.) + (start.1 - end.1).powf(2.)).sqrt()
}

// perpendicular distance from a point to a line
pub fn point_line_distance(point: &(f64, f64), start: &(f64, f64), end: &(f64, f64)) -> f64 {
    if start == end {
        return distance(&point, &start);
    } else {

        let n = ((end.0 - start.0) * (start.1 - point.1) - (start.0 - point.0) * (end.1 - start.1))
            .abs();
        let d = ((end.0 - start.0).powf(2.0) + (end.1 - start.1).powf(2.0)).sqrt();
        n / d
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let start = (0.0, 0.0);
        let end = (3.0, 4.0);
        assert_eq!(distance(&start, &end), 5.);
    }

    #[test]
    fn test_point_line_distance() {
        let point = (1.0, 1.0);
        let start = (1.0, 2.0);
        let end = (3.0, 4.0);
        assert_eq!(point_line_distance(&point, &start, &end),
                   0.7071067811865475);
    }
}
