# RDP
A Rust implementation of the [Ramer–Douglas-Peucker](https://en.wikipedia.org/wiki/Ramer–Douglas–Peucker_algorithm) line simplification algorithm.

Also available from [crates.io](https://crates.io/crates/rdp)  
## FFI
The shared library exposes a FFI: `simplify_linestring_ffi`.  
### Arguments
- A C-compatible `struct` containing the following fields:
    - `data`: a void pointer to a 2D array of double-precision floats: `[[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]]``
    - `len`: a `size_t` denoting the length of the array (i.e. 3, above)
- A precision parameter, double_precision `float`. E.g. `1.0`

The return type is the same `struct` as above, containing the simplified linestring coordinates.  
### Freeing FFI Memory
Callers **must** call `drop_float_array()`, passing the returned `struct` in order to free the memory that the shared library has allocated. Failure to do so will result in memory leaks.
### Example Implementation
A Python 2.7/3.5 implementation can be found at [`ffi.py`](ffi.py
)

# License
[MIT](license.txt)
