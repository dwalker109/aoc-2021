# Advent of Code 2021

This year I am, once again, completing AoC in Rust. I'm enjoying learning all
about the language, and I enjoy writing Rust far more than any other language
right now. An added benefit is the incredible speeds possible, so I am also
aiming to solve all of the year's problems in under a half a second.

> It was going so well! And then I found myself brute forcing billions of 
> values on day 24 and that's not going to come in at a few milliseconds. 
> There are ways to solve these puzzles with maths alone, but I don't fancy 
> that.

## Timings

In general, each part is run in sequence, except where noted. A few solutions 
do all of the work in one pass, so I have included timing info for that too.

Timings are approximate (they aren't coming from a benchmark tool). They are
running on an Intel i3-8109U (4) @ 3.600GHz with 8GB of RAM and some
basic SSD. OS is Debian Buster. Day24 was timed on an M1 MacBook Pro, because
the little i3 machine ran out of RAM trying to run it.

| Day | Title                   | Part 1            | Part 2            |
|-----|-------------------------|-------------------|-------------------|
| 1   | Sonar Sweep             | 39.904µs          | 45.991µs          |
| 2   | Dive!                   | 90.951µs          | 88.139µs          |
| 3   | Binary Diagnostic       | 363.728µs         | 353.476µs         |
| 4   | Giant Squid             | 76ns (1.479774ms) | 40ns (1.479774ms) |
| 5   | Hydrothermal Venture    | 7.820412ms        | 14.522496ms       |
| 6   | Lanternfish             | 5.55µs            | 4.72µs            |
| 7   | The Treachery of Whales | 387.301µs         | 3.003278ms        |
| 8   | Seven Segment Search    | 354.545µs         | 117.896284ms      |
| 9   | Smoke Basin             | 1.959009ms        | 5.34808ms         |
| 10  | Syntax Scoring          | 129.367µs         | 151.763µs         |
| 11  | Dumbo Octopus           | 1.468121ms        | 3.608265ms        |
| 12  | Passage Pathing         | 1.599947ms        | 44.621352ms       |
| 13  | Transparent Origami     | 185.854µs         | 301.42µs          |
| 14  | Extended Polymerization | 109.409µs         | 365.583µs         |
| 15  | Chiton                  | 3.329833ms        | 136.242458ms      |
| 16  | Packet Decoder          | 163.291µs         | 142.25µs          |
| 17  | Trick Shot              | 863.768µs         | 1.542742ms        |
| 18  | Snailfish               | 2.314282ms        | 36.458083ms       |
| 19  | Beacon Scanner          | 1.827810354s      | 1.831874847s      |
| 20  | Trench Map              | 6.521235ms        | 331.531499ms      |
| 21  | Dirac Dice              | 3.612µs           | 172.758068ms      |
| 22  | Reactor Reboot          | 5.6415ms          | 68.50499ms        |
| 23  | Amphipod                | 1.471228134s      | 3.15547152s       |
| 24  | Arithmetic Logic Unit   | 50.330940166s     | 50.653740375s     |
| 25  | Sea Cucumber            | 124.53114ms       | N/A               |

