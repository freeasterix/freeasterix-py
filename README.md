# freeasterix-py
Python bindings for freeasterix

## Build and development

```bash
# follow instructions on https://rustup.rs/ to install Rust toolchain (once)
# install dev dependencies (once)
$ python -m pip install requirements-dev.txt
# install package locally for development (optional)
$ python setup.py develop
# build a "release" version of the package
$ python setup.py build
# make distributable package
$ python setup.py bdist_wheel
```

## Example

```python
from freeasterix import AxConverter
from binascii import hexlify

# specify directory to load XML specifications from
# the directory must not contain other files
converter = AxConverter('../spec')

packet_data = {
    "CAT": 62,
    "records": [
        {
            "010": {"SAC": 13,"SIC": 37}
        },
        {
            "010": {"SAC": 13,"SIC": 37},
            "210": {"Ax": 22.0,"Ay": 6.75},
            "290": {
                "MDS": {"MDS": 63.0},
                "PSR": {"PSR": 63.0},
                "SSR": {"SSR": 63.0}
            }
        }
    ]
}
packet_bytes = converter.encode(packet_data)
print('encoded packet:', hexlify(packet_bytes).decode())
decoded_data = converter.decode(packet_bytes)
print('decoded packet:', decoded_data)
```
