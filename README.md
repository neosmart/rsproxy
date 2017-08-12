# rsproxy
_a simple, cross-platform, multi-client TCP/UDP proxy_

`rsproxy` is a cross-platform, multi-client TCP/UDP proxy written in rust, that is designed for those "one-time" tasks where you usually end up spending more time installing a proxy server and setting up the myriad configuration files and options than you do actually using it.

## Usage

`rsproxy` is a command-line application. One instance of `rsproxy` should be started for each remote endpoint you wish to proxy data to/from. All configuration is done via command-line arguments, in keeping with the spirit of this project.

`rsproxy` is written as a wrapper around the standalone [`tcpproxy`](https://github.com/neosmart/tcpproxy) and [`udpproxy`](https://github.com/neosmart/udpproxy) proxy servers, and shells out to one or the other depending on the chosen protocol. If `tcpproxy` or `udpproxy` is not installed, `rsproxy` will attempt to install them automatically (via the `cargo` package manager).

```
rsproxy [-b BIND_ADDR] -l LOCAL_PORT -h REMOTE_ADDR -r REMOTE_PORT [[-t]|-u]

Options:
    -l, --local-port LOCAL_PORT
                        The local port to which proxy should bind to
    -r, --remote-port REMOTE_PORT
                        The remote port to which UDP packets should be
                        forwarded
    -h, --host REMOTE_ADDR
                        The remote address to which packets will be forwarded
    -b, --bind BIND_ADDR
                        The address on which to listen for incoming requests
    -d, --debug         Enable debug mode
    -t, --tcp           Run in TCP mode. Cannot be used together with --udp
    -u, --udp           Run in UDP mode. Cannot be used together with --tcp
```

Where possible, sane defaults for arguments are provided automatically. `rsproxy` defaults to TCP mode if neither `--tcp` nor `--udp` is specified.

## Installation

`rsproxy` is available via `crate`, the rust package manager. Installation is as follows:

    cargo install rsproxy

Pre-complied binaries for select platforms may be available from the `rsproxy` homepage at https://neosmart.net/rsproxy/

## License

`rsproxy` is open source and licensed under the terms of the MIT public license.
