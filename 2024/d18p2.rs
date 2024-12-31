// Originally by: caavik
use std::fmt;

pub fn run(input: &str) -> impl fmt::Display {
    unsafe {
        const GRID_WIDTH: usize = 71;
        const PADDED_WIDTH: usize = GRID_WIDTH + 2; // Don't need bounds checks
        const START_ID: usize = PADDED_WIDTH + 1;
        const END_ID: usize = GRID_WIDTH * PADDED_WIDTH + GRID_WIDTH;

        struct Coordinate {
            x: usize,
            y: usize,
        }

        impl fmt::Display for Coordinate {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{},{}", self.x, self.y)
            }
        }

        // Initialize data structures equivalent to the C# code
        static mut byte_order: [u16; GRID_WIDTH * GRID_WIDTH] = [0u16; GRID_WIDTH * GRID_WIDTH];
        byte_order.fill(0);
        static mut order_lookup: [u16; PADDED_WIDTH * PADDED_WIDTH] = [u16::MAX; PADDED_WIDTH * PADDED_WIDTH];
        order_lookup.fill(u16::MAX);

        // Mark borders as visited
        static mut visited: [u64; (PADDED_WIDTH * PADDED_WIDTH + 63) / 64] = [0u64; (PADDED_WIDTH * PADDED_WIDTH + 63) / 64];
        visited.fill(0);
        let visited_len = visited.len();
        let overflow = visited_len * 64 - (PADDED_WIDTH * PADDED_WIDTH);

        // mark borders as visited
        *visited.get_unchecked_mut(0) = u64::MAX;
        *visited.get_unchecked_mut(1) |= (1u64 << (PADDED_WIDTH - 64)) - 1;
        *visited.get_unchecked_mut(visited_len - 1) = u64::MAX;
        *visited.get_unchecked_mut(visited_len - 2) |= u64::MAX << (128 - (PADDED_WIDTH + overflow));

        for i in 1..PADDED_WIDTH - 1 {
            let idx1 = (i * PADDED_WIDTH) / 64;
            let bit1 = 1u64 << ((i * PADDED_WIDTH) % 64);
            *visited.get_unchecked_mut(idx1) |= bit1;

            let idx2 = (i * PADDED_WIDTH + PADDED_WIDTH - 1) / 64;
            let bit2 = 1u64 << ((i * PADDED_WIDTH + PADDED_WIDTH - 1) % 64);
            *visited.get_unchecked_mut(idx2) |= bit2;
        }

        static mut reachable: [u64; (GRID_WIDTH * GRID_WIDTH + 63) / 64] = [0u64; (GRID_WIDTH * GRID_WIDTH + 63) / 64];
        reachable.fill(0);

        // Parse the input and populate byte_order and order_lookup arrays
        let mut byte_count: u16 = 0;
        let input_bytes = input.as_bytes();
        let mut idx = 0;
        while idx < input_bytes.len() {
            let mut x = (input_bytes.get_unchecked(idx) - b'0') as usize;
            idx += 1;
            let mut c = *input_bytes.get_unchecked(idx);
            idx += 1;
            if c != b',' {
                x = x * 10 + (c - b'0') as usize;
                idx += 1; // Skip ','
            }

            let mut y = (*input_bytes.get_unchecked(idx) - b'0') as usize;
            idx += 1;
            c = *input_bytes.get_unchecked(idx);
            idx += 1;
            if c != b'\n' {
                y = y * 10 + (c - b'0') as usize;
                idx += 1; // Skip '\n'
            }

            x += 1;
            y += 1;

            let id = (y * PADDED_WIDTH + x) as u16;

            *byte_order.get_unchecked_mut(byte_count as usize) = id;
            *order_lookup.get_unchecked_mut(id as usize) = byte_count;
            byte_count += 1;
        }

        // Mark the start position (0, 0) as the final fallen byte and mark it as reachable/visited
        *byte_order.get_unchecked_mut(byte_count as usize) = START_ID as u16;
        *order_lookup.get_unchecked_mut(START_ID) = byte_count;
        *reachable.get_unchecked_mut((byte_count / 64) as usize) |= 1u64 << (byte_count % 64);
        *visited.get_unchecked_mut(START_ID / 64) |= 1u64 << (START_ID % 64);

        // Implement BFS using a manually managed stack
        let mut bfs_stack = [0u16; 128];
        let mut ptr = 0usize;

        for i in (0..reachable.len()).rev() {
            loop {
                let reachable_i = *reachable.get_unchecked(i);
                let leading_zeros = reachable_i.leading_zeros() as i32;
                let next_in_reach = 63 - leading_zeros;
                if next_in_reach == -1 {
                    break;
                }
                let max_reachable = 64 * i + next_in_reach as usize;
                *reachable.get_unchecked_mut(i) ^= 1u64 << next_in_reach;
                let max_byte = *byte_order.get_unchecked(max_reachable);
                *bfs_stack.get_unchecked_mut(ptr) = max_byte;
                ptr += 1;

                while ptr > 0 {
                    ptr -= 1;
                    let next_byte = *bfs_stack.get_unchecked(ptr);

                    if next_byte as usize == END_ID {
                        let result_x = (max_byte as usize % PADDED_WIDTH) - 1;
                        let result_y = (max_byte as usize / PADDED_WIDTH) - 1;
                        return Coordinate { x: result_x, y: result_y };
                    }

                    // Neighboring positions
                    let neighbors = [
                        next_byte - PADDED_WIDTH as u16, // Up
                        next_byte - 1,                   // Left
                        next_byte + 1,                   // Right
                        next_byte + PADDED_WIDTH as u16, // Down
                    ];

                    for &neighbor in &neighbors {
                        let bit = 1u64 << (neighbor as usize % 64);
                        let idx = neighbor as usize / 64;
                        if (*visited.get_unchecked(idx) & bit) == 0 {
                            *visited.get_unchecked_mut(idx) |= bit;
                            let neighbor_order = *order_lookup.get_unchecked(neighbor as usize);
                            if neighbor_order > max_reachable as u16 {
                                *bfs_stack.get_unchecked_mut(ptr) = neighbor;
                                ptr += 1;
                            } else {
                                let order_idx = (neighbor_order / 64) as usize;
                                *reachable.get_unchecked_mut(order_idx) |= 1u64 << (neighbor_order % 64);
                            }
                        }
                    }
                }
            }
        }

        // If not found, return a default coordinate
        Coordinate { x: 0, y: 0 }
    }
}

