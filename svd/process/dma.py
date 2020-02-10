#!/usr/bin/env python3

"""DMA register manipulations
"""

import xml.etree.ElementTree as ET


def _tcd() -> ET.Element:
    """Returns a TCD element"""
    name = ET.Element("name")
    name.text = "TCD%s"

    description = ET.Element("description")
    description.text = "Transfer Control Descriptor"

    dim = ET.Element("dim")
    dim.text = "32"

    dimIncrement = ET.Element("dimIncrement")
    dimIncrement.text = "32"

    addressOffset = ET.Element("addressOffset")
    addressOffset.text = "0x1000"

    cluster = ET.Element("cluster")
    cluster.extend([name, description, dim, dimIncrement, addressOffset])
    return cluster


def dma(tree: ET.Element):
    """Perform transformations on the DMA peripheral. Specifically,

    - De-duplicate the channel priority registers. Replace them with a multi-
      dimension register. Strip the reset values.
    - Represent all TCDs as a cluster.
    """
    dma = tree.find("./peripherals/peripheral[name='DMA0']/registers")
    dchpri = dma.find("./register[name='DCHPRI3']")

    # Add the register dimensions
    #
    # Replace DCHPRI3, because it's the first of these registers.
    dchpri.find("./name").text = "DCHPRI%s"
    dimIndex = ET.Element("dimIndex")
    dimIndex.text = "3,2,1,0,7,6,5,4,11,10,9,8,15,14,13,12,19,18,17,16,23,22,21,20,27,26,25,24,31,30,29,28"
    dimIncrement = ET.Element("dimIncrement")
    dimIncrement.text = "1"

    dim = ET.Element("dim")
    dim.text = "32"

    dchpri.insert(2, dimIndex)
    dchpri.insert(2, dimIncrement)
    dchpri.insert(2, dim)

    # Remove all reset values
    dchpri.remove(dchpri.find("./resetValue"))
    dchpri.remove(dchpri.find("./resetMask"))

    for register in dma.findall("./register"):
        name = register.find("./name").text
        if name.startswith("DCHPRI") and name != "DCHPRI%s":
            dma.remove(register)

    # Build up the transfer control descriptor
    tcd = _tcd()

    for register in dma.findall("./register"):
        name = register.find("./name")
        # Remove all the TCDs, even those that are TCD0.
        # We'll modify and re-add those elements under the
        # TCD cluster
        if name.text.startswith("TCD"):
            dma.remove(register)
        if name.text.startswith("TCD0_"):
            name.text = name.text[5:]
            offset = register.find("addressOffset")
            offset.text = hex(int(offset.text, 0) - 0x1000)
            tcd.append(register)

    dma.append(tcd)
