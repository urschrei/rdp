

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

// Ramer–Douglas-Peucker line simplification algorithm
// Test with an epsilon of 1.0
pub fn rdp(points: &[(f64, f64)], epsilon: &f64) -> Vec<(f64, f64)> {
    let mut dmax = 1.0;
    let mut index: usize = 0;
    for i in 1..points.len() - 1 {
        let distance = point_line_distance(&points[i],
                                           &points.first().unwrap(),
                                           &points.last().unwrap());
        if distance > dmax {
            index = i;
            dmax = distance;
        }
    }
    if dmax >= *epsilon {
        let mut first: Vec<(f64, f64)> = rdp(&points[..index + 1], &epsilon);
        // remove last element
        first.pop();
        first.extend_from_slice(&rdp(&points[index..], &epsilon));
        first
    } else {
        let other = vec![*points.first().unwrap(), *points.last().unwrap()];
        other
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

    #[test]
    fn test_rdp() {
        let foo: Vec<_> = rdp(&[(0.0, 0.0), (10.0, 10.0), (11.0, 11.0), (11.2, 13.45)],
                              &1.0);
        assert_eq!(foo, vec![(1.0, 2.0), (3.0, 4.0)]);
    }
}
