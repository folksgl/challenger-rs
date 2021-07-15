![](.github/img/challenger.svg)

[![Tests](https://github.com/folksgl/challenger-rs/actions/workflows/rust-checks.yaml/badge.svg)](https://github.com/folksgl/challenger-rs/actions/workflows/rust-checks.yaml)
![contributions welcome](https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=flat)

Challenger is a [UCI-compliant](http://wbec-ridderkerk.nl/html/UCIProtocol.html)
chess engine. Challenger-rs is a re-designed version of the 
[original challenger project](https://github.com/folksgl/challenger) that uses
Rust and some lessons learned from it's predecessor.

## Project Goals
### Primary
  - To "challenge" the Stockfish chess engine, which is currently the best
    chess engine in the world.
  - Have fun.
### Secondary
  - Improved understanding of internal chess engine design techniques.
  - Learn the Rust Programming Language. (done!)

## Status
Currently the project is focused on implementing the uci commands required to
be a uci-compliant chess engine. This includes writing the move generation
functions, search capabilities, position evaluation, and ensuring challenger
can play fully legal chess.

## How is challenger implemented (FAQ's)

### What protocol does Challenger use to talk to GUI's?
Challenger is a UCI-compliant engine. UCI (The 
[Universal Chess Interface](http://wbec-ridderkerk.nl/html/UCIProtocol.html))
is a text-based protocol that allows a GUI or other interface to send game commands
to the engine. 

### How is the board represented?
Challenger uses [Bitboards](https://www.chessprogramming.org/Bitboards), a
popular piece-centric board representation. Using an unsigned, 64-bit integer,
the least significant bit represts A1, and the most significant represents H8.

### How are moves represented?
Inspired by [from-to based moves](https://www.chessprogramming.org/Encoding_Moves)
as well as Stockfish's move representation, Challenger uses an unsigned 16-bit
integer to encode moves with the lower 12 bits representing the origin and
destination squares, and the upper 4 bits for encoding special move types.

### How are moves generated?
For efficiency purposes, challenger uses [Magic Bitboards](https://www.chessprogramming.org/Magic_Bitboards)
to perform lookups on pre-initialized attack tables for sliding pieces. A similar,
less complex, set of lookup tables are used for Knights and Kings. Pawns are
the only piece type that are generated on the fly due to their relatively simple
move patterns.

## Progress

Current level: 0

Percentage winning next level:  ![progress](https://progress-bar.dev/0)
 
The progress of challenger will be gauged as follows: Challenger will play
Stockfish starting at its lowest level. Once the Challenger engine can beat
the Stockfish engine at the given level at least 60% of the time, the Stockfish
level will be increased, the "Current level completed" label above will be
updated, and a release of the code will be made to snapshot the progress made.
The "Percentage winning next level" label indicates challenger's progress in
beating the next level of Stockfish.

Percentages will be determined by playing 10 games and recording winnings.
While this method is *not* statistically sound, it works well enough for
determining when challenger should increase the difficulty level of the
Stockfish engine again.

## Contributing
Development for challenger will be done in Rust, using the built-in testing
and benchmarking capabilities. Before opening issues or pull requests, please
read the [contributing page](#CONTRIBUTING.md) for Challenger.

Contributions of all kinds are welcomed and encouraged. This project is far
from finished and there are always portions of the code or documentation that
can be improved.

## License
Please read the [License](#COPYING) for this project.
