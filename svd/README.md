## MIMXRT1062 SVD

This is the original SVD file for the MIMXRT1062, available
in the CMSIS Pack [here](https://developer.arm.com/embedded/cmsis/cmsis-packs/devices/NXP/MIMXRT1062XXXXA).

Before using this SVD to generate a PAC, we apply a few custom modifications. The modifications are implemented in the Python `process` module. The modifications are intended to simplify the API for programming in Rust, or for achieving compilation speed-ups. Run `process.py` to apply the modifications to a SVD file.

- We implemented a `<cluster>` to represent each PWM submodule's register. Before that change, there were four independent submodules for each PWM module. Each submodule had its own name. When represented with a `<cluster>`, the auto-generated Rust code is nicer to work with.
- The ADC data result registers were originally named `R[0]`, which conflicted with the `svd2rust` register reader type, `R`. We renamed these registers to `RESULT[0]`.
- We use a multi-dimension register to represent the DMA channels. We also de-duplicate the DMA TCDs using a cluster. This particular change reduced the amount of auto-generated code from around 108K lines to 21K lines of Rust code. The trade-off is that we lose the reset values for the DMA channel priority registers, which are typically the value of the channel number.

The original SVD was formatted using the provided `format.sh` script.

## Generate the PAC supercrate

Run the script `superpac.sh` to generate the super PAC crate. The modules of the super PAC crate may then be imported to our `imxrt1062-pac` crate as independent crates. See the README.md in the `imxrt1062-pac` directory for more information on the import process. Ensure that both `svd2rust` and `form` are installed:

```
cargo install svd2rust
cargo install form
```

The super PAC is excluded from this project's version control system.