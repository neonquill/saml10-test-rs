# saml10-test

Basic rust test code for working with the Microchip SAML10 family of
chips.

Includes a device crate for the atsaml10e16a as well as a basic blink
example. These have been tested with the [SAML10 XPLAINED][xplained]
board.

## Programming

### [cargo saml10][cargo_saml10]

```
cargo saml10
```

### [MPLAB IPE]

To generate a hex file for MPLAB IPE:
```
arm-none-eabi-objcopy -O ihex target/thumbv8m.base-none-eabi/release/blink blink.hex
```

Then use the MPLAB IPE app to program the chip using the built in debugger.

### [edbg][edbg]

To generate a binary file for edbg:
```
arm-none-eabi-objcopy -O binary target/thumbv8m.base-none-eabi/release/blink blink.bin
```

To flash the chip:
`edbg --target saml10 --program --verify -f blink.bin`


[xplained]: https://www.microchip.com/en-us/development-tool/DM320204)
[edbg]: https://github.com/ataradov/edbg
[ipe]: https://www.microchip.com/en-us/tools-resources/production/mplab-integrated-programming-environment
[cargo_saml10]: https://github.com/neonquill/cargo-saml10

## License

Choose any of:

 - Apache 2.0: [LICENSE-APACHE](LICENSE-APACHE)
 - MIT: [LICENSE-MIT](LICENSE-MIT)
 - Blue Oak: [LICENSE-BLUE-OAK.md](LICENSE-BLUE-OAK.md)
