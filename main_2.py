import json
from time import time

start = time()
from jsonschema import Draft202012Validator
print(f"Import time {time() - start}")


if __name__ == "__main__":
    with open("demo.json", "r") as filey:
        schema = json.load(filey)

    start = time()

    for x in range(1):
        validator = Draft202012Validator(schema)

        json_data = {
            "productId": 1,
            "productName": "cat food"
        }

        is_valid = True
        for error in validator.iter_errors(json_data):
            is_valid = False
            print(error.message)

    print(f"Duration {time() - start:f}")