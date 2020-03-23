# mget - HTTP GET from scratch (almost)

## Quickstart


1. Setup TAP device.

```sh
$ ./tap.sh setup
Created TAP device:
  Name: tap-rust
  IP: 192.168.69.100/24
Forwarding packets from 192.168.69.0/24
net.ipv4.ip_forward = 1
```

2. Make a request, specifying the TAP device to use.

```sh
cargo run -- http://httpbin.org/json tap-rust`
```

3. Remove TAP device once finished.

```sh
$ ./tap.sh teardown
Removed TAP device:
  Name: tap-rust
```

Note: pass `-h`/`--help` to either program to see full help.
