# blured_ipc

This repository provides a minimal definition of Blured’s IPC messages.

[Blured](https://github.com/rboeni2s/blured) is a Wayland wallpaper utility.

Blured’s IPC works by sending JSON messages over Unix domain sockets. The format of these messages is defined in `src/msg.rs`. Each exchange between a client and the Blured server follows this structure:

1. The `client` sends a `Message` to the Blured server containing an `Action`.
2. The Blured server receives this `Message` and processes its `Action`.
3. The Blured server sends a `Response` back, indicating the result of that `Action`.

*A `client` in this context is a software tool or utility that sends commands (referred to as actions) to a running Blured instance.*

Clients can use `blured_ipc::instance::Instance` to connect to and communicate with the Blured server, or implement their own Unix domain socket communication (for example, if you want asynchronous UDS instead).
