# MCHPRS_IO

A derivative of [MCHPRS](https://github.com/MCHPR/MCHPRS) — a multithreaded Minecraft 1.20.4 creative server built for computational redstone — with a plain-text TCP automation protocol and a ready-to-run demo.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Derivative Notice

This project is a **derivative (fork) of [MCHPRS](https://github.com/MCHPR/MCHPRS)**.

- Upstream repository: <https://github.com/MCHPR/MCHPRS>
- Upstream license: **MIT** — Copyright (c) 2020 Ojas Landge
- This derivative is also licensed under the **MIT License** (see [LICENSE](./LICENSE)).

## Overview

MCHPRS_IO keeps all of MCHPRS's core strengths for computational redstone:

- Plots run on their own threads — less lag, more concurrency. This fork runs a
  single 512x512 plot, `p0,0`; see "Runtime Model" below.
- [Redpiler](docs/Redpiler.md), the "Redstone Compiler", compiles redstone circuits for extremely fast simulation.
- A plain-text **automation protocol** over TCP (default port 25585) that lets external programs directly read and drive redstone machines.

The `MCHPRS_IO/` folder in this repository contains a complete, ready-to-run demo of the automation protocol, so you can see results in minutes.

## Features

- Multithreaded redstone server for Minecraft 1.20.4 creative mode
- Redpiler redstone compiler
- Automation protocol (TCP port 25585, plain text, protocol version 0.4.3-beta):
  - `PIN` / `POUT` data ports expose redstone regions as hex values; the
    hex is the whole bit vector as one integer, zero-padded at the front
    (changed in 0.4.0-beta)
  - `INPUT` / `OUTPUT` read and write port values
  - `EVENT` pushes notify subscribers of changes
  - Port names are case-sensitive and pushed exactly as registered
  - `/` command lines let scripts run supported player commands (for example
    `/redpiler compile`) on the server's single plot, without a player
- **Headless operation**: circuits keep running with no player connected, so
  scripts can drive them unattended
- **Single plot, skyblock-style**: only `p0,0` exists; no other plot is loaded
- **Controlled crash handling**: a compile crash saves the plot, logs `[CRASH]`
  and exits with code 1
- **Empty containers no longer abort a compile**: they compile as 0 and
  every affected position is reported in one red warning
- **File-backed memories**: read-only ROMs from `memory/*.rom.txt` and
  writable RAMs from `memory/*.ram.txt` (Logisim `v3.0 hex words addressed`),
  driven by redstone address, clock and enable lines
- WorldEdit-style building commands, Velocity ip-forwarding, LuckPerms support

## Runtime Model (0.4.0-beta)

### A single plot

The server keeps only the spawn plot `p0,0` (512x512). No other plot is ever
loaded and players cannot enter one. Walking outside `p0,0` does not pull you
back: the surrounding space is empty (a skyblock-style island), so you can drift
out and still see the island behind you. A player whose saved position lies
outside the island is teleported to the plot center on join or reconnect.

### Runs without players

Redstone simulation, redpiler flushes and every automation sample no longer
depend on a connected client. With nobody online, `p0,0` keeps ticking, `EVENT` /
`OUTPUT` lines keep being pushed and `PIN` / `INPUT` / `SUB` keep being answered.
Together with the `/` command below, scripts can drive circuits completely
unattended.

### Scripts can run commands

A whole line starting with `/` sent over the automation connection is executed
as if a player had typed it. An automation caller has no player entity, so it
acts with **operator permissions**, commands always apply to **`p0,0`**, and
relative coordinates (`~` / `^`) are rejected. Supported commands:

| Command | Meaning |
| --- | --- |
| `/redpiler compile [args]` | Trigger a compile — the key to starting a circuit with nobody online |
| `/redpiler reset` | Reset the circuit |
| `/radvance <ticks>` | Advance the plot by N ticks |
| `/toggleautorp` | Toggle automatic compilation |
| `/worldsendrate <hz>` | Set the world send rate |
| `/memory reload` | Re-read `memory/*.rom.txt` from disk |
| `/stop` | Shut the server down |

Success replies `OK CMD`; failure replies `ERR CMD <INACTIVE|UNKNOWN|ARG>`. The
command's own chat output is **not echoed back**. Everything else (`/plot`,
`/gamemode`, `//load`, ...) returns `ERR CMD UNKNOWN`.

### Compile crashes

If an unrecoverable error occurs while redpiler is compiling, the server first
**saves that plot's data**, then logs a single `[CRASH]` line (timestamp, plot
coordinates, error message, save path, exit code):

```text
[CRASH] plot (0, 0) redpiler panic: <message> — saved to ./world/plots/p0,0 — exiting with code 1
```

and terminates with exit code **1**.

### Empty containers

A comparator reading an empty container (empty cauldron / barrel / furnace /
hopper / composter) **no longer aborts the compile**: that block is compiled as 0
and the player who started the compile receives **one red warning** listing every
affected position, for example:

```text
Redpiler compile warning: comparator at (143,89,367),(126,69,127) reads some empty containers
```

