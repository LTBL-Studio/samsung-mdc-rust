# Samsung MDC client

> (Partial) implementation of Samsung MDC protocol in Rust

This crate provides basic communication to Samsung screens supporting MDC protocol.

## Features

* Set panel on and off
* Set power on and off

## Quick start

```rust
// Default port is 1515
let display_addr = "10.0.151.55:1515".parse().unwrap();

// Establish a TCP connection with your screen
let mut session = MDCSession::new_from_tcp(display_addr)
    .expect("Failed to connect to device");

// Power off screen 0 (first screen in serial chain)
session.display(0)
        .set_power_off()
        .expect("Failed to set power off");
```

Check example folder for more.

## Contributing

Currently, available commands are really small.
We're open to contributions to improve and support more features.

## Resources

* [MDC Protocol specifications](https://vgavro.github.io/samsung-mdc/MDC-Protocol.pdf)
* [MDC Protocol summary](https://gist.github.com/paltaio-admin/0c6ca6c2a5210684fb6a81cbc913feeb)