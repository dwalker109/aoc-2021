# Advent of Code 2021

This year I am, once again, completing AoC in Rust. I'm enjoying learning all
about the language, and I enjoy writing Rust far more than any other language
right now. An added benefit is the incredible speeds possible, so I am also
aiming to solve all of the year's problems in under a half a second.

## Timings

In general, each part is run in sequence - running them in seperate threads
rarely offers any speed boost anyway. A few solutions do all of the work in
one pass, so I have included timing info for that too.

Timings are approximate (they aren't coming from a benchmark tool). They are
running on an Intel i3-8109U (4) @ 3.600GHz with 8GB of RAM and some
basic SSD. OS is Debian Buster.

| Day | Title | Part 1 | Part 2 |
| --- | ----- | ------ | ------ |
| 1 | Sonar Sweep | 39.904µs | 45.991µs |
| 2 | Dive! | 90.951µs | 88.139µs |
| 3 | Binary Diagnostic | 363.728µs | 353.476µs |
| 4 | Giant Squid | 76ns (1.479774ms) | 40ns (1.479774ms) |
| 5 | Hydrothermal Venture | 7.820412ms | 14.522496ms |
| 6 | Lanternfish | 5.55µs | 4.72µs |
| 7 | The Treachery of Whales | 387.301µs | 3.003278ms |
