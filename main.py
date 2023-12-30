import ctypes
import json
from time import time


rust = ctypes.CDLL("target/release/librust_lib.so")


if __name__ == "__main__":
    with open("demo.json", "r") as filey:
        schema_str = filey.read()

    start = time()
    for x in range(1):
        json_data = json.dumps({
            "productId": 1,
            "productName": "cat food"
        })

        is_valid = rust.rusty_validate_json(schema_str.encode("UTF-8"), json_data.encode("UTF-8"))

    print(f"Duration: {time() - start}s")
