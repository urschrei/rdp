

// distance formula
pub fn distance(start: &(f64, f64), end: &(f64, f64)) -> f64 {
    ((start.0 - end.0).powf(2.) + (start.1 - end.1).powf(2.)).sqrt()
}

// perpendicular distance from a point to a line
pub fn point_line_distance(point: &(f64, f64), start: &(f64, f64), end: &(f64, f64)) -> f64 {
    if start == end {
        return distance(*&point, *&start);
    } else {

        let n = ((end.0 - start.0) * (start.1 - point.1) - (start.0 - point.0) * (end.1 - start.1))
            .abs();
        let d = ((end.0 - start.0).powf(2.0) + (end.1 - start.1).powf(2.0)).sqrt();
        n / d
    }
}

// Ramer–Douglas-Peucker line simplification algorithm
// It's OK to use unwrap here for now
pub fn rdp(points: &[(f64, f64)], epsilon: &f64) -> Vec<(f64, f64)> {
    let mut dmax = 1.0;
    let mut index: usize = 0;
    let mut distance: f64;
    for (i, _) in points.iter().enumerate().take(points.len() - 1).skip(1) {
        distance = point_line_distance(&points[i],
                                       &*points.first().unwrap(),
                                       &*points.last().unwrap());
        if distance > dmax {
            index = i;
            dmax = distance;
        }
    }
    if dmax > *epsilon {
        let mut intermediate = rdp(&points[..index + 1], &*epsilon);
        intermediate.pop();
        // recur!
        intermediate.extend_from_slice(&rdp(&points[index..(points.len() - 1)], &*epsilon));
        intermediate
    } else {
        vec![*points.first().unwrap(), *points.last().unwrap()]
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
        let points = vec![(0.0, 0.0), (5.0, 4.0), (11.0, 5.5), (17.3, 3.2), (27.8, 0.1)];
        let foo: Vec<_> = rdp(&points, &1.0);
        assert_eq!(foo, vec![(0.0, 0.0), (5.0, 4.0), (11.0, 5.5), (17.3, 3.2)]);
    }
}