Only the first ten positions are listed, followed by `,(...)`. Blocks the server
has no rule for (`Unsupported`) still abort the compile.

### Memory files (ROM and RAM)

A memory is defined by a text file in `memory/`, next to `Config.toml`.
`*.rom.txt` files are read-only, `*.ram.txt` files are writable. Every file in
that directory is loaded at startup.

```text
v0.4.3-beta hex words addressed
origin = 0
clk = 0
address coordinate = [
width = 2
-8, 64, 0
-7, 64, 0
]
data coordinate = [
width = 2
2, 64, 0
3, 64, 0
]
00: 0f 3c 69 96
```

- The first line must name the running kernel version, otherwise the file is
  rejected.
- `clk = 0` means **no clock**: the data bus is rewritten whenever the address
  value changes. `clk = x, y, z` reads once per rising edge of that element.
- `origin = 0` (or `0, 0, 0`) keeps coordinates absolute; any other triple is
  added to every coordinate, `clk` included. Keys may appear in any order.
- The **first entry of every list must be `width = N`**, declaring how many
  coordinates that list expands to; a mismatch rejects the whole file. An empty
  list `[ ]` is an error as well.
- `address coordinate` entries must be redstone lamps, trapdoors or note
  blocks; `data coordinate` entries must be levers or pressure plates. The
  first entry of each list is the **most significant** bit; bus width is not
  limited to a multiple of four.
- A line is either a single coordinate `x, y, z` or a **step expansion**
  `x, y, z, +2x, 4`: the sign is mandatory, the axis must be x, y or z (either
  case), the step magnitude is 1-255 and the count is 1-1024 written as plain
  digits.
- `matrix: x, y, z, +1x, 3, +1y, 5` expands up to three directions at once
  (row major, first direction fastest), and such a list must occupy exactly one
  line. It aligns the hex words by padding every row at its front with
  `(4 - first % 4) % 4` zeros, so one word needs
  `ceil(first/4) * product(remaining counts)` hex digits.
- Any two bits of the same memory that resolve to the same coordinate are an
  error, reported with line, expansion index, coordinate and the clashing
  line.
- The payload is the Logisim `v3.0 hex words addressed` format: `addr: word
  word ...`. Words are separated by spaces or newlines, `#` / `//` / `/* */`
  comments are accepted everywhere, unlisted addresses read as 0, and a word
  wider than the data bus keeps its low bits.
- A memory is validated after every successful compile. A memory whose
  coordinates do not resolve is disabled with a red chat message; the others
  keep running. Reads and writes happen right after a tick, so a memory has a
  fixed **one tick latency**.
- A data lever that shares a face with an address or clock element is rejected:
  it would drive the very signal the memory reads (a one tick feedback loop).
  Indirect coupling through wires is not detected.
- Two memories may share address and data lines; a shared **clock** element
  disables both.
- `/memory reload` re-reads `memory/` without restarting the server. It can be
  typed by a player or sent by a script as a `/` line.

### Writable memories (`*.ram.txt`)

A RAM keeps its contents in the server and writes them back to the same file.

```text
v0.4.3-beta hex words addressed
origin = 0
clk = 12, 37, 76                  // mandatory; drives both read and write
write enable = 43, 46, 25
read enable = 81, 66, 48
byte write enable = [             // use width = 0 when there is none
width = 1
26, 67, 94
]
address coordinate = [
width = 2
258, 10, 263
258, 8, 263
]
write data coordinate = [         // driven by the circuit (lamps)
width = 2
256, 14, 264
256, 12, 264
]
read data coordinate = [          // driven by the server (levers)
width = 2
256, 34, 264
256, 32, 264
]
0: 3 1 0 0 0 0 0 0
```

- The clock is mandatory and drives both halves. Everything is latched on its
  rising edge, so a read or a write lands one tick later.
- `write data coordinate` and `read data coordinate` must be different blocks
  with the same width.
- Byte write enables are grouped from the most significant end, eight bits at a
  time: `ceil(N/8)` entries, the first entry covers the most significant group
  and the last one always covers the low 8 bits. An enabled group takes the new
  bits, a disabled group keeps the old value.
- Read priority: when read and write enable are both high on the same edge, the
  read data bus carries the **old** word while the write is applied.
- Contents are written back to the same file (header preserved, first line
  updated to the running version). The data section is Logisim compatible:
  each line holds exactly 8 words, its address is aligned to 8 and written
  with as many hex digits as the address bus is wide, and blocks without any
  stored word are omitted (their addresses read back as zero). Nothing is
  written during normal operation: the file is saved when a player leaves the
  server, on shutdown, on `/redpiler reset`, on a compile crash and before
  `/memory reload`.
- A read data lever must not be driven by the circuit: the memory would fight
  it. This is checked when the memory is validated.

## Quick Start

Build the server:

```shell
cargo build --release
```

The optimized executable is located at `./target/release/mchprs`.

Start the server with the demo configuration (binds `127.0.0.1:25565`, automation port `25585`):

```shell
cd MCHPRS_IO
/path/to/target/release/mchprs
```

In a second terminal, run the demo client:

