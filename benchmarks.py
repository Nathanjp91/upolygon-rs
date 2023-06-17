import upolygon as upol
import upolygon_rs as upol_rs
import numpy as np
from timeit import timeit
import seaborn as sns
import pandas as pd


sizes = [x for x in range(100, 2000, 100)]
results_rs = [[], []]
results_py = [[], []]
iterations = 1
for i in sizes:
    data_encode = np.random.randint(2, size=i*i).reshape((i, i)).astype(np.uint64)
    encoded = upol_rs.rle_encode(data_encode)
    encoded_upol = encoded.tolist()
    out = timeit("upol_rs.rle_encode(data_encode)", globals=globals(), number=iterations)
    results_rs[0].append(out)
    out = timeit("upol_rs.rle_decode(encoded, i, i)", globals=globals(), number=iterations)
    results_rs[1].append(out)
    out = timeit("upol.rle_encode(data_encode)", globals=globals(), number=iterations)
    results_py[0].append(out)
    out = timeit("upol.rle_decode(encoded_upol, (i, i))", globals=globals(), number=iterations)
    results_py[1].append(out)
    


df = pd.DataFrame({"size": sizes, "rust_encode": results_rs[0], "rust_decode": results_rs[1], "cpython_encode": results_py[0], "cpython_decode": results_py[1]})

headers = [x for x in df.columns if x != "size"]
plot = sns.lineplot(data=df[headers])
fig = plot.get_figure()
fig.savefig("benchmark.png")
# out = timeit("upol_rs.rle_encode(data_encode)", globals=globals(), number=50)
# print(f"encode rust: {out}")
# out = timeit("upol_rs.rle_decode(encoded, 500, 500)", globals=globals(), number=50)
# print(f"decode rust: {out}")
# out = timeit("upol.rle_encode(data_encode)", globals=globals(), number=50)
# print(f"encode cpython: {out}")
# out = timeit("upol.rle_decode(encoded.tolist(), (500, 500))", globals=globals(), number=50)
# print(f"decode cpython: {out}")
# upol.rle_decode(data)
# print("Benchmarking upolygon_rs")