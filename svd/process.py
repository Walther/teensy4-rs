#!/usr/bin/env python3

"""The script may reproduce our customizations on the MIMXRT1062 SVD.

It was designed against version 1.0 of the MIMXRT1062DVL6A SVD from NXP.
"""

import argparse
import xml.etree.ElementTree as ET

from process import dma

parser = argparse.ArgumentParser()
parser.add_argument("input", help="Path to input SVD")
parser.add_argument("output", help="Path to output SVD")

args = parser.parse_args()

svd = ET.parse(args.input)

dma(svd)

svd.write(args.output)
