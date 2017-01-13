#![doc(html_logo_url = "https://cdn.rawgit.com/urschrei/rdp/6c84264fd9cdc0b8fdf974fc98e51fea4834ed05/rdp.svg",
       html_root_url = "https://urschrei.github.io/rdp/")]
//! This crate provides FFI functions for accessing the Ramer–Douglas–Peucker and Visvalingam-Whyatt line simplification algorithms

use std::mem;
use std::slice;
use std::f64;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

extern crate libc;
use self::libc::{c_void, size_t, c_double};

extern crate num;
use self::num::Float;

extern crate geo;
use self::geo::{Point, LineString};
use self::geo::simplify::Simplify;

/// A C-compatible `struct` used for passing arrays across the FFI boundary
#[repr(C)]
pub struct Array {
    pub data: *const c_void,
    pub len: size_t,
}

// Build an Array from a LineString, so it can be leaked across the FFI boundary
impl<T> From<LineString<T>> for Array
    where T: Float
{
    fn from(sl: LineString<T>) -> Self {
        let v: Vec<[T; 2]> = sl.0
            .iter()
            .map(|p| [p.x(), p.y()])
            .collect();
        let array = Array {
            data: v.as_ptr() as *const c_void,
            len: v.len() as size_t,
        };
        mem::forget(v);
        array
    }
}

// Build a Vec from an Array
// Ideally this would be a LineString, but local types blah blah
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
pub extern "C" fn simplify_rdp_ffi(coords: Array, precision: c_double) -> Array {
    LineString(Vec::from(coords)
            .iter()
            .map(|i| Point::new(i[0], i[1]))
            .collect())
        .simplify(&precision)
        .into()
}

/// FFI wrapper for [`visvalingam`](fn.visvalingam.html)
///
/// Callers must pass two arguments:
///
/// - a [Struct](struct.Array.html) with two fields:
///     - `data`, a void pointer to an array of floating-point point coordinates: `[[1.0, 2.0], …]`
///     - `len`, the length of the array being passed. Its type must be `size_t`
/// - a double-precision `float` for the epsilon
///
/// Implementations calling this function **must** call [`drop_float_array`](fn.drop_float_array.html)
/// with the returned `c_char` pointer, in order to free the memory it allocates.
///
/// # Safety
///
/// This function is unsafe because it accesses a raw pointer which could contain arbitrary data
#[no_mangle]
pub extern "C" fn simplify_visvalingam_ffi(coords: Array, precision: c_double) -> Array {
    LineString(Vec::from(coords)
            .iter()
            .map(|i| Point::new(i[0], i[1]))
            .collect())
        .simplifyvw(&precision)
        .into()
}

// A helper struct for `visvalingam`, defined out here because
// #[deriving] doesn't work inside functions.
#[derive(PartialEq, Debug)]
struct VScore<T>
    where T: Float
{
    area: T,
    current: usize,
    left: usize,
    right: usize,
}

// These impls give us a min-heap
impl<T> Ord for VScore<T>
    where T: Float
{
    fn cmp(&self, other: &VScore<T>) -> Ordering {
        other.area.partial_cmp(&self.area).unwrap()
    }
}

