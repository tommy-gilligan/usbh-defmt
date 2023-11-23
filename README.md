# usbh-defmt

A trivial [`usbh`](https://github.com/nilclass/usbh) wrapper that provides tracing via [`defmt`](https://github.com/knurling-rs/defmt)

```rust
let usb_host = UsbHost::new(
    // This layer just adds tracing
    UsbhDefmt::new(
        // UsbHostBus here is the `usbh::bus::HostBus` implementation for the rp2040.
        UsbHostBus::new(
            ctx.device.USBCTRL_REGS,
            ctx.device.USBCTRL_DPRAM,
            clocks.usb_clock,
            &mut ctx.device.RESETS,
        )
    )
);
```

Output looks like this
```
TRACE poll start
└─ usbh_defmt::{impl#1}::poll @ /Users/tom/usbh-defmt/src/lib.rs:97
TRACE poll end Some(InterruptPipe(0))
└─ usbh_defmt::{impl#1}::poll @ /Users/tom/usbh-defmt/src/lib.rs:99
TRACE pipe_continue start 0
└─ usbh_defmt::{impl#1}::pipe_continue @ /Users/tom/usbh-defmt/src/lib.rs:147
TRACE pipe_continue end 0
└─ usbh_defmt::{impl#1}::pipe_continue @ /Users/tom/usbh-defmt/src/lib.rs:149
```
