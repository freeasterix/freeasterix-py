# freeasterix-py
Python bindings for freeasterix

## Quick start

On Debian/Ubuntu systems, execute:

```bash
apt -y install --no-install-recommends python3-pip
```

on other systems refer to system documentation how to setup Python3 and
Python3-pip.

### Install from wheel

Download a provided wheel package, then execute:

```bash
pip3 install ./freeasterix-0.1.0-cp310-cp310-linux_x86_64.whl
```

### Install from sources

Install [setuptools-rust](https://pypi.org/project/setuptools-rust/) and [Rust
compiler](https://www.rust-lang.org/tools/install):

```bash
pip3 install setuptools_rust
apt -y install --no-install-recommends build-essential
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh /dev/stdin -y -c rustc
source "$HOME/.cargo/env"
```

Until the primary repo is opened for public: make sure you have access to
<https://github.com/freeasterix/freeasterix> and have GitHub ssh key set-up:

```
ssh -T git@github.com
```

Build and install the module:

```bash
git clone https://github.com/freeasterix/freeasterix-py
cd freeasterix-py
python3 setup.py install
```

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
