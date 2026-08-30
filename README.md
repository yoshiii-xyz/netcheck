# netcheck

Check TCP/UDP network connectivity and DNS resolution.

## Install

```console
cargo build --release
sudo cp target/release/netcheck /usr/local/bin/
```

## Usage

```console
netcheck google.com 443
netcheck 1.1.1.1 53 --timeout 5
```

Output:

```
OK google.com:443. Connected in 45ms
```
