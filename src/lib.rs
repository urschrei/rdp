#![doc(
    html_logo_url = "https://cdn.rawgit.com/urschrei/rdp/6c84264fd9cdc0b8fdf974fc98e51fea4834ed05/rdp.svg",
    html_root_url = "https://urschrei.github.io/rdp/"
)]
//! This crate provides FFI functions for accessing the Ramer–Douglas–Peucker and Visvalingam-Whyatt line simplification algorithms

use std::f64;
use std::mem;
use std::slice;

use libc;

use self::num_traits::Float;
use num_traits;

use self::geo::simplify::{Simplify, SimplifyIdx};
use self::geo::simplifyvw::{SimplifyVW, SimplifyVWPreserve, SimplifyVwIdx};
use self::geo::LineString;
use geo;

/// A C-compatible `struct` used for passing arrays across the FFI boundary
#[repr(C)]
pub struct Array {
    pub data: *const libc::c_void,
    pub len: libc::size_t,
}

// Build an Array from a LineString, so it can be leaked across the FFI boundary
impl<T> From<LineString<T>> for Array
where
    T: Float,
{
    fn from(sl: LineString<T>) -> Self {
        let v: Vec<[T; 2]> = sl.0.iter().map(|p| [p.x, p.y]).collect();
        let array = Array {
            data: v.as_ptr() as *const libc::c_void,
            len: v.len() as libc::size_t,
        };
        mem::forget(v);
        array
    }
}

// Build an Array from a vec of usize, so it can be leaked across the FFI boundary
impl From<Vec<usize>> for Array {
    fn from(v: Vec<usize>) -> Self {
        let array = Array {
            data: v.as_ptr() as *const libc::c_void,
            len: v.len() as libc::size_t,
        };
        mem::forget(v);
        array
    }
}

// Build a 2D Vec from an Array
// Ideally this would be a LineString, but local types blah blah
impl From<Array> for Vec<[f64; 2]> {
    fn from(arr: Array) -> Self {
        unsafe { slice::from_raw_parts(arr.data as *mut [f64; 2], arr.len).to_vec() }
    }
}

// Build a Vec of usize from an Array
impl From<Array> for Vec<usize> {
    fn from(arr: Array) -> Self {
        unsafe { slice::from_raw_parts(arr.data as *mut usize, arr.len).to_vec() }
    }
}

/// FFI wrapper for RDP, returning simplified geometry **coordinates**
///
/// Callers must pass two arguments:
///
/// - a [Struct](struct.Array.html) with two fields:
///     - `data`, a void pointer to an array of floating-point point coordinates: `[[1.0, 2.0], ...]`
///     - `len`, the length of the array being passed. Its type must be `size_t`
/// - a double-precision `float` for the tolerance
///
/// Implementations calling this function **must** call [`drop_float_array`](fn.drop_float_array.html)
/// with the returned `Array` pointer, in order to free the memory it allocates.
///
/// # Safety
///
/// This function is unsafe because it accesses a raw pointer which could contain arbitrary data
#[no_mangle]
pub extern "C" fn simplify_rdp_ffi(coords: Array, precision: libc::c_double) -> Array {
    let v: Vec<[f64; 2]> = Vec::from(coords);
    let ls: LineString<_> = v.into();
    ls.simplify(&precision).into()
}

/// FFI wrapper for RDP, returning simplified geometry **indices**
///
/// Callers must pass two arguments:
///
/// - a [Struct](struct.Array.html) with two fields:
///     - `data`, a void pointer to an array of floating-point point coordinates: `[[1.0, 2.0], ...]`
///     - `len`, the length of the array being passed. Its type must be `size_t`
/// - a double-precision `float` for the tolerance
///
/// Implementations calling this function **must** call [`drop_usize_array`](fn.drop_usize_array.html)
/// with the returned `Array` pointer, in order to free the memory it allocates.
///
/// # Safety
///
/// This function is unsafe because it accesses a raw pointer which could contain arbitrary data
#[no_mangle]
pub extern "C" fn simplify_rdp_idx_ffi(coords: Array, precision: libc::c_double) -> Array {
    let v: Vec<[f64; 2]> = Vec::from(coords);
    let ls: LineString<_> = v.into();
    ls.simplify_idx(&precision).into()
}

