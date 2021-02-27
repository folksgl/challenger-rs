![](.github/IMG/challenger.svg)

[![Tests](https://github.com/folksgl/challenger-rs/actions/workflows/test.yaml/badge.svg)](https://github.com/folksgl/challenger-rs/actions/workflows/test.yaml)
![contributions welcome](https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=flat)

Challenger is my own custom chess engine. Challenger-rs is a re-designed version
of the [original challenger project](https://github.com/folksgl/challenger).

## Project Goals
  - To "challenge" the Stockfish chess engine, which is currently the best
    chess engine in the world.
  - Learn the Rust Programming Language

## Status
Building legal, uci-compliant chess engine.

Next: Improve inital position evaluation function for mini-max search.

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
Please read the [License](#LICENSE) for this project.
