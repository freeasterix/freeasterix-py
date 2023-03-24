from .freeasterix import *

class AxConverter():
    def __init__(self, directory):
        self._converter = create_converter(directory)
    def encode(self, data):
        return encode(self._converter, data)
    def decode(self, payload):
        return decode(self._converter, payload)