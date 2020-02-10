#!/usr/bin/env python3

"""PWM modifications

Turn the PWM submodules into a cluster
"""

import xml.etree.ElementTree as ET


def _submodule() -> ET.Element:
    name = ET.Element("name")
    name.text = "SM[%s]"

    description = ET.Element("description")
    description.text = "PWM Submodule"

    dim = ET.Element("dim")
    dim.text = "4"

    dimIncrement = ET.Element("dimIncrement")
    dimIncrement.text = "0x60"

    addressOffset = ET.Element("addressOffset")
    addressOffset.text = "0"

    cluster = ET.Element("cluster")
    cluster.extend([name, description, dim, dimIncrement, addressOffset])
    return cluster


def _replace_sm0_throughout(register: ET.Element):
    """Modifies the fields and enumerated values in the register,
    removing the 'SW0', and preprending 'SW'.
    """
    fields = register.find("./fields")
    for field in fields.findall("./field"):
        name = field.find("./name")
        name.text = name.text.replace("SM0", "SM")
        enumeratedValues = field.find("./enumeratedValues")
        for enumeratedValue in enumeratedValues.findall("./enumeratedValue"):
            name = enumeratedValue.find("./name")
            name.text = name.text.replace("SM0", "SM")


def pwm(tree: ET.Element):
    pwm = tree.find("./peripherals/peripheral[name='PWM1']/registers")

    submodule = _submodule()
    for register in pwm.findall("./register"):
        name = register.find("./name")

        if name.text.startswith("SM"):
            pwm.remove(register)

        if name.text.startswith("SM0"):
            # Use the SM0 registers as the registers to
            # define the cluster, since they'll have the
            # correct address offset by default.
            name.text = name.text.replace("SM0", "SM")
            submodule.append(register)

        if name.text in {"SWCOUT", "DTSRCSEL"}:
            _replace_sm0_throughout(register)

    pwm.append(submodule)
