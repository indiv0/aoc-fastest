#!/usr/bin/env sh

get_and_save() {
  day=${1}
  part=${2}
  time=${3}
  database=../../ferris-elf/database.db
  query="select code from runs where day=${day} and part=${part} AND time=${time} limit 1;"
  file="d${day}p${part}.rs"
  # Fetch code from DB;
  # Change newline to UNIX;
  # Save to file.
  sqlite3 ${database} "${query}" | sed 's/\r$//' > ${file}
  rustfmt ${file}
}

get_and_save  1 1   9150
get_and_save  1 2   4945
get_and_save  2 1   3274
get_and_save  2 2   3749
get_and_save  3 1   2138
get_and_save  3 2   2391
get_and_save  4 1   3636
get_and_save  4 2    691
get_and_save  5 1   5467
get_and_save  5 2   9440
get_and_save  6 1   5527
get_and_save  6 2  66803
get_and_save  7 1   5413
get_and_save  7 2   7516
get_and_save  8 1    725
get_and_save  8 2   2146
get_and_save  9 1  15850
get_and_save  9 2  49969
get_and_save 10 1   3013
get_and_save 10 2  4908 # 4488
get_and_save 11 1     22
get_and_save 11 2     19
get_and_save 12 1  24238
get_and_save 12 2  25721
get_and_save 13 1   1902
get_and_save 13 2   2181 # 2128
get_and_save 14 1   3540
get_and_save 14 2   2072
get_and_save 15 1  24386
get_and_save 15 2  34862
get_and_save 16 1  43778
get_and_save 16 2  56360
get_and_save 17 1     12
get_and_save 17 2      1
get_and_save 18 1   2865
get_and_save 18 2  12838
get_and_save 19 1  12362
get_and_save 19 2  18610
get_and_save 20 1  16407
get_and_save 20 2  47626
get_and_save 21 1      3
get_and_save 21 2      3
get_and_save 22 1   6703
get_and_save 22 2 423158
get_and_save 23 1  10031
get_and_save 23 2   7357
get_and_save 24 1   1830
get_and_save 24 2   1436
get_and_save 25 1   2335
