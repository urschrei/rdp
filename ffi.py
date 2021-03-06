#!/usr/bin/env python
# -*- coding: utf-8 -*-
"""
ffi.py

Created by Stephan Hügel on 2016-08-3

This file is part of rdp.

The MIT License (MIT)

Copyright (c) 2016 Stephan Hügel

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.

"""

import os
from sys import platform
from ctypes import Structure, POINTER, c_void_p, c_size_t, c_double, cast, cdll
import numpy as np

file_path = os.path.dirname(__file__)
prefix = {'win32': ''}.get(platform, 'lib')
extension = {'darwin': '.dylib', 'win32': '.dll'}.get(platform, '.so')

lib = cdll.LoadLibrary(os.path.join(file_path, "target/release", prefix + "rdp" + extension))

class _FFIArray(Structure):
    """
    Convert sequence of float lists to a C-compatible void array
    example: [[1.0, 2.0], [3.0, 4.0]]

    """
    _fields_ = [("data", c_void_p),
                ("len", c_size_t)]

    @classmethod
    def from_param(cls, seq):
        """  Allow implicit conversions """
        return seq if isinstance(seq, cls) else cls(seq)

    def __init__(self, seq, data_type = c_double):
        self.data = cast(
            np.array(seq, dtype=np.float64).ctypes.data_as(POINTER(data_type)),
            c_void_p
        )
        self.len = len(seq)


class _CoordResult(Structure):
    """ Container for returned FFI coordinate data """
    _fields_ = [("coords", _FFIArray)]

def _void_array_to_nested_list(res, _func, _args):
    """ Dereference the FFI result to a list of coordinates """
    try:
        shape = res.coords.len, 2
        ptr = cast(res.coords.data, POINTER(c_double))
        array = np.ctypeslib.as_array(ptr, shape)
        return array.tolist()
    finally:
        drop_array(res.coords)

simplify_coords = lib.simplify_rdp_ffi
simplify_coords.argtypes = (_FFIArray, c_double)
simplify_coords.restype = _CoordResult
simplify_coords.errcheck = _void_array_to_nested_list

simplify_coords_vw = lib.simplify_visvalingam_ffi
simplify_coords_vw.argtypes = (_FFIArray, c_double)
simplify_coords_vw.restype = _CoordResult
simplify_coords_vw.errcheck = _void_array_to_nested_list

simplify_coords_vwp = lib.simplify_visvalingamp_ffi
simplify_coords_vwp.argtypes = (_FFIArray, c_double)
simplify_coords_vwp.restype = _CoordResult
simplify_coords_vwp.errcheck = _void_array_to_nested_list

drop_array = lib.drop_float_array
drop_array.argtypes = (_FFIArray,)
drop_array.restype = None

if __name__ == "__main__":
    print(simplify_coords(
        [[0.0, 0.0], [5.0, 4.0], [11.0, 5.5], [17.3, 3.2], [27.8, 0.1]],
        1.0
    ))
