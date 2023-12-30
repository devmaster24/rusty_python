# Rust and Python

## Overview

I did a POC to benchmark the json schema validation time of python's `jsonschema` package and compared this against rust's `jsonschema` package.

I used the from Draft202012Validator in both cases.

I decided to use the FFI manual approach of Python calling a dynamically linked rust library. Thought this would be the most educational and low level, however for production use cases I would probably use pyo3 or rust-cpython as these packages abstact much of this complexity.

## Findings

For validating one schema, the rust library approach is much faster in total execution time. However this is deceiving & not the full story.

Upon further investigation I noticed that the reason python took much longer was importing the `jsonschema`'s `Draft202012Validator` class took the majority of the time. The actual JSON schema validation was very fast and this was proven by benchmarking the code block which performed the validation.

This makes logical sense & was confirmed as python outperformed my rust implementation each time when running multiple validations.

I do not think what I did was a fair comparision between the two languages - but instead was an educational exercise on invoking a c-compatable library built in rust from Python.

## Note

I did all this work in a linux environment, the python code will need to be tweaked if running on windows. Windows handles loading DLLs differently than Linux in python.

# Resource
- https://docs.rs/jsonschema/latest/jsonschema/index.html
- https://github.com/saidvandeklundert/rust_from_python/blob/main/rust_lib2/src/lib.rs
- https://docs.python.org/3/library/ctypes.html#pointers
- https://doc.rust-lang.org/std/ffi/index.html#types