import numpy as np
import pandas as pd
import pyfathom

data = {
    "name": "Eric",
    "values": list(range(10000)),
}

data_np = np.array(range(10000))

data_pd = pd.DataFrame({"A": range(1000), "B": range(1000, 2000)})

print("shallow:", pyfathom.sizeof(data["values"]))
print("deep   :", pyfathom.deep_sizeof(data))
print("shallow (np):", pyfathom.sizeof(data_np))
print("deep    (np):", pyfathom.deep_sizeof(data_np))
print("shallow (pd):", pyfathom.sizeof(data_pd))
print("deep    (pd):", pyfathom.deep_sizeof(data_pd))
