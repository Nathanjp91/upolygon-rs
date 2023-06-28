import upolygon_rs as up
import numpy as np
from dataclasses import dataclass

@dataclass
class Point:
    x: int
    y: int
empty_mask = np.zeros((4,4), dtype=np.uint64)
path = [Point(x=0, y=0), Point(x=3, y=0), Point(x=0, y=3), Point(x=0, y=0)]
path2 = [Point(x=2, y=2), Point(x=2, y=3), Point(x=3, y=3), Point(x=3, y=2)]
paths = [path, path2]
mask = up.draw_polygons(empty_mask, paths)
print(mask)


image = np.zeros((4,4,3), dtype=np.uint8)
mask = np.array([[1, 1, 1, 1],[1, 0, 0, 1],[1, 0, 0, 1],[1, 1, 1, 1]], dtype=np.uint8)
print(mask)
a, b, c = up.find_contours(mask)