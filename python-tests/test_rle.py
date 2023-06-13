import upolygon_rs as up
import pytest
import unittest
import numpy as np

def test_rle():
    data = np.array([0, 0, 0, 1, 1, 1, 1, 0, 0, 0])
    output = up.rle_encode(data)
    assert output == [0, 3, 1, 4, 0, 3]
    

def test_rle_multi():
    data = np.array([[0, 0, 0, 1, 1, 1, 1, 0, 0, 0],[0, 0, 0, 1, 1, 1, 1, 0, 0, 0]])
    output = up.rle_encode(data)
    assert output == [0, 3, 1, 4, 0, 3]