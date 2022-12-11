# aoc2022

My advent of code solutions for 2022 in Rust :)

Note: these solutions are **not** optimized and you will notice quite a few
`.unwrap()`s, as it is assumed that the input is valid and properly formed.

## Setup

In order to download the inputs, you will need to collect and save your session
cookie from [adventofcode.com](https://adventofcode.com). After signing in, use
the developer tools in your browser to find the cookie named "session" and store
the value to a file in the root of this repository called `session.txt`.

To setup a day that has not yet been initialized, you may run the command:

```sh
make setup-day01
```

replacing the suffix `"01"` with the appropriate day.


## Running

To run the code for all available days, use the command:

```sh
make
```

To actually run the code for a specific day, you may run the command:

```sh
make run-day01
```

replacing the suffix `"01"` with the appropriate day.

### Docker

To run each solution with Docker, you may run the command:

```sh
make docker-run-day01
```

again replacing the suffix `"01"` with the appropriate day.
