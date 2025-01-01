# aoc-fastest

*Check below for resources and optimization guides!*

## Quick Links to Solutions

- [D4P2 -- **Shortest Overall**](https://github.com/indiv0/aoc-fastest/blob/c3a2c3fa992441a481e6c15927b2cca28d715040/2024/d04p2.rs)
- [D17P2 -- **Fastest; LUTs + ASM**](https://github.com/indiv0/aoc-fastest/blob/c3a2c3fa992441a481e6c15927b2cca28d715040/2024/d17p2.rs)
- [D17P1 -- **Shortest w/ X86_64 Intrinsics**](https://github.com/indiv0/aoc-fastest/blob/c3a2c3fa992441a481e6c15927b2cca28d715040/2024/d17p1.rs)
- [D9P2 - **Most Documented**](https://github.com/indiv0/aoc-fastest/blob/c3a2c3fa992441a481e6c15927b2cca28d715040/2024/d09p2.rs)

## Results

here's the total of the fastest times for each day so far:
```
day | part |    time | user            | source available
--- | ---- | ------- | --------------- | ----------------
  1 |    1 |    9150 | doge            | yes
  1 |    2 |    4945 | doge            | yes
  2 |    1 |    3274 | giooschi        | yes
  2 |    2 |    3749 | giooschi        | yes
  3 |    1 |    2138 | alion02         | yes
  3 |    2 |    2391 | ameo            | yes
  4 |    1 |    3636 | giooschi        | yes
  4 |    2 |     691 | bendn           | yes
  5 |    1 |    5467 | giooschi        | yes
  5 |    2 |    9440 | giooschi        | yes
  6 |    1 |    5527 | doge            | yes
  6 |    2 |   66803 | giooschi        | yes
  7 |    1 |    5413 | giooschi        | yes
  7 |    2 |    7516 | giooschi        | yes
  8 |    1 |     725 | alion02         | yes
  8 |    2 |    2146 | bendn           | yes
  9 |    1 |   15850 | alion02         | yes
  9 |    2 |   49969 | ameo            | yes
 10 |    1 |    3013 | giooschi        | yes
 10 |    2 |    4488 | _mwlsk          |  no*
 11 |    1 |      22 | giooschi        | yes
 11 |    2 |      19 | giooschi        | yes
 12 |    1 |   24238 | giooschi        | yes
 12 |    2 |   25721 | giooschi        | yes
 13 |    1 |    1902 | alion02         | yes
 13 |    2 |    2128 | goldsteinq      |  no*
 14 |    1 |    3540 | giooschi        | yes
 14 |    2 |    2072 | giooschi        | yes
 15 |    1 |   24386 | alion02         | yes
 15 |    2 |   34862 | alion02         | yes
 16 |    1 |   43778 | alion02         | yes
 16 |    2 |   56360 | giooschi        | yes
 17 |    1 |      12 | alion02         | yes
 17 |    2 |       1 | alion02         | yes
 18 |    1 |    2865 | alion02         | yes
 18 |    2 |   12838 | caavik          | yes
 19 |    1 |   12362 | giooschi        | yes
 19 |    2 |   18610 | giooschi        | yes
 20 |    1 |   16407 | giooschi        | yes
 20 |    2 |   47626 | giooschi        | yes
 21 |    1 |       3 | bendn/giooschi  | yes
 21 |    2 |       3 | bendn/giooschi  | yes
 22 |    1 |    6703 | giooschi        | yes
 22 |    2 |  423158 | caavik+giooschi | yes
 23 |    1 |   10031 | giooschi        | yes
 23 |    2 |    7357 | giooschi        | yes
 24 |    1 |    1830 | giooschi        | yes
 24 |    2 |    1436 | giooschi        | yes
 25 |    1 |    2335 | giooschi        | yes
---------------------------------------------------------
              988936ns
```
for a total of 989us!

For any entry where source available is `no*`, the next fastest solution is
shown instead because the author of the fastest solution has not yet agreed to
have their code displayed here.

# Further Reading

IMO the best way to learn is to participate, which is why I highly encourage people to try to optimize AoC solutions themselves. It's a **fantastic** way to learn SIMD. If you decide to do so, absolutely join the [Rust Programming Language Community discord server](https://discord.gg/rust-lang-community)! It's a wonderful community with incredibly talented and knowledgeable folks who are happy to help you optimize. I've learned about topics like instruction pipelines, cache misses, and SIMD just by following the discussions there!

In-depth explanations of these topics would be super helpful. I hope to some day write those explanations myself.

In the meantime, if you would like a more in-depth explanation of some of the optimization techniques used, I highly recommend you check out this article by ameo (one of our participants). It covers the process they used to optimize their solution for Day 9 Part 2, and how they got it to the top of our leaderboard. The article provides incredible information on the process of both high-level and micro optimization:

- [Optimizing Advent of Code D9P2 with High-Performance Rust](https://cprimozic.net/blog/optimizing-advent-of-code-2024/)

Also check out the following:

- [Algorithms for Modern Hardware](https://en.algorithmica.org/hpc/)
- [Optimising my Rust solutions for Advent of Code ](https://nindalf.com/posts/optimising-rust/)
- [500 ⭐ in less than a second (Comment)](https://old.reddit.com/r/adventofcode/comments/1hlyocd/500_in_less_than_a_second/m3pyxdk/)
- [500 ⭐ in less than a second (Repo)](https://github.com/maneatingape/advent-of-code-rust)
- [One Billion Row Challenge](https://curiouscoding.nl/posts/1brc/)


# Credits

This repo contains code/optimizations from the following authors:

- giooschi/skifire13: https://github.com/SkiFire13
- alion02: https://github.com/alion02
- caavik: https://github.com/CameronAavik
- void*/\_\_main\_character\_\_
- ameo https://github.com/Ameobea/advent-of-code-2024
  - See also: https://cprimozic.net/blog/optimizing-advent-of-code-2024/
- doge
- bend-n https://github.com/bend-n/

Thank you so much to these talented individuals for participating in AoC with us!
I highly encourage you to checkout their repos for more details and examples.
These users are incredibly talented at what they do.
Thank you to everyone who particpated as well, even if your submissions did not end up on the top spots of the leaderboard!

This repo contains code submitted to the https://github.com/indiv0/ferris-elf bot, by multiple users.
Code is only included in this repo if the authors have explicitly provided permission to post their code here, so some solutions may not be present.
