# TCP-Chat

A simple command-line chat application for communicating within a local network via TCP sockets.

## Getting Started

### Prerequisites:

- [Go](https://go.dev/doc/install)+Rust or just [Docker](https://docs.docker.com/engine/install) installed

### 2. Installation:

**Install via docker:**

- Build docker image

  ```sh
      git clone github.com/anhtr13/tcpchat
      cd tcpchat
      docker build -t tcpchat-client ./tcpchat-client
      docker build -t tcpchat-server ./tcpchat-server
  ```

- Run the tcpchat-server

  ```sh
      docker run -p 8080:8080 tcpchat-server:latest
  ```

- Run some tcpchat-client with interactive shell

  ```sh
      docker run -it -e HOST=[server_ip] PORT=[server_port] tcpchat-client:latest
  ```

## Usage

```sh
> [/cmd] [message]
```

**Commands:**

- `/name`: Change your nick name.
- `/join`: Join a room or create if it doesn't exists.
- `/msg`: Send a message to a room.
- `/rooms`: Get all existing rooms.

**Example:**

```sh
> /name xbro        # Rename to xbro
> /join room1       # Join room1
> /msg Hello there  # Send message to room1
> /join room2       # Join room2
> /msg Hello        # Send message to room2
```
