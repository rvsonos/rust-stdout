# rust-stdout - Example of piping stdout and stderr to rust
An example of how to take a process' stdout and stderr and
write it to a file in real time.

## Prerequisites
- Python3 in your path

## How to
- Open a terminal and `git clone` this repo.
```
cd rust-stdout
```

- Open another terminal and
```
cd rust-stdout
cargo run
```

- In the first terminal,
```
tail -f *.log
```

Now, you should see the stdout and stderr being piped in real time.

## Credits
- Thanks to @mathieupoumeyrolsonos for his fixes
