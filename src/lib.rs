

// distance formula
pub fn distance(start: &(f64, f64), end: &(f64, f64)) -> f64 {
    ((start.0 - end.0).powf(2.) + (start.1 - end.1).powf(2.)).sqrt()
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
}
