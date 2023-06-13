import upolygon_rs as up
import pytest
import unittest
import numpy as np

def test_rle():
    data = np.array([0, 0, 0, 1, 1, 1, 1, 0, 0, 0])
    output = up.rle_encode(data)
    assert output == [3, 0, 4, 1, 3, 0]
    

def test_rle_multi():
    data = np.array([[0, 0, 0, 1, 1, 1, 1, 0, 0, 0],[0, 0, 0, 1, 1, 1, 1, 0, 0, 0]])
    output = up.rle_encode(data)
    assert output == [3, 0, 4, 1, 6, 0, 4, 1, 3, 0]