```shell
python3 MCHPRS_IO/test.py
```

The demo registers an 8-bit adder (`PIN add_in` input port, `POUT add_out` output port) over the automation protocol and verifies it against input vectors. You will see output like:

```text
<- HELLO MCHPRS AUTOMATION 0.4.3-beta
[25] passed=25 failed=0 rate=10/s
...
done. total=500 passed=500 failed=0 elapsed=50.1s rate=10/s
```

Set `VERBOSE = True` at the top of `test.py` to see every protocol line, for example:

```text
-> INPUT add_in a0ab
<- OK INPUT add_in
<- OUTPUT add_out df0
```

### Demo folder

| Path | Purpose |
| --- | --- |
| `MCHPRS_IO/Config.toml` | Demo server config (automation port 25585) |
| `MCHPRS_IO/test.py` | Demo client — Python 3 standard library only |
| `MCHPRS_IO/schems/cca_test.schem` | 8-bit adder schematic used by the demo |
| `MCHPRS_IO/world/` | Pre-built demo world |

### Demo test modes

Edit `TEST_MODE` at the top of `test.py`:

- `EXHAUSTIVE` — every input combination (256 x 256 x 2 = 131,072 vectors)
- `RANDOM` — 500 fixed-seed random vectors (default)
- `EDGE` — edge cases (0x00, 0x01, 0x7F, 0x80, 0xFF, ...)

## Automation Protocol

The full protocol specification:

- English: [src/protocol.txt](src/protocol.txt)
- 中文: [src/protocol_zh.txt](src/protocol_zh.txt)

## Configuration

The server generates a `Config.toml` in the working directory on first start.

| Field | Description | Default |
| --- | --- | --- |
| `bind_address` | Bind address and port | `0.0.0.0:25565` |
| `motd` | Message of the day | `"Minecraft High Performance Redstone Server"` |
| `chat_format` | Chat format using `{username}` / `{message}` | `<{username}> {message}` |
| `max_players` | Maximum simultaneous players | `99999` |
| `view_distance` | Max distance (in chunks) of chunks loaded around players | `8` |
| `schemati` | Mimic the ORE Schemati plugin directory layout | `false` |
| `block_in_hitbox` | Allow placing blocks inside players | `true` |
| `auto_redpiler` | Automatic compilation (**only triggers when rtps is `unlimited` or the plot is falling behind**; for unattended use prefer a script sending `/redpiler compile`) | `false` |
| `automation_port` | TCP port for the automation protocol (an invalid value falls back to `25585` with a warning; a failed bind logs a warning and disables automation) | `25585` |
| `automation_data_port_events` | Push `EVENT` lines for data-port changes | `false` |
| `automation_per_tick_push` | Sample data ports once per tick | `false` |

Velocity ip-forwarding and LuckPerms are supported; see the upstream README for setup details.

## Useful Commands

| Command | Alias | Description |
| --- | --- | --- |
| `/rtps [rtps\|unlimited]` | — | Set redstone ticks/s (default 10) |
| `/radvance <ticks>` | `/radv` | Advance the plot by `<ticks>` redstone ticks |
| `/redpiler compile` | `/rp c` | Start redpiler compilation |
| `/redpiler reset` | `/rp r` | Stop redpiler |
| `/toggleautorp` | — | Toggle automatic redpiler compilation |
| `/teleport <x> <y> <z>` | `/tp` | Teleport (supports relative coordinates) |
| `/speed <speed>` | — | Set fly speed |
| `/gamemode <mode>` | `/gmc`, `/gmsp` | Set gamemode |
| `/plot claim` | `/p c` | Claim the plot you are in |
| `/plot auto` | `/p a` | Automatically find and claim a free plot |
| `/memory reload` | — | Re-read the memory definitions from `memory/` |
| `/stop` | — | Stop the server |

`/redpiler compile`, `/redpiler reset`, `/radvance`, `/toggleautorp`,
`/worldsendrate`, `/memory reload` and `/stop` can also be sent by an automation
script as `/` lines, with no player online (see "Runtime Model" above).

> **Note**: this build runs only `p0,0`, so commands that rely on switching
> plots (`/plot auto`, `/plot visit`, `/plot claim`, ...) cannot move a player
> to another plot — those plots are never loaded. `/teleport` still lets you
> move outside the island, but the surrounding space is empty.

The full plot-ownership, WorldEdit and Redpiler command references are available in-game (`//help`, `/redpiler compile --help`) and in the upstream README.

## License

This derivative is licensed under the **MIT License** — the same license as upstream MCHPRS (Copyright (c) 2020 Ojas Landge). See [LICENSE](./LICENSE).

## Acknowledgments

This project is a derivative of [MCHPRS](https://github.com/MCHPR/MCHPRS) by [StackDoubleFlow](https://github.com/StackDoubleFlow) and contributors. Thanks to the upstream authors:

- [@AL1L](https://github.com/AL1L) for contributions to worldedit and other features.
- [@DavidGarland](https://github.com/DavidGarland) for the faster `get_entry` implementation in the in-memory storage.
