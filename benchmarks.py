import upolygon as upol
import upolygon_rs as upol_rs
import numpy as np
from timeit import timeit



data_encode = np.random.randint(2, size=1920*1080).reshape((1920, 1080)).astype(np.uint64)
data_decode = np.zeros((1920, 1080), dtype=np.uint64)
encoded = upol_rs.rle_encode(data_encode)
out = timeit("upol_rs.rle_encode(data_encode)", globals=globals(), number=50)
print(f"encode rust: {out}")
out = timeit("upol_rs.rle_decode(encoded, 1920, 1080)", globals=globals(), number=50)
print(f"decode rust: {out}")
out = timeit("upol.rle_encode(data_encode)", globals=globals(), number=50)
print(f"encode cpython: {out}")
out = timeit("upol.rle_decode(encoded, (1920, 1080))", globals=globals(), number=50)
print(f"decode cpython: {out}")
# upol.rle_decode(data)
# print("Benchmarking upolygon_rs")