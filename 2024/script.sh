#!/usr/bin/env sh

get_and_save() {
  day=${1}
  part=${2}
  time=${3}
  database=../../ferris-elf/database.db
  query="select code from runs where day=${day} and part=${part} AND time=${time} limit 1;"
  # Fetch code from DB;
  # Change newline to UNIX;
  # Save to file.
  sqlite3 ${database} "${query}" | sed 's/\r$//' > d${day}p${part}.rs
}

#get_and_save  1 1   5484
#get_and_save  1 2   2425
get_and_save  2 1   5002
get_and_save  2 2   6949
get_and_save  3 1   1676
get_and_save  3 2   1468
get_and_save  4 1   2259
get_and_save  4 2    473
get_and_save  5 1   3270
get_and_save  5 2   5613
#get_and_save  6 1   4643
get_and_save  6 2  94170
get_and_save  7 1  19841
get_and_save  7 2  30624
get_and_save  8 1    522
get_and_save  8 2   1326
get_and_save  9 1  12472
get_and_save  9 2  32345
get_and_save 10 1  12095
#get_and_save 10 2   3250
get_and_save 11 1     13
get_and_save 11 2     13
get_and_save 12 1  58662
get_and_save 12 2  58601
#get_and_save 13 1   1121
get_and_save 13 2   1205
get_and_save 14 1   1942
get_and_save 14 2   1183
get_and_save 15 1  13062
get_and_save 15 2  18900
get_and_save 16 1  23594
get_and_save 16 2  35869
get_and_save 17 1      7
get_and_save 17 2      0
get_and_save 18 1   1949
get_and_save 18 2   8187
get_and_save 19 1  28859
get_and_save 19 2  29823
get_and_save 20 1   9147
get_and_save 20 2  64162
#get_and_save 21 1      1
#get_and_save 21 2      1
get_and_save 22 1   4728
get_and_save 22 2 568491
get_and_save 23 1   6446
get_and_save 23 2   4657
get_and_save 24 1    898
get_and_save 24 2    834
get_and_save 25 1   1436
