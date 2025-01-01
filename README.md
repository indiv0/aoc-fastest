# aoc-fastest

here's the total of the fastest times for each day so far:
```
day | part |    time | rayon | user            | source available
--- | ---- | ------- | ----- | --------------- | ----------------
  1 |    1 |    9150 |    no | doge            | yes
  1 |    2 |    4945 |    no | doge            | yes
  2 |    1 |    3274 |    no | giooschi        | yes
  2 |    2 |    3749 |    no | giooschi        | yes
  3 |    1 |    2138 |    no | alion02         | yes
  3 |    2 |    2377 |    no | giooschi        | yes
  4 |    1 |    3636 |    no | giooschi        | yes
  4 |    2 |     691 |    no | bendn           | yes
  5 |    1 |    5467 |    no | giooschi        | yes
  5 |    2 |    9440 |    no | giooschi        | yes
  6 |    1 |    5527 |    no | doge            | yes
  6 |    2 |   66803 |   yes | giooschi        | yes
  7 |    1 |    5413 |    no | giooschi        | yes
  7 |    2 |    7516 |    no | giooschi        | yes
  8 |    1 |     725 |    no | alion02         | yes
  8 |    2 |    2146 |    no | bendn           | yes
  9 |    1 |   15850 |    no | alion02         | yes
  9 |    2 |   49969 |    no | ameo            | yes
 10 |    1 |    3013 |    no | giooschi        | yes
 10 |    2 |    4488 |    no | _mwlsk          |  no*
 11 |    1 |      22 |    no | giooschi        | yes
 11 |    2 |      19 |    no | giooschi        | yes
 12 |    1 |   24238 |    no | giooschi        | yes
 12 |    2 |   25721 |    no | giooschi        | yes
 13 |    1 |    1902 |    no | alion02         | yes
 13 |    2 |    2128 |    no | goldsteinq      |  no*
 14 |    1 |    3540 |    no | giooschi        | yes
 14 |    2 |    2072 |    no | giooschi        | yes
 15 |    1 |   24386 |    no | alion02         | yes
 15 |    2 |   34862 |    no | alion02         | yes
 16 |    1 |   43778 |    no | alion02         | yes
 16 |    2 |   56360 |    no | giooschi        | yes
 17 |    1 |      12 |    no | alion02         | yes
 17 |    2 |       1 |    no | alion02         | yes
 18 |    1 |    2865 |    no | alion02         | yes
 18 |    2 |   12838 |    no | caavik          | yes
 19 |    1 |   12362 |    no | giooschi        | yes
 19 |    2 |   18610 |   yes | giooschi        | yes
 20 |    1 |   16407 |    no | giooschi        | yes
 20 |    2 |   47626 |   yes | giooschi        | yes
 21 |    1 |       3 |    no | bendn/giooschi  | yes
 21 |    2 |       3 |    no | bendn/giooschi  | yes
 22 |    1 |    6703 |    no | giooschi        | yes
 22 |    2 |  423158 |   yes | caavik+giooschi | yes
 23 |    1 |   10031 |    no | giooschi        | yes
 23 |    2 |    7357 |    no | giooschi        | yes
 24 |    1 |    1830 |    no | giooschi        | yes
 24 |    2 |    1436 |    no | giooschi        | yes
 25 |    1 |    2335 |    no | giooschi        | yes
-----------------------------------------------------------------
              988922ns
```
for a total of 989us!

For any entry where source available is `no*`, the next fastest solution is
shown instead because the author of the fastest solution has not yet agreed to
have their code displayed here.

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
