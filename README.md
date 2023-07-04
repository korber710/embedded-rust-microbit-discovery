# Embedded Rust micro:bit v2 Discovery

The Embedded Rust Discovery Book Using Micro:bit v2

## Debugging

Open a terminal with OpenOCD.

```bash
openocd -f interface/cmsis-dap.cfg -f target/nrf51.cfg
```

Install the _Cortex-M Debug_ extension in VS Code and then add a new configutation with the following contents:

```json
{
    "name": "Debug (OpenOCD)",
    "type": "cortex-debug",
    "request": "launch",
    "servertype": "external",
    "gdbTarget": "localhost:3333",
    "cwd": "${workspaceFolder}",
    "device": "nRF52833-QIAA",
    "postLaunchCommands": ["monitor arm semihosting enable"],
    "configFiles": [
        "interface/cmsis-dap.cfg",
        "target/nrf51.cfg"
    ],
    "executable": "./target/thumbv7em-none-eabihf/debug/embedded-rust-microbit-discovery",
    "runToEntryPoint": "main"
}
```