/// FFI wrapper for Visvalingam-Whyatt, returning simplified geometry **coordinates**
///
/// Callers must pass two arguments:
///
/// - a [Struct](struct.Array.html) with two fields:
///     - `data`, a void pointer to an array of floating-point point coordinates: `[[1.0, 2.0], ...]`
///     - `len`, the length of the array being passed. Its type must be `size_t`
/// - a double-precision `float` for the epsilon
///
/// Implementations calling this function **must** call [`drop_float_array`](fn.drop_float_array.html)
/// with the returned `Array` pointer, in order to free the memory it allocates.
///
/// # Safety
///
/// This function is unsafe because it accesses a raw pointer which could contain arbitrary data
#[no_mangle]
pub extern "C" fn simplify_visvalingam_ffi(coords: Array, precision: libc::c_double) -> Array {
    let v: Vec<[f64; 2]> = Vec::from(coords);
    let ls: LineString<_> = v.into();
    ls.simplifyvw(&precision).into()
}

/// FFI wrapper for Visvalingam-Whyatt, returning simplified geometry **indices**
///
/// Callers must pass two arguments:
///
/// - a [Struct](struct.Array.html) with two fields:
///     - `data`, a void pointer to an array of floating-point point coordinates: `[[1.0, 2.0], ...]`
///     - `len`, the length of the array being passed. Its type must be `size_t`
/// - a double-precision `float` for the epsilon
///
/// Implementations calling this function **must** call [`drop_usize_array`](fn.drop_usize_array.html)
/// with the returned `Array` pointer, in order to free the memory it allocates.
///
/// # Safety
///
/// This function is unsafe because it accesses a raw pointer which could contain arbitrary data
#[no_mangle]
pub extern "C" fn simplify_visvalingam_idx_ffi(coords: Array, precision: libc::c_double) -> Array {
    let v: Vec<[f64; 2]> = Vec::from(coords);
    let ls: LineString<_> = v.into();
    ls.simplifyvw_idx(&precision).into()
}

/// FFI wrapper for topology-preserving Visvalingam-Whyatt, returning simplified geometry **coordinates**.
///
/// Callers must pass two arguments:
///
/// - a [Struct](struct.Array.html) with two fields:
///     - `data`, a void pointer to an array of floating-point point coordinates: `[[1.0, 2.0], ...]`
///     - `len`, the length of the array being passed. Its type must be `size_t`
/// - a double-precision `float` for the epsilon
///
/// Implementations calling this function **must** call [`drop_float_array`](fn.drop_float_array.html)
/// with the returned `Array` pointer, in order to free the memory it allocates.
///
/// # Safety
///
/// This function is unsafe because it accesses a raw pointer which could contain arbitrary data
#[no_mangle]
pub extern "C" fn simplify_visvalingamp_ffi(coords: Array, precision: libc::c_double) -> Array {
    let v: Vec<[f64; 2]> = Vec::from(coords);
    let ls: LineString<_> = v.into();
    ls.simplifyvw_preserve(&precision).into()
}

/// Free memory which has been allocated across the FFI boundary by:
/// - simplify_rdp_ffi
/// - simplify_visvalingam_ffi
/// - simplify_visvalingamp_ffi
///
/// # Safety
///
/// This function is unsafe because it accesses a raw pointer which could contain arbitrary data
#[no_mangle]
pub extern "C" fn drop_float_array(arr: Array) {
    if arr.data.is_null() {
        return;
    }
    let _: Vec<[f64; 2]> = arr.into();
}

/// Free memory which has been allocated across the FFI boundary by:
/// - simplify_rdp_idx_ffi
/// - simplify_visvalingam_idx_ffi
///
/// # Safety
///
/// This function is unsafe because it accesses a raw pointer which could contain arbitrary data
#[no_mangle]
pub extern "C" fn drop_usize_array(arr: Array) {
    if arr.data.is_null() {
        return;
    }
    let _: Vec<usize> = arr.into();
}

#[cfg(test)]
mod tests {
    use super::*;
    use geo;
    use geo::{LineString, Point};

