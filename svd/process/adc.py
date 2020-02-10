#!/usr/bin/env python3

"""ADC processing

Just rename the 'result' registers from R to RESULT
"""

import xml.etree.ElementTree as ET


def adc(tree: ET.Element):
    adc = tree.find("./peripherals/peripheral[name='ADC1']/registers")
    for register in adc.findall("./register"):
        name = register.find("./name")
        if name.text == "R0":
            name.text = "RESULT0"
        elif name.text == "R%s":
            name.text = "RESULT%s"
