# freeasterix-py
Python bindings for freeasterix

## Example

```python
from freeasterix import AxConverter
from binascii import hexlify

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