    use std::ptr;
    #[test]
    fn test_linestring_to_array() {
        let ls: LineString<_> = vec![Point::new(1.0, 2.0), Point::new(3.0, 4.0)].into();
        let _: Array = ls.into();
    }
    #[test]
    fn test_array_conversion() {
        let original = vec![
            [0.0, 0.0],
            [5.0, 4.0],
            [11.0, 5.5],
            [17.3, 3.2],
            [27.8, 0.1],
        ];
        let ls: LineString<_> = original.clone().into();
        // move into an Array, and leak it
        let arr: Array = ls.into();
        // move back into a Vec -- leaked value still needs to be dropped
        let converted: Vec<[f64; 2]> = arr.into();
        assert_eq!(converted, original);
        // drop it
        let ls: LineString<_> = converted.into();
        drop_float_array(ls.into());
    }
    #[test]
    fn test_ffi_rdp_simplification() {
        let input = vec![
            [0.0, 0.0],
            [5.0, 4.0],
            [11.0, 5.5],
            [17.3, 3.2],
            [27.8, 0.1],
        ];
        let ls: LineString<_> = input.into();
        let output = vec![[0.0, 0.0], [5.0, 4.0], [11.0, 5.5], [27.8, 0.1]];
        let transformed: Vec<[f64; 2]> = simplify_rdp_ffi(ls.into(), 1.0).into();
        assert_eq!(transformed, output);
    }
    #[test]
    fn test_ffi_rdp_idx_simplification() {
        let input = vec![
            [0.0, 0.0],
            [5.0, 4.0],
            [11.0, 5.5],
            [17.3, 3.2],
            [27.8, 0.1],
        ];
        let ls: LineString<_> = input.into();
        // let output = vec![[0.0, 0.0], [5.0, 4.0], [11.0, 5.5], [27.8, 0.1]];
        let output = vec![0, 1, 2, 4];
        let transformed: Vec<usize> = simplify_rdp_idx_ffi(ls.into(), 1.0).into();
        assert_eq!(transformed, output);
    }
    #[test]
    fn test_ffi_visvalingam_simplification() {
        let input = vec![
            [5.0, 2.0],
            [3.0, 8.0],
            [6.0, 20.0],
            [7.0, 25.0],
            [10.0, 10.0],
        ];
        let ls: LineString<_> = input.into();
        let output = vec![[5.0, 2.0], [7.0, 25.0], [10.0, 10.0]];
        let transformed: Vec<[f64; 2]> = simplify_visvalingam_ffi(ls.into(), 30.0).into();
        assert_eq!(transformed, output);
    }
    #[test]
    fn test_ffi_visvalingam_idx_simplification() {
        let input = vec![
            [5.0, 2.0],
            [3.0, 8.0],
            [6.0, 20.0],
            [7.0, 25.0],
            [10.0, 10.0],
        ];
        let ls: LineString<_> = input.into();
        // let output = vec![[5.0, 2.0], [7.0, 25.0], [10.0, 10.0]];
        let output = vec![0, 3, 4];
        let transformed: Vec<usize> = simplify_visvalingam_idx_ffi(ls.into(), 30.0).into();
        assert_eq!(transformed, output);
    }
    #[test]
    fn test_ffi_visvalingamp_simplification() {
        let input = vec![
            [5.0, 2.0],
            [3.0, 8.0],
            [6.0, 20.0],
            [7.0, 25.0],
            [10.0, 10.0],
        ];
        let ls: LineString<_> = input.into();
        let output = vec![[5.0, 2.0], [7.0, 25.0], [10.0, 10.0]];
        let transformed: Vec<[f64; 2]> = simplify_visvalingamp_ffi(ls.into(), 30.0).into();
        assert_eq!(transformed, output);
    }
    #[test]
    fn test_drop_empty_float_array() {
        let original = vec![[1.0, 2.0], [3.0, 4.0]];
        let ls: LineString<_> = original.into();
        // move into an Array, and leak it
        let mut arr: Array = ls.into();
        // zero Array contents
        arr.data = ptr::null();
        drop_float_array(arr);
    }
}
