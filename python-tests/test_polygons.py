import upolygon_rs as up
import pytest
import unittest
import numpy as np


def test_draw_polygon():
    data = np.array([0, 0, 0, 1, 1, 1, 1, 0, 0, 0], dtype=np.uint64)
    output = up.draw_polygon(data)
    assert np.array_equal(output, [0, 0, 0, 1, 1, 1, 1, 0, 0, 0])