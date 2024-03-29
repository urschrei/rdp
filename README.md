[![Test and Build](https://github.com/urschrei/rdp/actions/workflows/test.yml/badge.svg)](https://github.com/urschrei/rdp/actions/workflows/test.yml) [![Coverage Status](https://coveralls.io/repos/github/urschrei/rdp/badge.svg?branch=master)](https://coveralls.io/github/urschrei/rdp?branch=master) [![](https://img.shields.io/crates/v/rdp.svg)](https://crates.io/crates/rdp)

# RDP
A Rust implementation of the [Ramer–Douglas-Peucker](https://en.wikipedia.org/wiki/Ramer–Douglas–Peucker_algorithm) and [Visvalingam-Whyatt](https://bost.ocks.org/mike/simplify/) line simplification algorithms.

**The algorithms underlying this crate have now migrated to [rust-geo](https://github.com/georust/rust-geo) as the [`Simplify`](https://docs.rs/geo/*/geo/algorithm/simplify/index.html) and [`SimplifyVW`](https://docs.rs/geo/*/geo/algorithm/simplifyvw/index.html) traits.**

# FFI
The shared library exposes a(n) FFI: https://docs.rs/rdp/latest/rdp/#functions.  
Some examples are available in [this Jupyter notebook](examples.ipynb).  
[**Simplification**](https://pypi.python.org/pypi/simplification/), a Python package which uses this shared library, is available from PyPi.

### Example Implementation
A Python 2.7 / 3.5 / 3.6 implementation can be found at [`ffi.py`](ffi.py
)  
Run `cargo build --release`, then `python ffi.py` to test. It's also importable, exposing `simplify_linestring()` – call it with a coordinate list and a precision parameter. Allocated memory is dropped on exit.  

# Performance & Complexity
On an 841-point LineString, RDP runs around 3.5x faster than VW. However, RDP's worst-case time complexity is O(*n*<sup>2</sup>) – This implementation doesn't use the Convex Hull Speedup, see [Hershberger & Snoeyink](http://dl.acm.org/citation.cfm?id=902273), 1992 – whereas the VW implementation uses a min-heap, and thus has worst-case time-complexity of O(*n* log(*n*)), which may make it a better choice for larger LineStrings under certain conditions; RDP has an *average* time complexity of O(*n* log(*n*)), but LineStrings such as the one seen [here](http://stackoverflow.com/a/31566048/416626) will slow it down significantly.
You can verify these times for yourself by running `cargo bench`.

# License
[MIT](license.txt)

# References
**Douglas, D.H.**, **Peucker, T.K.**, 1973. *Algorithms for the reduction of the number of points required to represent a digitized line or its caricature*. Cartographica: The International Journal for Geographic Information and Geovisualization 10, 112–122. [DOI](http://dx.doi.org/10.3138/FM57-6770-U75U-7727)

**Ramer, U.**, 1972. *An iterative procedure for the polygonal approximation of plane curves*. Computer Graphics and Image Processing 1, 244–256. [DOI](http://dx.doi.org/10.1016/S0146-664X(72)80017-0)

**Visvalingam, M.**, **Whyatt, J.D.**, 1993. *Line generalisation by repeated elimination of points*. The Cartographic Journal 30, 46–51. [DOI](http://dx.doi.org/10.1179/000870493786962263)
