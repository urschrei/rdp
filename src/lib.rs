#![doc(html_logo_url = "https://cdn.rawgit.com/urschrei/rdp/6c84264fd9cdc0b8fdf974fc98e51fea4834ed05/rdp.svg",
       html_root_url = "https://urschrei.github.io/rdp/")]
//! This crate provides a Rust implementation of the Ramer–Douglas–Peucker line simplification algorithm,
//! and FFI functions for accessing it from third-party libraries.
use std::mem;
use std::slice;
use std::f64;

extern crate libc;
use self::libc::{c_void, size_t, c_double};

extern crate num;
use num::Float;

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
pub extern "C" fn simplify_linestring_ffi(coords: Array, precision: c_double) -> Array {
    LineString(Vec::from(coords)
            .iter()
            .map(|i| Point::new(i[0], i[1]))
            .collect())
        .simplify(&precision)
        .into()
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

#[cfg(test)]
mod tests {
    use super::{simplify_linestring_ffi, drop_float_array, Array};
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
        assert_eq!(converted,
                   vec![[0.0, 0.0], [5.0, 4.0], [11.0, 5.5], [17.3, 3.2], [27.8, 0.1]]);
        // drop it
        let ls = LineString(converted.iter().map(|i| Point::new(i[0], i[1])).collect());
        drop_float_array(ls.into());
    }
    #[test]
    fn test_ffi_coordinate_simplification() {
        let input = vec![[0.0, 0.0], [5.0, 4.0], [11.0, 5.5], [17.3, 3.2], [27.8, 0.1]];
        let ls = LineString(input.iter().map(|i| Point::new(i[0], i[1])).collect());
        let output = vec![[0.0, 0.0], [5.0, 4.0], [11.0, 5.5], [27.8, 0.1]];
        let transformed: Vec<_> = simplify_linestring_ffi(ls.into(), 1.0).into();
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
