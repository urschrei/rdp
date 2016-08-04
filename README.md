# RDP
A Rust implementation of the [Ramer–Douglas-Peucker](https://en.wikipedia.org/wiki/Ramer–Douglas–Peucker_algorithm) line simplification algorithm.

Also available from [crates.io](https://crates.io/crates/rdp)  
## FFI
The shared library exposes a FFI: `simplify_linestring_ffi`.  
### Arguments
- A C-compatible `struct` containing the following fields:
    - `data`: a void pointer to a 2D array of double-precision floats: `[[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]]`
    - `len`: a `size_t` denoting the length of the array (i.e. `3`, above)
- A precision parameter, double_precision `float`. E.g. `1.0`

The return type is the same `struct` as above, containing the simplified linestring coordinates.  
### Freeing FFI Memory
Callers **must** call `drop_float_array()`, passing the returned `struct`, in order to free the memory that the shared library has allocated. Failure to do so will result in memory leaks.
### Example Implementation
A Python 2.7/3.5 implementation can be found at [`ffi.py`](ffi.py
)  
Run `cargo build --release`, then `python ffi.py` to test. It's also importable, exposing `simplify_linestring()` – call it with a coordinate list and a precision parameter. Allocated memory is dropped on exit.

# License
[MIT](license.txt)

# References
**Douglas, David H**, and **Thomas K Peucker**. 1973. *“Algorithms for the Reduction of the Number of Points Required to Represent a Digitized Line or Its Caricature.”* Cartographica: The International Journal for Geographic Information and Geovisualization 10 (2): 112–122. [DOI](http://dx.doi.org/10.3138/FM57-6770-U75U-7727)

**Ramer, Urs**. 1972. *“An Iterative Procedure for the Polygonal Approximation of Plane Curves.”* Computer Graphics and Image Processing 1 (3): 244–256. [DOI](http://dx.doi.org/10.1016/S0146-664X(72)80017-0)

# Notes
This implementation doesn't use the Convex Hull speedup ([Hershberger & Snoeyink](http://dl.acm.org/citation.cfm?id=902273), 1992). Its worst-case complexity is thus O(n<sup>2</sup>)
