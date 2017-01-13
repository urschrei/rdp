[![Build Status](https://travis-ci.org/urschrei/rdp.svg?branch=master)](https://travis-ci.org/urschrei/rdp) [![Build status](https://ci.appveyor.com/api/projects/status/fc3h27ef9uhwhq20?svg=true)](https://ci.appveyor.com/project/urschrei/rdp/branch/master) [![Coverage Status](https://coveralls.io/repos/github/urschrei/rdp/badge.svg?branch=master)](https://coveralls.io/github/urschrei/rdp?branch=master) [![](https://img.shields.io/crates/v/rdp.svg)](https://crates.io/crates/rdp)

# RDP
A Rust implementation of the [Ramer–Douglas-Peucker](https://en.wikipedia.org/wiki/Ramer–Douglas–Peucker_algorithm) and [Visvalingam-Whyatt](https://bost.ocks.org/mike/simplify/) line simplification algorithms.

**The main functionality underlying this crate has now migrated to [rust-geo](https://github.com/georust/rust-geo) as the [`Simplify`](https://georust.github.io/rust-geo/geo/algorithm/simplify/trait.Simplify.html) trait.**

# FFI
The shared library exposes a(n) FFI: `simplify_rdp_ffi`, and `simplify_visvalingam_ffi`.  
Some examples are available in [this Jupyter notebook](examples.ipynb).  
[**Simplification**](https://pypi.python.org/pypi/simplification/), a Python package which uses this shared library, is available from PyPi.

## Arguments
- A C-compatible `struct` containing the following fields:
    - `data`: a void pointer to a 2D array of double-precision floats: `[[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]]`
    - `len`: a `size_t` denoting the length of the array (i.e. `3`, above)
- A precision parameter, double_precision `float`. E.g. `1.0`

The return type is the same `struct` as above, containing the simplified linestring coordinates.  
## Freeing FFI Memory
Callers **must** call `drop_float_array()`, passing the returned `struct`, in order to free the memory that the shared library has allocated. Failure to do so will result in memory leaks.
### Example Implementation
A Python 2.7 / 3.5 / 3.6 implementation can be found at [`ffi.py`](ffi.py
)  
Run `cargo build --release`, then `python ffi.py` to test. It's also importable, exposing `simplify_linestring()` – call it with a coordinate list and a precision parameter. Allocated memory is dropped on exit.  

# License
[MIT](license.txt)

# References
**Douglas, D.H.**, **Peucker, T.K.**, 1973. *Algorithms for the reduction of the number of points required to represent a digitized line or its caricature*. Cartographica: The International Journal for Geographic Information and Geovisualization 10, 112–122. [DOI](http://dx.doi.org/10.3138/FM57-6770-U75U-7727)

**Ramer, U.**, 1972. *An iterative procedure for the polygonal approximation of plane curves*. Computer Graphics and Image Processing 1, 244–256. [DOI](http://dx.doi.org/10.1016/S0146-664X(72)80017-0)

**Visvalingam, M.**, **Whyatt, J.D.**, 1993. *Line generalisation by repeated elimination of points*. The Cartographic Journal 30, 46–51. [DOI](http://dx.doi.org/10.1179/000870493786962263)

# Notes
This implementation doesn't use the Convex Hull speedup ([Hershberger & Snoeyink](http://dl.acm.org/citation.cfm?id=902273), 1992). Its worst-case complexity is thus O(n<sup>2</sup>)
