// Original by: giooschi
pub fn run(input: &str) -> i64 {
    part1(input) as i64
}

static LT_VALID: [bool; 256] = {
    let mut out = [false; 256];
    out[1] = true;
    out[2] = true;
    out[3] = true;
    out
};

#[inline(always)]
fn lt_valid(diff: i8) -> bool {
    LT_VALID[diff as u8 as usize]
}

static GT_VALID: [bool; 256] = {
    let mut out = [false; 256];
    out[253] = true;
    out[254] = true;
    out[255] = true;
    out
};

#[inline(always)]
fn gt_valid(diff: i8) -> bool {
    GT_VALID[diff as u8 as usize]
}

pub fn part1(input: &str) -> u32 {
    let mut input = input.as_bytes().iter();

    unsafe fn read(input: &mut std::slice::Iter<u8>) -> (i8, u8) {
        let d1 = *input.next().unwrap_unchecked();
        let mut d2 = *input.next().unwrap_unchecked();

        let mut n = d1 - b'0';

        if d2 >= b'0' {
            n = 10 * n + (d2 - b'0');
            d2 = *input.next().unwrap_unchecked();
        }

        (n as i8, d2)
    }

    let mut count = 0;
    unsafe {
        while !input.as_slice().is_empty() {
            let (n1, _) = read(&mut input);
            let (n2, c2) = read(&mut input);

            let diff = n2 - n1;

            static VALID: [bool; 256] = {
                let mut out = [false; 256];
                out[253] = true;
                out[254] = true;
                out[255] = true;
                out[1] = true;
                out[2] = true;
                out[3] = true;
                out
            };

            let mut prev = n2;
            let mut ctrl = c2;
            let mut valid = VALID[diff as u8 as usize];

            if valid {
                if diff > 0 {
                    while valid && ctrl != b'\n' {
                        let (n, c) = read(&mut input);
                        let new_diff = n - prev;
                        (prev, ctrl) = (n, c);

                        valid &= lt_valid(new_diff);
                    }
                } else {
                    while valid && ctrl != b'\n' {
                        let (n, c) = read(&mut input);
                        let new_diff = n - prev;
                        (prev, ctrl) = (n, c);

                        valid &= gt_valid(new_diff);
                    }
                }
            }

            if ctrl != b'\n' {
                while *input.next().unwrap_unchecked() != b'\n' {}
            }

            if valid {
                count += 1;
            }
        }
    }

    count
}

pub fn part2(input: &str) -> u32 {
    let mut input = input.as_bytes().iter();

    unsafe fn read(input: &mut std::slice::Iter<u8>) -> (i8, u8) {
        let d1 = *input.next().unwrap_unchecked();
        let mut d2 = *input.next().unwrap_unchecked();

        let mut n = d1 - b'0';

        if d2 >= b'0' {
            n = 10 * n + (d2 - b'0');
            d2 = *input.next().unwrap_unchecked();
        }

        (n as i8, d2)
    }

    let mut count = 0;
    unsafe {
        while !input.as_slice().is_empty() {
            let (n1, _) = read(&mut input);
            let (n2, c2) = read(&mut input);

            let diff = n2 - n1;

            let mut prevprev = n1;
            let mut prev = n2;
            let mut ctrl = c2;

            static STATE_MAP: [[u8; 4]; 4] =
                [[2, 1, 0, 0], [4, 3, 3, 3], [4, 3, 4, 3], [4, 4, 3, 3]];

            let mut lt_st = if lt_valid(diff) { 0 } else { 1 };
            let mut gt_st = if gt_valid(diff) { 0 } else { 1 };

            while lt_st != 4 && gt_st != 4 && ctrl != b'\n' {
                let (n, c) = read(&mut input);
                let p_diff = n - prev;
                let pp_diff = n - prevprev;

                let lt_idx = 2 * (lt_valid(p_diff) as usize) + lt_valid(pp_diff) as usize;
                let gt_idx = 2 * (gt_valid(p_diff) as usize) + gt_valid(pp_diff) as usize;

                lt_st = *STATE_MAP
                    .get_unchecked(lt_st as usize)
                    .get_unchecked(lt_idx);
                gt_st = *STATE_MAP
                    .get_unchecked(gt_st as usize)
                    .get_unchecked(gt_idx);

                (prevprev, prev, ctrl) = (prev, n, c);
            }

            if lt_st != 4 {
                while lt_st == 0 && ctrl != b'\n' {
                    let (n, c) = read(&mut input);
                    let p_diff = n - prev;

                    if !lt_valid(p_diff) {
                        let pp_diff = n - prevprev;
                        let lt_idx = 2 * (lt_valid(p_diff) as usize) + lt_valid(pp_diff) as usize;

                        lt_st = *STATE_MAP
                            .get_unchecked(lt_st as usize)
                            .get_unchecked(lt_idx);
                    }

                    (prevprev, prev, ctrl) = (prev, n, c);
                }

                if ctrl != b'\n' {
                    let (n, c) = read(&mut input);
                    let p_diff = n - prev;
                    let pp_diff = n - prevprev;

                    let lt_idx = 2 * (lt_valid(p_diff) as usize) + lt_valid(pp_diff) as usize;

                    lt_st = *STATE_MAP
                        .get_unchecked(lt_st as usize)
                        .get_unchecked(lt_idx);

                    (prev, ctrl) = (n, c);
                }

                while lt_st == 3 && ctrl != b'\n' {
                    let (n, c) = read(&mut input);
                    let p_diff = n - prev;

                    if !lt_valid(p_diff) {
                        lt_st = 4;
                    }

                    (prev, ctrl) = (n, c);
                }
            } else if gt_st != 4 {
                while gt_st == 0 && ctrl != b'\n' {
                    let (n, c) = read(&mut input);
                    let p_diff = n - prev;

                    if !gt_valid(p_diff) {
                        let pp_diff = n - prevprev;
                        let gt_idx = 2 * (gt_valid(p_diff) as usize) + gt_valid(pp_diff) as usize;

                        gt_st = *STATE_MAP
                            .get_unchecked(gt_st as usize)
                            .get_unchecked(gt_idx);
                    }

                    (prevprev, prev, ctrl) = (prev, n, c);
                }

                if ctrl != b'\n' {
                    let (n, c) = read(&mut input);
                    let p_diff = n - prev;
                    let pp_diff = n - prevprev;

                    let gt_idx = 2 * (gt_valid(p_diff) as usize) + gt_valid(pp_diff) as usize;

                    gt_st = *STATE_MAP
                        .get_unchecked(gt_st as usize)
                        .get_unchecked(gt_idx);

                    (prev, ctrl) = (n, c);
                }

                while gt_st == 3 && ctrl != b'\n' {
                    let (n, c) = read(&mut input);
                    let p_diff = n - prev;

                    if !gt_valid(p_diff) {
                        gt_st = 4;
                    }

                    (prev, ctrl) = (n, c);
                }
            }

            if ctrl != b'\n' {
                while *input.next().unwrap_unchecked() != b'\n' {}
            }

            if lt_st != 4 || gt_st != 4 {
                count += 1;
            }
        }
    }

    count
}
