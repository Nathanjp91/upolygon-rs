import upolygon_rs as up
import pytest
import unittest
import numpy as np

def test_rle():
    data = np.array([0, 0, 0, 1, 1, 1, 1, 0, 0, 0], dtype=np.uint64)
    output = up.rle_encode(data)
    assert np.array_equal(output, [3, 0, 4, 1, 3, 0])
    

def test_rle_multi():
    data = np.array([[0, 0, 0, 1, 1, 1, 1, 0, 0, 0],[0, 0, 0, 1, 1, 1, 1, 0, 0, 0]], dtype=np.uint64)
    output = up.rle_encode(data)
    assert np.array_equal(output, [3, 0, 4, 1, 6, 0, 4, 1, 3, 0])
    
    
def test_rled():
    data = np.array([3, 0, 4, 1, 3, 0], dtype=np.uint64)
    output = up.rle_decode(data)
    assert np.array_equal(output, [0, 0, 0, 1, 1, 1, 1, 0, 0, 0])
    
def test_rled_2d():
    data = np.array([10, 0], dtype=np.uint64)
    output = up.rle_decode(data, 2, 5)
    assert output.shape == (2, 5)
    assert all(output[0] == 0)