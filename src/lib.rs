#![doc(html_logo_url = "https://cdn.rawgit.com/urschrei/rdp/6c84264fd9cdc0b8fdf974fc98e51fea4834ed05/rdp.svg",
       html_root_url = "https://urschrei.github.io/rdp/")]
//! This crate provides a Rust implementation of the Ramer–Douglas–Peucker line simplification algorithm,
//! and FFI functions for accessing it from third-party libraries. 
use std::mem;
use std::slice;
use std::f64;

extern crate libc;
use self::libc::{c_void, size_t, c_double};

/// A C-compatible `struct` used for passing arrays across the FFI boundary
#[repr(C)]
pub struct Array {
    pub data: *const c_void,
    pub len: size_t,
}

// Build an Array from a Vec, so it can be leaked across the FFI boundary
impl From<Vec<[f64; 2]>> for Array {
    fn from(sl: Vec<[f64; 2]>) -> Self {
        let array = Array {
            data: sl.as_ptr() as *const c_void,
            len: sl.len() as size_t,
        };
        mem::forget(sl);
        array
    }
}

// Build a Vec from an Array
impl From<Array> for Vec<[f64; 2]> {
    fn from(arr: Array) -> Self {
        unsafe { slice::from_raw_parts(arr.data as *mut [f64; 2], arr.len).to_vec() }
    }
}

/// FFI wrapper for [`rdp`](fn.rdp.html)
///
/// Callers must pass two arguments:
///
/// - a [Struct](struct.Array.html) with two fields:
///     - `data`, a void pointer to an array of floating-point point coordinates: `[[1.0, 2.0], …]`
///     - `len`, the length of the array being passed. Its type must be `size_t`
/// - a double-precision `float` for the tolerance
///
/// Implementations calling this function **must** call [`drop_float_array`](fn.drop_float_array.html)
/// with the returned `c_char` pointer, in order to free the memory it allocates.
///
/// # Safety
///
/// This function is unsafe because it accesses a raw pointer which could contain arbitrary data
#[no_mangle]
pub extern "C" fn simplify_linestring_ffi(coords: Array, precision: c_double) -> Array {
    let inc: Vec<_> = coords.into();
    rdp(&inc, &precision).into()
}

/// Free Array memory which Rust has allocated across the FFI boundary by [`simplify_linestring_ffi`](fn.simplify_linestring_ffi.html)
///
/// # Safety
///
/// This function is unsafe because it accesses a raw pointer which could contain arbitrary data
#[no_mangle]
pub extern "C" fn drop_float_array(arr: Array) {
    if arr.data.is_null() {
        return;
    }
    let _: Vec<_> = arr.into();
}

// distance formula
fn distance(start: &[f64; 2], end: &[f64; 2]) -> f64 {
    let (dx, dy) = (start[0] - end[0], start[1] - end[1]);
    dx.hypot(dy)
}

// perpendicular distance from a point to a line
fn point_line_distance(point: &[f64; 2], start: &[f64; 2], end: &[f64; 2]) -> f64 {
    if start == end {
        distance(*&point, *&start)
    } else {
        let numerator = ((end[0] - start[0]) * (start[1] - point[1]) -
                         (start[0] - point[0]) * (end[1] - start[1]))
            .abs();
        let denominator = distance(start, end);
        numerator / denominator
    }
}

// It's OK to use unwrap here for now
/// Simplify a linestring using the [Ramer–Douglas–Peucker](https://en.wikipedia.org/wiki/Ramer–Douglas–Peucker_algorithm) algorithm
pub fn rdp(points: &[[f64; 2]], epsilon: &f64) -> Vec<[f64; 2]> {
    if points.is_empty() || points.len() == 1 {
        return points.to_vec()
    }
    let mut dmax = 0.0;
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
        intermediate.extend_from_slice(&rdp(&points[index..], &*epsilon));
        intermediate
    } else {
        vec![*points.first().unwrap(), *points.last().unwrap()]
    }
}

#[cfg(test)]
mod tests {
    use super::{rdp, distance, point_line_distance, simplify_linestring_ffi, drop_float_array, Array};
    use std::ptr;
    #[test]
    fn test_distance() {
        let start = [0.0, 0.0];
        let end = [3.0, 4.0];
        assert_eq!(distance(&start, &end), 5.);
    }
    #[test]
    fn test_point_line_distance() {
        let point = [1.0, 1.0];
        let start = [1.0, 2.0];
        let end = [3.0, 4.0];
        assert_eq!(point_line_distance(&point, &start, &end),
                   0.7071067811865475);
    }
    #[test]
    fn test_rdp() {
        let points = vec![[0.0, 0.0], [5.0, 4.0], [11.0, 5.5], [17.3, 3.2], [27.8, 0.1]];
        let foo: Vec<_> = rdp(&points, &1.0);
        assert_eq!(foo, vec![[0.0, 0.0], [5.0, 4.0], [11.0, 5.5], [27.8, 0.1]]);
    }
    #[test]
    fn test_rdp_empty() {
        let points = vec![];
        let foo: Vec<_> = rdp(&points, &1.0);
        assert_eq!(foo, points);
    }
    #[test]
    fn test_rdp_one() {
        let points = vec![[5.0, 4.0]];
        let foo: Vec<_> = rdp(&points, &1.0);
        assert_eq!(foo, points);
    }
    #[test]
    fn test_array_conversion() {
        let original = vec![[0.0, 0.0], [5.0, 4.0], [11.0, 5.5], [17.3, 3.2], [27.8, 0.1]];
        // move into an Array, and leak it
        let arr: Array = original.into();
        // move back into a Vec -- leaked value still needs to be dropped
        let converted: Vec<_> = arr.into();
        assert_eq!(converted,
                   vec![[0.0, 0.0], [5.0, 4.0], [11.0, 5.5], [17.3, 3.2], [27.8, 0.1]]);
        // drop it
        drop_float_array(converted.into());
    }
    #[test]
    fn test_ffi_coordinate_simplification() {
        let input = vec![[0.0, 0.0], [5.0, 4.0], [11.0, 5.5], [17.3, 3.2], [27.8, 0.1]];
        let output = vec![[0.0, 0.0], [5.0, 4.0], [11.0, 5.5], [27.8, 0.1]];
        let transformed: Vec<_> = simplify_linestring_ffi(input.into(), 1.0).into();
        assert_eq!(transformed, output);
    }
    #[test]
    fn test_drop_empty_float_array() {
        let original = vec![[1.0, 2.0], [3.0, 4.0]];
        // move into an Array, and leak it
        let mut arr: Array = original.into();
        // zero Array contents
        arr.data = ptr::null();
        drop_float_array(arr);
    }
}
