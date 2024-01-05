# baz_dueler

This crate manages boom and zoom binaries and facilitates playing them against each other.

# Installation
```sh
 cargo install --git https://github.com/dchiquito/boom-and-zoom baz_dueler --force
```
`--force` is required because I don't care enough to version things. If you're missing any recent features, just rerun that command.

# Usage

## config.yml
Dueler requires a `config.yml` in the directory that it is run in. This file contains all the configuration for the different binaries, and a description of the active tournament players.
```yml
players:
  - name: git-example
    args: ["play", "genius"]
    git:
      repo: https://github.com/dchiquito/boom-and-zoom
      target: main
    build: cargo build --bin baz_cli --release
    artifact: target/release/baz_cli
  - name: random
    args: ["play", "random"]
    workdir: ../cli/
    build: cargo build --release
    artifact: ../target/release/baz_cli
  - name: go-fast
    args: ["play", "go-fast"]
    workdir: ../cli/
    build: cargo build --release
    artifact: ../target/release/baz_cli
  - name: genius
    args: ["play", "genius"]
    workdir: ../cli/
    build: cargo build --release
    artifact: ../target/release/baz_cli
tournament:
  - random
  - go-fast
  - genius
```

### players
Each player must have a unique `name`.

`args` is passed in to the binary.

Each player must either have a `git` or a `workdir` field. `git` means that the given git `repo` will be pulled and the given `target` will be checked out before running the build from the root of the repo. `workdir` will simply navigate to the given directory.

`build` is the build script which generates the binary.

`artifact` is the location of the binary, relative to `workdir` or the root of the git `repo`.

### tournament
The `tournament` field lists the active players that will compete in any tournaments. You can set up as many players as you want, then limit the tournament to the players you are interested in.

## CLI
Dueler is invoked with `baz_dueler`. 

### `baz_dueler update`
This command pulls any git repositories, reruns all the build scripts, and copies all the binaries to `./players/`.

### `baz_dueler play [--update] [--skip-self] GAMES`
This command plays out a tournament.

If `--update` is specified, all participants are updated first to pick up any changes.

If `--skip-self` is specified, no mirror matchups will be played. This is useful if you are trying to determine how well one AI fares against another.

`GAMES` specifies the number of rounds in a match. Every matchup will be played out `GAMES` times.

The output table is formatted with white players on the left and black players on the top. Each cell is formatted as `{white wins}/{black wins}(draws)`.

# Implementing players
Simply use the included `StdioGamePlayer` from the `baz_dueler` crate:
```rust
// main.rs
use baz_core::GamePlayer;
use baz_dueler::StdioGamePlayer;

struct MyPlayer { ... }
impl GamePlayer for MyPlayer { ... }

fn main() -> std::io::Result<()> {
    let player = MyPlayer { ... };
    let mut stdio_player = StdioGamePlayer::new(player);
    stdio_player.main()?;
}
```
`StdioGamePlayer` will handle all the I/O, all you need to do is provide it your `GamePlayer` implementation and invoke it from your main. This will create a Dueler compatible binary.
