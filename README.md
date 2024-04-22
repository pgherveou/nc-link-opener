# nc-link-opener

Simple program to open links from your remote ssh machine, using netcat.

## Install

```shell
# build project
cargo build -r -p nc-link-opener

# copy to bin
cp target/release/nc-link-opener /usr/local/bin/nc-link-opener 

# Setup launch agent
cargo run -p nc-link-opener-launcher -- 8378 
```

## Test

# Usage

Update `~/.ssh/config` with `RemoteForward` to forward port to your machine.

```shell

Host my-machine.local
  # nc-link-opener
  RemoteForward 8378 localhost:8378
```

Configure your remote machine to use a custom open command.

```shell
open() {
    echo "$1" | nc -N localhost 8378
}
```

Test it.

```shell
open https://google.com
```
