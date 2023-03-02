#!/usr/bin/env python3

import freeasterix

freeasterix.load_category_xml(
    '/opt/freeasterix/specs-xml/asterix_cat062_1_18.xml')

payload = {
    "CAT": 62,
    "010": {
        "SAC": 255,
        "SIC": 255
    },
    "040": {
        "TrkN": 0
    },
    "080": {
        "AAC": 0,
        "ADS": 0,
        "AFF": 0,
        "AMA": 0,
        "CNF": 0,
        "CST": 0,
        "EMS": 0,
        "FPC": 0,
        "FPLT": 0,
        "FX": 1,
        "FX2": 1,
        "FX3": 1,
        "FX4": 1,
        "FX5": 0,
        "KOS": 0,
        "MD4": 0,
        "MD5": 0,
        "MDS": 0,
        "ME": 0,
        "MI": 0,
        "MON": 1,
        "MRH": 0,
        "PFT": 0,
        "PSR": 0,
        "SDS": 2,
        "SIM": 0,
        "SPI": 0,
        "SRC": 0,
        "SSR": 0,
        "STP": 0,
        "SUC": 0,
        "TSB": 0,
        "TSE": 0
    }
}

result = freeasterix.encode(payload)
import asterix
print(asterix.parse(result))
