#!/usr/bin/env python3

"""Library where we define MIMXRT1062 SVD transformations

All transformations were designed against version 1.0 of the
MIMXRT1062DVL6A SVD from NXP.
"""

from process.adc import adc
from process.dma import dma
from process.pwm import pwm