impl<T> PartialOrd for VScore<T>
    where T: Float
{
    fn partial_cmp(&self, other: &VScore<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Eq for VScore<T> where T: Float {}

/// Simplify a line using the [Visvalingam-Whyatt](http://www.tandfonline.com/doi/abs/10.1179/000870493786962263) algorithm
///
/// epsilon is the minimum triangle area
// The paper states that:
// If [the new triangle's] calculated area is less than that of the last point to be
// eliminated, use the latter's area instead.
// (This ensures that the current point cannot be eliminated
// without eliminating previously eliminated points)
// (Visvalingam and Whyatt 2013, p47)
// However, this does *not* apply if you're using a user-defined epsilon;
// It's OK to remove triangles with areas below the epsilon,
// then recalculate the new triangle area and push it onto the heap
pub fn visvalingam<T>(orig: &[Point<T>], epsilon: &T) -> Vec<Point<T>>
    where T: Float
{
    // No need to continue without at least three points
    if orig.len() < 3 || orig.is_empty() {
        return orig.to_vec();
    }

    let max = orig.len();

    // Adjacent retained points. Simulating the points in a
    // linked list with indices into `orig`. Big number (larger than or equal to
    // `max`) means no next element, and (0, 0) means deleted element.
    let mut adjacent: Vec<(_)> = (0..orig.len())
        .map(|i| {
            if i == 0 {
                (-1_i32, 1_i32)
            } else {
                ((i - 1) as i32, (i + 1) as i32)
            }
        })
        .collect();

    // Store all the triangles in a minimum priority queue, based on their area.
    // Invalid triangles are *not* removed if / when points
    // are removed; they're handled by skipping them as
    // necessary in the main loop. (This is handled by recording the
    // state in the VScore)
    let mut pq = BinaryHeap::new();
    // Compute the initial triangles, i.e. take all consecutive groups
    // of 3 points and form triangles from them
    for (i, win) in orig.windows(3).enumerate() {
        pq.push(VScore {
            area: area(win.first().unwrap(), &win[1], win.last().unwrap()),
            current: i + 1,
            left: i,
            right: i + 2,
        });
    }
    // While there are still points for which the associated triangle
    // has an area below the epsilon
    loop {
        let smallest = match pq.pop() {
            // We've exhausted all the possible triangles, so leave the main loop
            None => break,
            // This triangle's area is above epsilon, so skip it
            Some(ref x) if x.area > *epsilon => continue,
            //  This triangle's area is below epsilon: eliminate the associated point
            Some(s) => s,
        };
        let (left, right) = adjacent[smallest.current];
        // A point in this triangle has been removed since this VScore
        // was created, so skip it
        if left as i32 != smallest.left as i32 || right as i32 != smallest.right as i32 {
            continue;
        }
        // We've got a valid triangle, and its area is smaller than epsilon, so
        // remove it from the simulated "linked list"
        let (ll, _) = adjacent[left as usize];
        let (_, rr) = adjacent[right as usize];
        adjacent[left as usize] = (ll, right);
        adjacent[right as usize] = (left, rr);
        adjacent[smallest.current as usize] = (0, 0);

        // Now recompute the triangle area, using left and right adjacent points
        let choices = [(ll, left, right), (left, right, rr)];
        for &(ai, current_point, bi) in &choices {
            if ai as usize >= max || bi as usize >= max {
                // Out of bounds, i.e. we're on one edge
                continue;
            }
            let new_left = Point::new(orig[ai as usize].x(), orig[ai as usize].y());
            let new_current = Point::new(orig[current_point as usize].x(),
                                         orig[current_point as usize].y());
            let new_right = Point::new(orig[bi as usize].x(), orig[bi as usize].y());
            pq.push(VScore {
                area: area(&new_left, &new_current, &new_right),
                current: current_point as usize,
                left: ai as usize,
                right: bi as usize,
            });
        }
    }
    // Filter out the points that have been deleted, returning remaining points
    orig.iter()
        .zip(adjacent.iter())
        .filter_map(|(tup, adj)| { if *adj != (0, 0) { Some(*tup) } else { None } })
        .collect::<Vec<Point<T>>>()
}

// Area of a triangle given three vertices
fn area<T>(p1: &Point<T>, p2: &Point<T>, p3: &Point<T>) -> T
    where T: Float
{
    ((p1.x() - p3.x()) * (p2.y() - p3.y()) - (p2.x() - p3.x()) * (p1.y() - p3.y())).abs() /
    (T::one() + T::one())
}

pub trait SimplifyVW<T, Epsilon = T> {
    /// Returns the simplified representation of a LineString, using the [Visvalingam-Whyatt](http://www.tandfonline.com/doi/abs/10.1179/000870493786962263) algorithm  
    /// 
    /// See [here](https://bost.ocks.org/mike/simplify/) for a graphical explanation 
    ///
    /// ```
    /// use geo::{Point, LineString};
    /// use geo::algorithm::simplifyvw::{SimplifyVW};
    ///
    /// let mut vec = Vec::new();
    /// vec.push(Point::new(5.0, 2.0));
    /// vec.push(Point::new(3.0, 8.0));
    /// vec.push(Point::new(6.0, 20.0));
    /// vec.push(Point::new(7.0, 25.0));
    /// vec.push(Point::new(10.0, 10.0));
    /// let linestring = LineString(vec);
    /// let mut compare = Vec::new();
    /// compare.push(Point::new(5.0, 2.0));
    /// compare.push(Point::new(7.0, 25.0));
    /// compare.push(Point::new(10.0, 10.0));
    /// let ls_compare = LineString(compare);
    /// let simplified = linestring.simplifyvw(&30.0);
    /// assert_eq!(simplified, ls_compare)
    /// ```
    fn simplifyvw(&self, epsilon: &T) -> Self where T: Float;
}

impl<T> SimplifyVW<T> for LineString<T>
    where T: Float
{
    fn simplifyvw(&self, epsilon: &T) -> LineString<T> {
        LineString(visvalingam(&self.0, epsilon))
    }
}

/// Free Array memory which Rust has allocated across the FFI boundary by [`simplify_rdp_ffi`](fn.simplify_rdp_ffi.html)
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

#[cfg(test)]
mod tests {
    use super::{simplify_rdp_ffi, simplify_visvalingam_ffi, drop_float_array, Array};
    extern crate geo;
    use geo::{Point, LineString};
    extern crate num;
    use std::ptr;
    #[test]
    fn test_linestring_to_array() {
        let ls = LineString(vec![Point::new(1.0, 2.0), Point::new(3.0, 4.0)]);
        let _: Array = ls.into();
    }
    #[test]
    fn test_array_conversion() {
        let original = vec![[0.0, 0.0], [5.0, 4.0], [11.0, 5.5], [17.3, 3.2], [27.8, 0.1]];
        let ls = LineString(original.iter().map(|i| Point::new(i[0], i[1])).collect());
        // move into an Array, and leak it
        let arr: Array = ls.into();
        // move back into a Vec -- leaked value still needs to be dropped
        let converted: Vec<_> = arr.into();
        assert_eq!(converted, original);
        // drop it
        let ls = LineString(converted.iter().map(|i| Point::new(i[0], i[1])).collect());
        drop_float_array(ls.into());
    }
    #[test]
    fn test_ffi_rdp_simplification() {
        let input = vec![[0.0, 0.0], [5.0, 4.0], [11.0, 5.5], [17.3, 3.2], [27.8, 0.1]];
        let ls = LineString(input.iter().map(|i| Point::new(i[0], i[1])).collect());
        let output = vec![[0.0, 0.0], [5.0, 4.0], [11.0, 5.5], [27.8, 0.1]];
        let transformed: Vec<_> = simplify_rdp_ffi(ls.into(), 1.0).into();
        assert_eq!(transformed, output);
    }
    #[test]
    fn test_ffi_visvalingam_simplification() {
        let input = vec![[5.0, 2.0], [3.0, 8.0], [6.0, 20.0], [7.0, 25.0], [10.0, 10.0]];
        let ls = LineString(input.iter().map(|i| Point::new(i[0], i[1])).collect());
        let output = vec![[5.0, 2.0], [7.0, 25.0], [10.0, 10.0]];
        let transformed: Vec<_> = simplify_visvalingam_ffi(ls.into(), 30.0).into();
        assert_eq!(transformed, output);
    }
    #[test]
    fn test_drop_empty_float_array() {
        let original = vec![[1.0, 2.0], [3.0, 4.0]];
        let ls = LineString(original.iter().map(|i| Point::new(i[0], i[1])).collect());
        // move into an Array, and leak it
        let mut arr: Array = ls.into();
        // zero Array contents
        arr.data = ptr::null();
        drop_float_array(arr);
    }
}
