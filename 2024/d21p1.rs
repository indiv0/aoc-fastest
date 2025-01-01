// Original by: bendn
#![allow(warnings)]
#![allow(
    confusable_idents,
    uncommon_codepoints,
    non_upper_case_globals,
    internal_features,
    mixed_script_confusables,
    static_mut_refs,
    incomplete_features
)]
#![feature(
    iter_repeat_n,
    slice_swap_unchecked,
    generic_const_exprs,
    iter_array_chunks,
    if_let_guard,
    get_many_mut,
    maybe_uninit_uninit_array,
    once_cell_get_mut,
    iter_collect_into,
    hint_assert_unchecked,
    let_chains,
    anonymous_lifetime_in_impl_trait,
    array_windows,
    maybe_uninit_array_assume_init,
    vec_into_raw_parts,
    try_blocks,
    slice_take,
    portable_simd,
    test,
    slice_as_chunks,
    array_chunks,
    slice_split_once,
    core_intrinsics
)]
use std::ops::Neg;

mod util {
    #![allow(non_snake_case, unused_macros)]

    use rustc_hash::FxHashMap as HashMap;
    use rustc_hash::FxHashSet as HashSet;
    use std::{
        cmp::Reverse,
        collections::{hash_map::Entry, BinaryHeap},
        fmt::{Debug, Display, Write},
        hash::Hash,
        mem::swap,
        ops::RangeInclusive,
        str::FromStr,
    };

    pub mod prelude {
        #[allow(unused_imports)]
        pub(crate) use super::{bits, dang, leek, mat, shucks, C};
        pub use super::{
            even, gcd, gt, l, lcm, lt, nail, pa, r, rand, reading, reading::Ext, sort, DigiCount,
            Dir, FilterBy, FilterBy3, GreekTools, IntoCombinations, IntoLines, IterͶ,
            NumTupleIterTools, ParseIter, Printable, Skip, TakeLine, TupleIterTools2,
            TupleIterTools2R, TupleIterTools3, TupleUtils, UnifiedTupleUtils, UnsoundUtilities,
            Widen, Ͷ, Α, Κ, Λ, Μ,
        };
        pub use itertools::izip;
        pub use itertools::Itertools;
        pub use rustc_hash::FxHashMap as HashMap;
        pub use rustc_hash::FxHashSet as HashSet;
        pub use std::{
            cmp::Ordering::*,
            cmp::{max, min},
            collections::{hash_map::Entry, VecDeque},
            fmt::{Debug, Display},
            hint::black_box as boxd,
            io::{self, Read, Write},
            iter,
            mem::{replace as rplc, swap, transmute as rint},
            ops::Range,
        };
    }

    macro_rules! C {
        ($obj:ident.$what:ident$($tt:tt)+) => {{
            let x = &mut $obj.$what;
            C!( x$($tt)+ )
        }};
        (&$buf:ident[$n:expr]) => {{
            #[allow(unused_unsafe)]
            unsafe {
                $buf.get_unchecked($n)
            }
        }};
        ($buf:ident[$n:expr]) => {{
            #[allow(unused_unsafe)]
            *unsafe {
                $buf.get_unchecked($n)
            }
        }};
        (&mut $buf:ident[$n:expr]) => {{
            #[allow(unused_unsafe)]
            unsafe {
                $buf.get_unchecked_mut($n)
            }
        }};
        ($buf:ident[$a:expr] = $rbuf:ident[$b:expr]) => {
            *unsafe { $buf.get_unchecked_mut($a) } = unsafe { *$rbuf.get_unchecked($b) }
        };
        ($buf:ident[$n:expr] = $e:expr) => {
            *unsafe { $buf.get_unchecked_mut($n) } = $e
        };
        ($buf:ident[$a:expr][$b:expr]) => {
            unsafe { *$buf.get_unchecked($a).get_unchecked($b) }
        };
        ($buf:ident[$a:expr][$b:expr] = $rbuf:ident[$ra:expr]) => {
            *unsafe { $buf.get_unchecked_mut($a).get_unchecked_mut($b) } =
                unsafe { *$rbuf.get_unchecked($ra) }
        };
        ($buf:ident[$a:expr][$b:expr] = $rbuf:ident[$ra:expr][$rb:expr]) => {
            *unsafe { $buf.get_unchecked_mut($a).get_unchecked_mut($b) } =
                unsafe { *$rbuf.get_unchecked($ra).get_unchecked($rb) }
        };
        ($buf:ident[$a:expr][$b:expr] = $c:expr) => {{
            #[allow(unused_unsafe)]
            {
                *unsafe { $buf.get_unchecked_mut($a).get_unchecked_mut($b) } = unsafe { $c }
            }
        }};
    }
    pub(crate) use C;

    macro_rules! shucks {
        () => {
            if cfg!(debug_assertions) {
                unreachable!();
            } else {
                unsafe { std::hint::unreachable_unchecked() }
            }
        };
        ($fmt:literal $(, $args:expr)* $(,)?) => {
            if cfg!(debug_assertions) {
                unreachable!($fmt $(, $args)*);
            } else {
                unsafe { std::hint::unreachable_unchecked() }
            }
        };
        (if $x:expr) => {
            if $x {
                if cfg!(debug_assertions) {
                    unreachable!();
                } else {
                    unsafe { std::hint::unreachable_unchecked() }
                }
            }
        };
    }
    pub(crate) use shucks;

    macro_rules! dang {
        () => {
            panic!()
        };
    }
    pub(crate) use dang;

    macro_rules! leek {
        ($($allocation:ident)+) => {
            $(std::mem::forget($allocation);)+
        };
    }
    pub(crate) use leek;

    macro_rules! mat {
        ($thing:ident { $($what:pat => $b:expr,)+ }) => {
            match $thing { $($what => { $b })+ _ => shucks!() }
        };
    }
    pub(crate) use mat;

    #[cfg(target_feature = "avx2")]
    unsafe fn count_avx<const N: usize>(hay: &[u8; N], needle: u8) -> usize {
        use std::arch::x86_64::*;
        let find = _mm256_set1_epi8(needle as i8);
        let mut counts = _mm256_setzero_si256();
        for i in 0..(N / 32) {
            counts = _mm256_sub_epi8(
                counts,
                _mm256_cmpeq_epi8(
                    _mm256_loadu_si256(hay.as_ptr().add(i * 32) as *const _),
                    find,
                ),
            );
        }
        const MASK: [u8; 64] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        ];
        counts = _mm256_sub_epi8(
            counts,
            _mm256_and_si256(
                _mm256_cmpeq_epi8(
                    _mm256_loadu_si256(hay.as_ptr().add(N - 32) as *const _),
                    find,
                ),
                _mm256_loadu_si256(MASK.as_ptr().add(N % 32) as *const _),
            ),
        );

        let sums = _mm256_sad_epu8(counts, _mm256_setzero_si256());
        (_mm256_extract_epi64(sums, 0)
            + _mm256_extract_epi64(sums, 1)
            + _mm256_extract_epi64(sums, 2)
            + _mm256_extract_epi64(sums, 3)) as usize
    }

    pub fn count<const N: usize>(hay: &[u8; N], what: u8) -> usize {
        #[cfg(target_feature = "avx2")]
        return unsafe { count_avx(hay, what) };
        #[cfg(not(target_feature = "avx2"))]
        hay.iter().filter(|&&x| x == what).count()
    }

    pub fn lcm(n: impl IntoIterator<Item = u64>) -> u64 {
        let mut x = n.into_iter();
        let mut lcm = x.by_ref().next().expect("cannot compute LCM of 0 numbers");
        let mut gcd;
        for x in x {
            gcd = crate::util::gcd(x, lcm);
            lcm = (lcm * x) / gcd;
        }
        lcm
    }

    #[repr(u8)]
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
    pub enum Dir {
        N = b'U',
        E = b'R',
        S = b'D',
        W = b'L',
    }

    pub struct UnionFind {
        p: Vec<usize>,
        s: Vec<usize>,
    }

    impl UnionFind {
        pub fn new(size: usize) -> Self {
            Self {
                s: vec![1; size],
                p: (0..size).collect(),
            }
        }

        fn reset(&mut self) {
            self.s.fill(1);
            self.p
                .iter_mut()
                .enumerate()
                .for_each(|(idx, val)| *val = idx);
        }

        pub fn find(&mut self, key: usize) -> usize {
            if self.p[key] == key {
                return key;
            }
            let parent = self.find(self.p[key]);
            self.p[key] = parent;
            parent
        }

        pub fn union(&mut self, a: usize, b: usize) -> bool {
            let α = self.find(a);
            let β = self.find(b);
            if α == β {
                return false;
            }
            let a = self.s[α];
            let b = self.s[β];
            if a >= b {
                self.s[α] += b;
                self.p[β] = α;
            } else {
                self.s[β] += a;
                self.p[α] = β;
            }
            true
        }

        fn group_size(&self, group: usize) -> usize {
            self.s[group]
        }
    }

    pub trait UnsoundUtilities<T> {
        fn ψ(self) -> T;
    }

    impl<T> UnsoundUtilities<T> for Option<T> {
        fn ψ(self) -> T {
            if cfg!(debug_assertions) && self.is_none() {
                panic!();
            }
            unsafe { self.unwrap_unchecked() }
        }
    }

    impl<T, E> UnsoundUtilities<T> for Result<T, E> {
        #[cfg_attr(debug_assertions, track_caller)]
        fn ψ(self) -> T {
            if cfg!(debug_assertions) && self.is_err() {
                panic!();
            }
            unsafe { self.unwrap_unchecked() }
        }
    }

    pub struct LMap<K, V>(HashMap<K, V>, fn(K) -> V)
    where
        K: Eq + Hash + Copy;
    impl<K: Ord + Eq + Debug + Hash + Copy, V: Copy + Debug> LMap<K, V> {
        pub fn new(f: fn(K) -> V) -> Self {
            Self {
                0: HashMap::default(),
                1: f,
            }
        }

        pub fn with_cap(f: fn(K) -> V, cap: usize) -> Self {
            Self {
                0: HashMap::with_capacity_and_hasher(cap, rustc_hash::FxBuildHasher::default()),
                1: f,
            }
        }

        pub fn get(&mut self, k: K) -> V {
            if let Some(x) = self.0.get(&k) {
                return *x;
            }
            // let mut ks = self.0.keys().collect::<Vec<_>>();
            // ks.sort();
            // println!("{ks:?}");
            let elm = self.1(k);
            self.0.insert(k, elm);
            elm
        }
    }

    macro_rules! memoize {
        (|$pat:pat_param in $in:ty| -> $out:ty $body:block; $arg:expr) => {{
            static mut MEMOIZER: std::sync::OnceLock<crate::util::LMap<$in, $out>> =
                std::sync::OnceLock::new();
            unsafe {
                MEMOIZER.get_mut_or_init(|| crate::util::LMap::new(|$pat: $in| -> $out { $body }))
            }
            .get($arg)
        }};
        (|$pat:pat_param in $in:ty| -> $out:ty $body:block; $arg:expr; with cap $cap:literal) => {{
            static mut MEMOIZER: std::sync::OnceLock<crate::util::LMap<$in, $out>> =
                std::sync::OnceLock::new();
            unsafe {
                MEMOIZER.get_mut_or_init(|| {
                    crate::util::LMap::with_cap(|$pat: $in| -> $out { $body }, $cap)
                })
            }
            .get($arg)
        }};
    }
    pub(crate) use memoize;

    pub fn countg_with_check<N: Debug + PartialEq + Hash + Eq + Copy, I: Iterator<Item = N>>(
        start: N,
        graph: &mut impl Fn(N) -> I,
        ok: &mut impl Fn(N, N) -> bool,
        sum: &mut usize,
        end: &mut impl Fn(N) -> bool,
    ) {
        if end(start) {
            *sum += 1;
        } else {
            graph(start)
                .map(|x| {
                    if ok(start, x) {
                        // println!("\"{start:?}\" -> \"{x:?}\"");
                        countg_with_check(x, graph, ok, sum, end);
                    }
                })
                .Θ();
        }
    }

    pub fn countg_uniq_with_check<
        N: Debug + PartialEq + Hash + Eq + Copy,
        I: Iterator<Item = N>,
    >(
        start: N,
        graph: &mut impl Fn(N) -> I,
        ok: &mut impl Fn(N, N) -> bool,
        sum: &mut usize,
        end: &mut impl Fn(N) -> bool,
        has: &mut HashSet<N>,
    ) {
        if end(start) && has.insert(start) {
            *sum += 1;
        } else {
            graph(start)
                .map(|x| {
                    if ok(start, x) {
                        countg_uniq_with_check(x, graph, ok, sum, end, has);
                    }
                })
                .Θ();
        }
    }

    pub fn countg<N: Debug + PartialEq + Hash + Eq + Copy, I: Iterator<Item = N>>(
        start: N,
        graph: &mut impl Fn(N) -> I,
        sum: &mut usize,
        end: &mut impl Fn(N) -> bool,
        has: &mut HashSet<N>,
    ) {
        if end(start) {
            *sum += 1;
        } else {
            graph(start)
                .map(|x| {
                    if has.insert(x) {
                        countg(x, graph, sum, end, has);
                    }
                })
                .Θ();
        }
    }

    // pub fn appearances(x: )

    pub fn iterg<N: Debug + Copy, I: Iterator<Item = N>>(
        start: N,
        graph: &mut impl Fn(N) -> I,
        end: &mut impl Fn(N) -> bool,
        finally: &mut impl FnMut(N),
    ) {
        if end(start) {
            finally(start);
        } else {
            graph(start).map(|x| iterg(x, graph, end, finally)).Θ();
        };
    }

    pub fn show<N: Debug + Eq + Hash + Copy + Ord, I: Iterator<Item = (N, u16)>, D: Display>(
        graph: impl Fn(N) -> I,
        start: N,
        end: impl Fn(N) -> bool,
        name: impl Fn(N) -> D,
    ) {
        println!("digraph {{");
        let mut s = HashSet::default();
        let mut q = BinaryHeap::new();
        q.push(Reverse((0, start)));
        while let Some(Reverse((c, n))) = q.pop() {
            if end(n) {
                println!("}}");
                return;
            }
            if !s.insert(n) {
                continue;
            }
            print!("\t{}", name(n));
            for (n, d) in graph(n) {
                if s.contains(&n) {
                    continue;
                }
                print!(" -> {}", name(n));
                q.push(Reverse((c + d, n)));
            }
            println!(";");
        }
        dang!();
    }

    pub fn dijkstra_h<N: Debug + Eq + Hash + Copy + Ord, I: Iterator<Item = (N, u16)>>(
        graph: impl Fn(N) -> I,
        start: N,
        end: impl Fn(N) -> bool,
        h: impl Fn(N) -> u16,
    ) -> u16 {
        let mut q = BinaryHeap::new();
        let mut s = HashSet::default();
        q.push(Reverse((h(start), 0, start)));
        while let Some(Reverse((_, c, n))) = q.pop() {
            if end(n) {
                return c;
            }
            if !s.insert(n) {
                continue;
            }
            for (n, d) in graph(n) {
                if s.contains(&n) {
                    continue;
                }
                q.push(Reverse((h(n) + c + d, c + d, n)));
            }
        }
        dang!()
    }

    pub fn dijkstra<N: Debug + Eq + Hash + Copy + Ord, I: Iterator<Item = (N, u16)>>(
        graph: impl Fn(N) -> I,
        start: N,
        end: impl Fn(N) -> bool,
    ) -> u16 {
        let mut q = BinaryHeap::new();
        let mut s = HashSet::default();
        q.push(Reverse((0, start)));
        while let Some(Reverse((c, n))) = q.pop() {
            if end(n) {
                return c;
            }
            if !s.insert(n) {
                continue;
            }
            for (n, d) in graph(n) {
                if s.contains(&n) {
                    continue;
                }
                q.push(Reverse((c + d, n)));
            }
        }
        dang!()
    }

    impl std::ops::Add<(i64, i64)> for Dir {
        type Output = (i64, i64);
        fn add(self, (x, y): (i64, i64)) -> Self::Output {
            match self {
                Dir::N => (x, y - 1),
                Dir::E => (x + 1, y),
                Dir::S => (x, y + 1),
                Dir::W => (x - 1, y),
            }
        }
    }

    impl std::ops::Add<(usize, usize)> for Dir {
        type Output = (usize, usize);
        fn add(self, (x, y): (usize, usize)) -> Self::Output {
            match self {
                Dir::N => (x, y - 1),
                Dir::E => (x + 1, y),
                Dir::S => (x, y + 1),
                Dir::W => (x - 1, y),
            }
        }
    }

    impl std::ops::Add<(i32, i32)> for Dir {
        type Output = (i32, i32);
        fn add(self, (x, y): (i32, i32)) -> Self::Output {
            match self {
                Dir::N => (x, y - 1),
                Dir::E => (x + 1, y),
                Dir::S => (x, y + 1),
                Dir::W => (x - 1, y),
            }
        }
    }

    impl std::ops::Add<(u16, u16)> for Dir {
        type Output = (u16, u16);

        fn add(self, (x, y): (u16, u16)) -> Self::Output {
            match self {
                Dir::N => (x, y - 1),
                Dir::E => (x + 1, y),
                Dir::S => (x, y + 1),
                Dir::W => (x - 1, y),
            }
        }
    }

    impl std::ops::Add<(i16, i16)> for Dir {
        type Output = (i16, i16);
        fn add(self, (x, y): (i16, i16)) -> Self::Output {
            match self {
                Dir::N => (x, y - 1),
                Dir::E => (x + 1, y),
                Dir::S => (x, y + 1),
                Dir::W => (x - 1, y),
            }
        }
    }

    impl std::ops::Add<(u8, u8)> for Dir {
        type Output = Option<(u8, u8)>;

        fn add(self, (x, y): (u8, u8)) -> Self::Output {
            match self {
                Dir::N => Some((x, y.checked_sub(1)?)),
                Dir::E => Some((x + 1, y)),
                Dir::S => Some((x, y + 1)),
                Dir::W => Some((x.checked_sub(1)?, y)),
            }
        }
    }

    impl Dir {
        pub fn turn_90(self) -> Self {
            match self {
                Dir::N => Dir::E,
                Dir::E => Dir::S,
                Dir::S => Dir::W,
                Dir::W => Dir::N,
            }
        }
        pub fn turn_90ccw(self) -> Self {
            match self {
                Dir::N => Dir::W,
                Dir::E => Dir::N,
                Dir::S => Dir::E,
                Dir::W => Dir::S,
            }
        }
    }

    pub fn pa<T: std::fmt::Debug>(a: &[T]) {
        for e in a {
            print!("{e:?}");
        }
        println!();
    }

    pub fn gcd(mut a: u64, mut b: u64) -> u64 {
        if a == 0 || b == 0 {
            return a | b;
        }
        let shift = (a | b).trailing_zeros();
        a >>= shift;
        loop {
            b >>= b.trailing_zeros();
            if a > b {
                swap(&mut a, &mut b);
            }
            b -= a;
            if b == 0 {
                break;
            }
        }
        a << shift
    }

    pub trait Λ {
        fn λ<T: FromStr>(&self) -> T
        where
            <T as FromStr>::Err: std::fmt::Display;
    }

    impl Λ for String {
        fn λ<T: FromStr>(&self) -> T
        where
            <T as FromStr>::Err: std::fmt::Display,
        {
            self.as_str().λ()
        }
    }
    impl Λ for &[u8] {
        fn λ<T: FromStr>(&self) -> T
        where
            <T as FromStr>::Err: std::fmt::Display,
        {
            std::str::from_utf8(self).α().λ()
        }
    }
    impl Λ for &str {
        /// parse, unwrap
        fn λ<T: FromStr>(&self) -> T
        where
            <T as FromStr>::Err: std::fmt::Display,
        {
            match self.parse() {
                Ok(v) => v,
                Err(e) => {
                    panic!(
                        "{e}: {self} should parse into {}",
                        std::any::type_name::<T>()
                    )
                }
            }
        }
    }
    pub trait Κ {
        fn κ<T: FromStr>(self) -> impl Iterator<Item = T>
        where
            <T as FromStr>::Err: std::fmt::Display;
    }

    impl Κ for &[u8] {
        fn κ<T: FromStr>(self) -> impl Iterator<Item = T>
        where
            <T as FromStr>::Err: std::fmt::Display,
        {
            std::str::from_utf8(self).unwrap().κ()
        }
    }

    impl Κ for &str {
        fn κ<T: FromStr>(self) -> impl Iterator<Item = T>
        where
            <T as FromStr>::Err: std::fmt::Display,
        {
            self.split_ascii_whitespace().map(|x| x.λ())
        }
    }

    pub trait Α<T> {
        fn α(self) -> T;
    }

    impl<T, E: std::fmt::Display> Α<T> for Result<T, E> {
        #[cfg_attr(debug_assertions, track_caller)]
        fn α(self) -> T {
            match self {
                Ok(v) => v,
                Err(e) => {
                    panic!("unwrap failed: {e}");
                }
            }
        }
    }
    impl<T> Α<T> for Option<T> {
        #[cfg_attr(debug_assertions, track_caller)]
        fn α(self) -> T {
            match self {
                Some(v) => v,
                None => panic!("nothingness!"),
            }
        }
    }

    pub trait DigiCount {
        fn ͱ(self) -> u32;
    }

    pub const powers: [u64; 20] = car::from_fn!(|x| 10u64.pow(x as u32));
    // https://stackoverflow.com/a/9721570
    impl DigiCount for u64 {
        fn ͱ(self) -> u32 {
            static powers: [u64; 20] = car::from_fn!(|x| 10u64.pow(x as u32));
            static mdigs: [u32; 65] = car::from_fn!(|x| 2u128.pow(x as u32).ilog10() + 1);
            let bit = std::mem::size_of::<Self>() * 8 - self.leading_zeros() as usize;
            let mut digs = mdigs[bit];
            if self < C! { powers[digs as usize - 1] } {
                digs -= 1;
            }
            digs
        }
    }

    impl DigiCount for u32 {
        fn ͱ(self) -> Self {
            static powers: [u32; 10] = car::from_fn!(|x| 10u32.pow(x as u32));
            static mdigs: [u32; 33] = car::from_fn!(|x| 2u128.pow(x as u32).ilog10() + 1);
            let bit = std::mem::size_of::<Self>() * 8 - self.leading_zeros() as usize;
            let mut digs = mdigs[bit];
            if self < C! { powers[digs as usize - 1] } {
                digs -= 1;
            }
            digs
        }
    }

    impl DigiCount for u16 {
        fn ͱ(self) -> u32 {
            self.checked_ilog10().ψ() + 1
        }
    }

    impl DigiCount for u8 {
        fn ͱ(self) -> u32 {
            self.checked_ilog10().ψ() + 1
        }
    }

    impl DigiCount for u128 {
        fn ͱ(self) -> u32 {
            self.checked_ilog10().ψ() + 1
        }
    }

    pub trait Ͷ: DigiCount {
        fn ͷ(self) -> impl Iterator<Item = u8>;
        fn Ͷ(self, i: u8) -> u8;
    }

    macro_rules! digs {
        ($for:ty) => {
            impl Ͷ for $for {
                fn ͷ(self) -> impl Iterator<Item = u8> {
                    let digits = self.ͱ() as u8;
                    (0..digits).rev().map(move |n| self.Ͷ(n))
                }
                fn Ͷ(self, i: u8) -> u8 {
                    ((self / (10 as $for).pow(i as _)) % 10) as u8
                }
            }
        };
    }
    digs!(u128);
    digs!(u64);
    digs!(u32);
    digs!(u16);
    digs!(u8);

    #[derive(Copy, Clone, PartialEq, PartialOrd)]
    pub struct Ronge {
        pub begin: u16,
        pub end: u16,
    }

    impl Debug for Ronge {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}..{}", self.begin, self.end)
        }
    }

    impl Display for Ronge {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}..{}", self.begin, self.end)
        }
    }

    impl From<RangeInclusive<u16>> for Ronge {
        fn from(value: RangeInclusive<u16>) -> Self {
            Self {
                begin: *value.start(),
                end: *value.end(),
            }
        }
    }

    impl PartialEq<RangeInclusive<u16>> for Ronge {
        fn eq(&self, other: &RangeInclusive<u16>) -> bool {
            self == &Self::from(other.clone())
        }
    }

    impl Ronge {
        pub fn sane(self) -> bool {
            self.end >= self.begin
        }
        pub fn checked_len(self) -> Option<u16> {
            self.sane().then(|| self.len())
        }
        pub fn len(self) -> u16 {
            self.end - self.begin
        }

        /// push up
        pub fn pushu(&mut self, to: u16) {
            self.begin = self.begin.max(to);
        }

        /// push down
        pub fn pushd(&mut self, to: u16) {
            self.end = self.end.min(to);
        }

        pub fn intersect(self, with: Self) -> Self {
            Self {
                begin: self.begin.max(with.begin),
                end: self.end.min(with.end),
            }
        }

        pub fn news(&self, begin: u16) -> Self {
            Self {
                begin,
                end: self.end,
            }
        }

        pub fn newe(&self, end: u16) -> Self {
            Self {
                begin: self.begin,
                end,
            }
        }

        pub fn shrink(&mut self, with: Ronge) {
            self.pushu(with.begin);
            self.pushd(with.end);
        }
    }

    impl IntoIterator for Ronge {
        type Item = u16;

        type IntoIter = std::ops::Range<u16>;

        fn into_iter(self) -> Self::IntoIter {
            self.begin..self.end
        }
    }

    pub trait Μ where
        Self: Sized,
    {
        fn μ(self, d: char) -> (Self, Self);
        fn μκ<T: FromStr>(self, d: char) -> impl Iterator<Item = (T, T)>
        where
            <T as FromStr>::Err: std::fmt::Display;

        fn μ1(self, d: char) -> Self {
            self.μ(d).1
        }

        fn μ0(self, d: char) -> Self {
            self.μ(d).0
        }

        fn between(self, a: char, b: char) -> Self {
            self.μ1(a).μ0(b)
        }
    }

    impl Μ for &[u8] {
        fn μ(self, d: char) -> (Self, Self) {
            let i = memchr::memchr(d as u8, self)
                .unwrap_or_else(|| shucks!("{} should split at {d} fine", self.p()));
            (&self[..i], &self[i + 1..])
        }

        fn μκ<T: FromStr>(self, d: char) -> impl Iterator<Item = (T, T)>
        where
            <T as FromStr>::Err: std::fmt::Display,
        {
            let (α, β) = self.μ(d);
            α.κ::<T>().zip(β.κ::<T>())
        }
    }

    pub fn gt<A: std::cmp::PartialOrd<T>, T>(n: T) -> impl Fn(A) -> bool {
        move |a| a > n
    }

    pub fn lt<A: std::cmp::PartialOrd<T>, T>(n: T) -> impl Fn(A) -> bool {
        move |a| a < n
    }

    impl Μ for &str {
        fn μ(self, d: char) -> (Self, Self) {
            self.split_once(d)
                .unwrap_or_else(|| shucks!("{self} should split at {d} fine"))
        }

        fn μκ<T: FromStr>(self, d: char) -> impl Iterator<Item = (T, T)>
        where
            <T as FromStr>::Err: std::fmt::Display,
        {
            let (α, β) = self.μ(d);
            α.κ::<T>().zip(β.κ::<T>())
        }
    }

    pub trait IterͶ: Iterator {
        fn ͷ(self) -> impl Iterator<Item = u8>;
    }

    impl<I: Iterator<Item = u64>> IterͶ for I {
        fn ͷ(self) -> impl Iterator<Item = u8> {
            self.flat_map(Ͷ::ͷ)
        }
    }

    pub trait TupleIterTools3<T, U, V>: Iterator {
        fn l(self) -> impl Iterator<Item = T>;
        fn m(self) -> impl Iterator<Item = U>;
        fn r(self) -> impl Iterator<Item = V>;
        fn lm(self) -> impl Iterator<Item = (T, U)>;
        fn lr(self) -> impl Iterator<Item = (T, V)>;
        fn mr(self) -> impl Iterator<Item = (U, V)>;
    }

    pub trait TupleIterTools2<T, U>: Iterator {
        fn l(self) -> impl Iterator<Item = T>;
        fn r(self) -> impl Iterator<Item = U>;
    }

    pub trait TupleIterTools2R<T, U>: Iterator {
        fn l(self) -> impl Iterator<Item = T>;
        fn r(self) -> impl Iterator<Item = U>;
    }

    pub fn l<R, T, U>(f: impl Fn(T) -> R) -> impl Fn((T, U)) -> R {
        move |(x, _)| f(x)
    }
    pub fn r<R, T, U>(f: impl Fn(U) -> R) -> impl Fn((T, U)) -> R {
        move |(_, x)| f(x)
    }

    pub trait FilterBy3<T, U, V>: Iterator {
        fn fl(self, f: impl Fn(T) -> bool) -> impl Iterator<Item = (T, U, V)>;
        fn fm(self, f: impl Fn(U) -> bool) -> impl Iterator<Item = (T, U, V)>;
        fn fr(self, f: impl Fn(V) -> bool) -> impl Iterator<Item = (T, U, V)>;
    }

    impl<T: Copy, U: Copy, V: Copy, I: Iterator<Item = (T, U, V)>> FilterBy3<T, U, V> for I {
        fn fl(self, f: impl Fn(T) -> bool) -> impl Iterator<Item = (T, U, V)> {
            self.filter(move |(x, _, _)| f(*x))
        }

        fn fm(self, f: impl Fn(U) -> bool) -> impl Iterator<Item = (T, U, V)> {
            self.filter(move |(_, x, _)| f(*x))
        }
        fn fr(self, f: impl Fn(V) -> bool) -> impl Iterator<Item = (T, U, V)> {
            self.filter(move |(_, _, x)| f(*x))
        }
    }
    pub trait FilterBy<T, U>: Iterator {
        fn fl(self, f: impl Fn(T) -> bool) -> impl Iterator<Item = (T, U)>;
        fn fr(self, f: impl Fn(U) -> bool) -> impl Iterator<Item = (T, U)>;
    }

    impl<T: Copy, U: Copy, I: Iterator<Item = (T, U)>> FilterBy<T, U> for I {
        fn fl(self, f: impl Fn(T) -> bool) -> impl Iterator<Item = (T, U)> {
            self.filter(move |(x, _)| f(*x))
        }

        fn fr(self, f: impl Fn(U) -> bool) -> impl Iterator<Item = (T, U)> {
            self.filter(move |(_, x)| f(*x))
        }
    }

    pub trait NumTupleIterTools {
        fn πολλαπλασιάζω_και_αθροίζω(&mut self) -> u64;
    }

    impl<I: Iterator<Item = (u64, u64)>> NumTupleIterTools for I {
        fn πολλαπλασιάζω_και_αθροίζω(&mut self) -> u64 {
            self.map(|(a, b)| a * b).sum()
        }
    }

    impl<T, U, I: Iterator<Item = (T, U)>> TupleIterTools2<T, U> for I {
        fn l(self) -> impl Iterator<Item = T> {
            self.map(|(x, _)| x)
        }

        fn r(self) -> impl Iterator<Item = U> {
            self.map(|(_, x)| x)
        }
    }

    impl<'a, T: Copy + 'a, U: Copy + 'a, I: Iterator<Item = &'a (T, U)>> TupleIterTools2R<T, U> for I {
        fn l(self) -> impl Iterator<Item = T> {
            self.map(|&(x, _)| x)
        }
        fn r(self) -> impl Iterator<Item = U> {
            self.map(|&(_, x)| x)
        }
    }

    impl<T, U, V, I: Iterator<Item = (T, U, V)>> TupleIterTools3<T, U, V> for I {
        fn l(self) -> impl Iterator<Item = T> {
            self.map(|(x, _, _)| x)
        }

        fn m(self) -> impl Iterator<Item = U> {
            self.map(|(_, x, _)| x)
        }

        fn r(self) -> impl Iterator<Item = V> {
            self.map(|(_, _, x)| x)
        }

        fn lm(self) -> impl Iterator<Item = (T, U)> {
            self.map(|(a, b, _)| (a, b))
        }

        fn lr(self) -> impl Iterator<Item = (T, V)> {
            self.map(|(a, _, b)| (a, b))
        }

        fn mr(self) -> impl Iterator<Item = (U, V)> {
            self.map(|(_, a, b)| (a, b))
        }
    }

    pub trait GreekTools<T>: Iterator {
        fn Δ(&mut self) -> T;
        fn ι<N>(&mut self) -> impl Iterator<Item = (T, N)>
        where
            Self: Ι<T, N>;
        fn ι1<N>(&mut self) -> impl Iterator<Item = (T, N)>
        where
            Self: Ι<T, N>;
        fn ν<const N: usize>(&mut self, into: &mut [T; N]) -> usize;
        fn Θ(&mut self);
    }

    pub trait ParseIter {
        fn κ<T: FromStr>(&mut self) -> impl Iterator<Item = T>
        where
            <T as FromStr>::Err: std::fmt::Display;
    }

    impl<'x, I: Iterator<Item = &'x [u8]>> ParseIter for I {
        fn κ<T: FromStr>(&mut self) -> impl Iterator<Item = T>
        where
            <T as FromStr>::Err: std::fmt::Display,
        {
            self.flat_map(|x| x.κ())
        }
    }

    pub trait Ι<T, N>: Iterator {
        fn ι(&mut self) -> impl Iterator<Item = (T, N)>;
        fn ι1(&mut self) -> impl Iterator<Item = (T, N)>;
    }

    macro_rules! ι {
        ($t:ty) => {
            impl<T, I: Iterator<Item = T>> Ι<T, $t> for I {
                fn ι(&mut self) -> impl Iterator<Item = (T, $t)> {
                    self.zip(0..)
                }

                fn ι1(&mut self) -> impl Iterator<Item = (T, $t)> {
                    self.zip(1..)
                }
            }
        };
    }
    ι!(i8);
    ι!(u8);
    ι!(u16);
    ι!(u32);
    ι!(u64);
    ι!(usize);

    pub fn nail<const N: usize, T: Copy>(x: &[T]) -> [T; N] {
        unsafe { (x.as_ptr() as *const [T; N]).read() }
    }

    pub mod reading {
        #[inline]
        pub fn 八(n: u64) -> u64 {
            // reinterpret as u64 ("92233721" => 92233721)
            // let n = u64::from_le_bytes(s);
            // combine 4 pairs of single digits:
            // split pieces into odd and even
            //  1_7_3_2_ (le repr)
            // _2_3_2_9
            // then combine
            // _21_37_23_92 (le repr, each byte as 2 digits)
            let n = ((n & 0x0f000f000f000f00) >> 8) + ((n & 0x000f000f000f000f) * 10);
            // combine 2 pairs of 2 digits:
            // split again
            // _21___23__
            // ___37___92
            // then combine
            // __14|137__36|7 (le repr, pipes separating bytes)
            let n = ((n & 0x00ff000000ff0000) >> 16) + ((n & 0x000000ff000000ff) * 100);
            // combine pair of 4 digits
            // split again
            // __14|137____ (then moved to ______14|137, as u64:3721)
            // ______36|07 (as u64: 9223)
            // then combine
            ((n & 0x0000ffff00000000) >> 32) + ((n & 0x000000000000ffff) * 10000)
        }

        use std::{
            io::{self, Read},
            ops::{Add, BitOrAssign, Shl},
        };
        pub trait Ext {
            fn rd<const N: usize>(&mut self) -> io::Result<[u8; N]>;
            fn by(&mut self) -> io::Result<u8> {
                Ok(self.rd::<1>()?[0])
            }
        }

        impl<T: Read> Ext for T {
            fn rd<const N: usize>(&mut self) -> io::Result<[u8; N]> {
                let mut buf = [0; N];
                self.read_exact(&mut buf)?;
                Ok(buf)
            }
        }
        use crate::util::prelude::*;
        pub fn κ<
            T: Default
                + std::ops::Mul<T, Output = T>
                + Add<T, Output = T>
                + From<u8>
                + Copy
                + Ten
                + Debug,
        >(
            x: &[u8],
            v: &mut [T],
        ) -> usize {
            let mut n = 0;
            let mut s = T::default();
            for &b in x {
                match b {
                    b' ' => {
                        C! { v[n] = s };
                        n += 1;
                        s = T::default();
                    }
                    b => {
                        s = s * T::ten() + T::from(b - b'0');
                    }
                }
            }
            C! {v[n] = s};
            n + 1
        }
        pub trait Ten {
            fn ten() -> Self;
        }
        macro_rules! tenz {
            ($for:ty) => {
                impl Ten for $for {
                    fn ten() -> $for {
                        10
                    }
                }
            };
        }
        tenz!(u8);
        tenz!(u16);
        tenz!(u32);
        tenz!(u64);
        tenz!(u128);
        tenz!(i8);
        tenz!(i16);
        tenz!(i32);
        tenz!(i64);
        tenz!(i128);

        const DIG: [u8; 256] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 11, 12, 13, 14, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        pub fn hex_dig(b: u8) -> u8 {
            DIG[b.nat()]
            // (b & 0xF) + 9 * (b >> 6)
        }

        pub fn hexN<
            T: From<u8> + TryFrom<usize> + Shl<T, Output = T> + BitOrAssign<T>,
            const N: usize,
        >(
            a: [u8; N],
        ) -> T {
            let mut c = T::from(hex_dig(a[0])) << T::try_from((N - 1) * 4).ψ();
            for (&n, sh) in a[1..].iter().zip((0..N - 1).rev()) {
                c |= T::from(hex_dig(n)) << T::try_from(sh * 4).ψ();
            }
            c
        }

        pub fn hex(mut d: &[u8]) -> Result<u32, ()> {
            let &b = d.take_first().ok_or(())?;
            let mut num = hex_dig(b) as u32;
            while let Some(&b) = d.take_first() {
                num = num * 16 + hex_dig(b) as u32;
            }
            Ok(num)
        }

        pub fn 迄または完了<
            T: Default + std::ops::Mul<T, Output = T> + Add<T, Output = T> + From<u8> + Copy + Ten,
        >(
            x: &mut &[u8],
            until: u8,
        ) -> T {
            let mut n = T::default();
            while let Ok(x) = x.by() {
                if x == until {
                    return n;
                }
                n = n * T::ten() + T::from(x - b'0')
            }
            n
        }

        pub fn 負迄(x: &mut &[u8], until: u8) -> i64 {
            let (sign, mut n) = match x.by().ψ() {
                b'-' => (-1, 0),
                b => (1, i64::from(b - b'0')),
            };
            loop {
                let byte = x.by().ψ();
                if byte == until {
                    return n * sign as i64;
                }
                n = n * 10 + i64::from(byte - b'0');
            }
        }

        pub fn until<
            T: std::ops::Mul<T, Output = T> + Add<T, Output = T> + From<u8> + Copy + Ten,
        >(
            x: &mut &[u8],
            until: u8,
        ) -> T {
            let mut n = T::from(x.by().ψ() - b'0');
            loop {
                let byte = x.by().ψ();
                if byte == until {
                    return n;
                }
                n = n * T::ten() + T::from(byte - b'0');
            }
        }

        #[cfg_attr(debug_assertions, track_caller)]
        pub fn all<
            T: Default + std::ops::Mul<T, Output = T> + Add<T, Output = T> + From<u8> + Copy + Ten,
        >(
            x: &[u8],
        ) -> T {
            let mut n = T::default();
            for &byte in x {
                n = n * T::ten() + T::from(byte - b'0');
            }
            n
        }
    }

    pub fn even(x: &usize) -> bool {
        x % 2 == 0
    }

    impl<T, I: Iterator<Item = T>> GreekTools<T> for I {
        #[cfg_attr(debug_assertions, track_caller)]
        fn Δ(&mut self) -> T {
            self.next().ψ()
        }

        fn ν<const N: usize>(&mut self, into: &mut [T; N]) -> usize {
            let mut set = 0;
            for e in into {
                let Some(y) = self.next() else { break };
                *e = y;
                set += 1;
            }
            set
        }

        fn ι<N>(&mut self) -> impl Iterator<Item = (T, N)>
        where
            Self: Ι<T, N>,
        {
            self.ι()
        }

        fn ι1<N>(&mut self) -> impl Iterator<Item = (T, N)>
        where
            Self: Ι<T, N>,
        {
            self.ι1()
        }

        fn Θ(&mut self) {
            for _ in self {}
        }
    }

    pub trait TupleUtils<T, U> {
        fn mr<W>(self, f: impl FnOnce(U) -> W) -> (T, W);
        fn ml<V>(self, f: impl FnOnce(T) -> V) -> (V, U);
        fn rev(self) -> (U, T);
    }

    pub trait Widen<Wide> {
        fn nat(self) -> usize;
        fn widen(self) -> Wide;
    }

    macro_rules! wide {
        ($t:ty: $upper:ty) => {
            impl Widen<$upper> for $t {
                fn nat(self) -> usize {
                    self as _
                }

                fn widen(self) -> $upper {
                    self as _
                }
            }
        };
    }
    wide!(u8: u16);
    wide!(u16: u32);
    wide!(u32: u64);
    wide!(u64: u128);

    pub trait UnifiedTupleUtils<T> {
        fn mb<U>(self, f: impl FnMut(T) -> U) -> (U, U);
    }

    impl<T> UnifiedTupleUtils<T> for (T, T) {
        fn mb<U>(self, mut f: impl FnMut(T) -> U) -> (U, U) {
            (f(self.0), f(self.1))
        }
    }

    impl<T, U> TupleUtils<T, U> for (T, U) {
        fn mr<W>(self, f: impl FnOnce(U) -> W) -> (T, W) {
            (self.0, f(self.1))
        }
        fn ml<V>(self, f: impl FnOnce(T) -> V) -> (V, U) {
            (f(self.0), self.1)
        }
        fn rev(self) -> (U, T) {
            (self.1, self.0)
        }
    }

    #[allow(dead_code)]
    fn cast_to<T: From<bool>>(x: bool, _to: T) -> T {
        x.into()
    }

    #[allow(unused_macros)]
    macro_rules! bits {
        ($bitset:ident + $bit:expr) => {
            $bitset |= 1 << $bit
        };
        ($holder:ident[$index:expr] + $bit:expr) => {
            $holder[$index] |= 1 << $bit;
        };
        ($bitset:ident[$bit:expr]) => {
            ($bitset & 1 << $bit) != 0
        };
        ($holder:ident[$index:expr][$bit:expr]) => {
            ($holder[$index] & 1 << $bit) != 0
        };
        ($holder:ident[$index:expr][$index2:expr][$bit:expr]) => {
            ($holder[$index][$index2] & 1 << $bit) != 0
        };
        ($holder:ident[$index:expr][$index2:expr] + $bit:expr) => {
            $holder[$index][$index2] |= 1 << $bit
        };
        ($bitset:ident[$bit:expr] = $val:expr) => {
            $bitset = ($bitset & !(1 << $bit)) | (crate::util::cast_to($val, $bitset) << $bit)
        };
        ($bitset:ident - $bit:expr) => {
            $bitset &= !(1 << $bit)
        };
        ($bitset:ident ! $bit:expr) => {
            $bitset ^= 1 << $bit
        };
    }
    pub(crate) use bits;

    pub struct Lines<'a> {
        bytes: &'a [u8],
    }

    impl<'a> Iterator for Lines<'a> {
        type Item = &'a [u8];

        fn next(&mut self) -> Option<Self::Item> {
            self.bytes.take_line()
        }
    }

    impl<'a> std::iter::FusedIterator for Lines<'a> {}

    impl<'a> DoubleEndedIterator for Lines<'a> {
        #[inline]
        fn next_back(&mut self) -> Option<Self::Item> {
            self.bytes.take_backline()
        }
    }

    pub trait IntoLines {
        fn 行(&self) -> Lines<'_>;
    }

    impl<T: AsRef<[u8]>> IntoLines for T {
        fn 行(&self) -> Lines<'_> {
            Lines {
                bytes: self.as_ref(),
            }
        }
    }

    pub trait Printable {
        fn p(&self) -> impl std::fmt::Display;
    }

    struct PrintU8s<'a>(&'a [u8]);
    impl std::fmt::Display for PrintU8s<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for &b in self.0 {
                if b.is_ascii() {
                    f.write_char(b as char)?;
                } else {
                    write!(f, "\\x{b:x}")?;
                }
            }
            Ok(())
        }
    }

    struct PrintManyU8s<'a, T: AsRef<[u8]>>(&'a [T]);
    impl<T: AsRef<[u8]>> std::fmt::Display for PrintManyU8s<'_, T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for row in self.0.as_ref() {
                write!(f, "{},", row.as_ref().p())?;
            }
            Ok(())
        }
    }
    impl Printable for [Vec<u8>] {
        fn p(&self) -> impl std::fmt::Display {
            PrintManyU8s(self)
        }
    }

    impl Printable for [&&[u8]] {
        fn p(&self) -> impl std::fmt::Display {
            PrintManyU8s(self)
        }
    }

    impl Printable for [&[u8]] {
        fn p(&self) -> impl std::fmt::Display {
            PrintManyU8s(self)
        }
    }

    impl Printable for [u8] {
        fn p(&self) -> impl std::fmt::Display {
            PrintU8s(self)
        }
    }

    impl Printable for Vec<u8> {
        fn p(&self) -> impl std::fmt::Display {
            PrintU8s(self)
        }
    }

    pub fn sort<T: Ord>(mut x: Vec<T>) -> Vec<T> {
        x.sort_unstable();
        x
    }

    pub trait TakeLine<'b> {
        fn take_line<'a>(&'a mut self) -> Option<&'b [u8]>;
        fn take_backline<'a>(&'a mut self) -> Option<&'b [u8]>;
    }

    impl<'b> TakeLine<'b> for &'b [u8] {
        fn take_line<'a>(&'a mut self) -> Option<&'b [u8]> {
            match memchr::memchr(b'\n', self) {
                None if self.is_empty() => None,
                None => Some(std::mem::replace(self, b"")),
                Some(end) => {
                    let line = C! { &self[..end]};
                    *self = C! { &self[end + 1..]};
                    Some(line)
                }
            }
        }

        fn take_backline<'a>(&'a mut self) -> Option<&'b [u8]> {
            let end = self.len().checked_sub(1)?;
            match memchr::memrchr(b'\n', &self[..end]) {
                None => Some(std::mem::replace(self, b"")),
                Some(end) => {
                    let line = &self[end + 1..];
                    *self = &self[..end];
                    Some(line)
                }
            }
        }
    }

    impl<'b> TakeLine<'b> for &'b str {
        fn take_line<'a>(&'a mut self) -> Option<&'b [u8]> {
            match memchr::memchr(b'\n', self.as_bytes()) {
                None if self.is_empty() => None,
                None => Some(std::mem::replace(self, "").as_bytes()),
                Some(end) => {
                    let line = self[..end].as_bytes();
                    *self = &self[end + 1..];
                    Some(line)
                }
            }
        }

        fn take_backline<'a>(&'a mut self) -> Option<&'b [u8]> {
            let end = self.len().checked_sub(1)?;
            match memchr::memrchr(b'\n', &self.as_bytes()[..end]) {
                None => Some(std::mem::replace(self, "").as_bytes()),
                Some(end) => {
                    let line = &self[end + 1..];
                    *self = &self[..end];
                    Some(line.as_bytes())
                }
            }
        }
    }

    pub trait IntoCombinations<T: Copy>: Iterator {
        /// LEAKY
        fn combine(self) -> impl Iterator<Item = (T, T)>;
    }

    impl<T: Copy + 'static, I: Iterator<Item = T>> IntoCombinations<T> for I {
        fn combine(self) -> impl Iterator<Item = (T, T)> {
            let x = Box::leak(self.collect::<Box<[_]>>());
            x.iter()
                .enumerate()
                .flat_map(|(i, &a)| x[i..].iter().map(move |&b| (a, b)))
        }
    }

    pub trait Skip {
        fn skip(&mut self, n: usize);
        fn skip_n(&mut self, n: &'static str) {
            self.skip(n.len())
        }
    }

    impl<T> Skip for &[T] {
        #[cfg_attr(debug_assertions, track_caller)]
        fn skip(&mut self, n: usize) {
            if cfg!(debug_assertions) {
                *self = &self[n..];
            } else {
                *self = C! { &self[n..] };
            }
        }
    }

    impl Skip for &str {
        #[cfg_attr(debug_assertions, track_caller)]
        fn skip(&mut self, n: usize) {
            if cfg!(debug_assertions) {
                *self = &self[n..];
            } else {
                *self = C! { &self[n..] };
            }
        }
    }

    /// WYRAND based rng's
    pub mod rand {
        /// WYRAND
        pub fn u64() -> u64 {
            static mut STATE: u64 = 0;
            let tmp = unsafe {
                STATE = STATE.wrapping_add(0x60bee2bee120fc15);
                (STATE as u128).wrapping_mul(0xa3b195354a39b70d)
            };
            let m1 = (tmp >> 64) ^ tmp;
            let tmp = m1.wrapping_mul(0x1b03738712fad5c9);
            ((tmp >> 64) ^ tmp) as u64
        }

        /// 0..N
        pub mod limit {
            use crate::Widen;

            pub fn u64(of: u64) -> u64 {
                ((super::u64().widen().wrapping_mul(of.widen())) >> 64) as u64
            }
        }

        pub fn u32() -> u32 {
            u64() as u32
        }

        pub fn u16() -> u16 {
            u64() as u16
        }

        pub fn f32() -> f32 {
            (1.0 / ((1u32 << 24) as f32)) * ((u32() >> 8) as f32)
        }

        pub fn f64() -> f64 {
            (1.0 / ((1u64 << 53) as f64)) * ((u64() >> 11) as f64)
        }
    }
}

use util::prelude::*;

static P1: [u64; 3750202] = {
    let mut l = [0; 3750202];
    {
        l[3223600] = 66;
        l[3289136] = 96;
        l[3354672] = 162;
        l[3420208] = 272;
        l[3485744] = 250;
        l[3551280] = 336;
        l[3616816] = 490;
        l[3682352] = 416;
        l[3747888] = 522;
        l[3158320] = 740;
        l[3223856] = 726;
        l[3289392] = 840;
        l[3354928] = 910;
        l[3420464] = 1092;
        l[3486000] = 1200;
        l[3551536] = 1280;
        l[3617072] = 1360;
        l[3682608] = 1476;
        l[3748144] = 1558;
        l[3158576] = 1120;
        l[3224112] = 1470;
        l[3289648] = 1056;
        l[3355184] = 1288;
        l[3420720] = 1776;
        l[3486256] = 1500;
        l[3551792] = 1716;
        l[3617328] = 2052;
        l[3682864] = 1736;
        l[3748400] = 1972;
        l[3158832] = 2040;
        l[3224368] = 2418;
        l[3289904] = 2304;
        l[3355440] = 1782;
        l[3420976] = 2788;
        l[3486512] = 2660;
        l[3552048] = 2376;
        l[3617584] = 3108;
        l[3683120] = 2964;
        l[3748656] = 2652;
        l[3159088] = 3040;
        l[3224624] = 3362;
        l[3290160] = 3276;
        l[3355696] = 3354;
        l[3421232] = 2992;
        l[3486768] = 3240;
        l[3552304] = 3312;
        l[3617840] = 3760;
        l[3683376] = 3936;
        l[3748912] = 4018;
        l[3159344] = 2900;
        l[3224880] = 3774;
        l[3290416] = 3328;
        l[3355952] = 3392;
        l[3421488] = 3888;
        l[3487024] = 2750;
        l[3552560] = 3248;
        l[3618096] = 4332;
        l[3683632] = 3596;
        l[3749168] = 4012;
        l[3159600] = 4200;
        l[3225136] = 5002;
        l[3290672] = 4712;
        l[3356208] = 4410;
        l[3421744] = 5120;
        l[3487280] = 4810;
        l[3552816] = 3696;
        l[3618352] = 5628;
        l[3683888] = 5304;
        l[3749424] = 4692;
        l[3159856] = 5460;
        l[3225392] = 5964;
        l[3290928] = 5760;
        l[3356464] = 5840;
        l[3422000] = 6216;
        l[3487536] = 6000;
        l[3553072] = 6080;
        l[3618608] = 5390;
        l[3684144] = 5772;
        l[3749680] = 5846;
        l[3160112] = 4800;
        l[3225648] = 6156;
        l[3291184] = 5412;
        l[3356720] = 5478;
        l[3422256] = 6384;
        l[3487792] = 5610;
        l[3553328] = 5676;
        l[3618864] = 6438;
        l[3684400] = 4576;
        l[3749936] = 5340;
        l[3160368] = 6480;
        l[3225904] = 7644;
        l[3291440] = 7176;
        l[3356976] = 6696;
        l[3422512] = 7896;
        l[3488048] = 7410;
        l[3553584] = 6912;
        l[3619120] = 7954;
        l[3684656] = 7448;
        l[3750192] = 5742;
        l[3158065] = 5800;
        l[3223601] = 9494;
        l[3289137] = 7752;
        l[3354673] = 8446;
        l[3420209] = 9984;
        l[3485745] = 8190;
        l[3551281] = 8904;
        l[3616817] = 10486;
        l[3682353] = 8640;
        l[3747889] = 9374;
        l[3158321] = 6380;
        l[3223857] = 5550;
        l[3289393] = 6048;
        l[3354929] = 6102;
        l[3420465] = 7068;
        l[3486001] = 7360;
        l[3551537] = 7424;
        l[3617073] = 7488;
        l[3682609] = 7788;
        l[3748145] = 7854;
        l[3158577] = 7440;
        l[3224113] = 9196;
        l[3289649] = 6588;
        l[3355185] = 7626;
        l[3420721] = 9920;
        l[3486257] = 8250;
        l[3551793] = 9072;
        l[3617329] = 10414;
        l[3682865] = 8704;
        l[3748401] = 9546;
        l[3158833] = 8840;
        l[3224369] = 10218;
        l[3289905] = 9504;
        l[3355441] = 7182;
        l[3420977] = 10988;
        l[3486513] = 10260;
        l[3552049] = 8976;
        l[3617585] = 11508;
        l[3683121] = 10764;
        l[3748657] = 9452;
        l[3159089] = 9800;
        l[3224625] = 10716;
        l[3290161] = 10224;
        l[3355697] = 10296;
        l[3421233] = 8928;
        l[3486769] = 9570;
        l[3552305] = 9636;
        l[3617841] = 10878;
        l[3683377] = 11248;
        l[3748913] = 11324;
        l[3159345] = 10800;
        l[3224881] = 13288;
        l[3290417] = 11856;
        l[3355953] = 11934;
        l[3421489] = 13244;
        l[3487025] = 9920;
        l[3552561] = 11232;
        l[3618097] = 14130;
        l[3683633] = 12008;
        l[3749169] = 13038;
        l[3159601] = 12480;
        l[3225137] = 14490;
        l[3290673] = 13608;
        l[3356209] = 12714;
        l[3421745] = 14432;
        l[3487281] = 13530;
        l[3552817] = 10624;
        l[3618353] = 15364;
        l[3683889] = 14448;
        l[3749425] = 12844;
        l[3159857] = 12240;
        l[3225393] = 13338;
        l[3290929] = 12728;
        l[3356465] = 12802;
        l[3422001] = 13572;
        l[3487537] = 12950;
        l[3553073] = 13024;
        l[3618609] = 11328;
        l[3684145] = 12104;
        l[3749681] = 12172;
        l[3160113] = 13320;
        l[3225649] = 16290;
        l[3291185] = 14560;
        l[3356721] = 14640;
        l[3422257] = 16560;
        l[3487793] = 14800;
        l[3553329] = 14880;
        l[3618865] = 16456;
        l[3684401] = 12408;
        l[3749937] = 13986;
        l[3160369] = 15200;
        l[3225905] = 17572;
        l[3291441] = 16512;
        l[3356977] = 15440;
        l[3422513] = 17848;
        l[3488049] = 16770;
        l[3553585] = 15680;
        l[3619121] = 17730;
        l[3684657] = 16632;
        l[3750193] = 13134;
        l[3158066] = 9600;
        l[3223602] = 16884;
        l[3289138] = 13332;
        l[3354674] = 14616;
        l[3420210] = 17544;
        l[3485746] = 13940;
        l[3551282] = 15244;
        l[3616818] = 18216;
        l[3682354] = 14560;
        l[3747890] = 15884;
        l[3158322] = 14700;
        l[3223858] = 13082;
        l[3289394] = 13992;
        l[3354930] = 14058;
        l[3420466] = 15836;
        l[3486002] = 16340;
        l[3551538] = 16416;
        l[3617074] = 16492;
        l[3682610] = 17004;
        l[3748146] = 17082;
        l[3158578] = 10560;
        l[3224114] = 13702;
        l[3289650] = 8880;
        l[3355186] = 10704;
        l[3420722] = 14784;
        l[3486258] = 11700;
        l[3551794] = 13108;
        l[3617330] = 15436;
        l[3682866] = 12312;
        l[3748402] = 13740;
        l[3158834] = 14260;
        l[3224370] = 16632;
        l[3289906] = 15312;
        l[3355442] = 11184;
        l[3420978] = 17784;
        l[3486514] = 16450;
        l[3552050] = 14160;
        l[3617586] = 18486;
        l[3683122] = 17136;
        l[3748658] = 14818;
        l[3159090] = 17760;
        l[3224626] = 19280;
        l[3290162] = 18392;
        l[3355698] = 18468;
        l[3421234] = 16104;
        l[3486770] = 17150;
        l[3552306] = 17220;
        l[3617842] = 19266;
        l[3683378] = 19840;
        l[3748914] = 19920;
        l[3159346] = 15000;
        l[3224882] = 19076;
        l[3290418] = 16632;
        l[3355954] = 16698;
        l[3421490] = 18796;
        l[3487026] = 13260;
        l[3552562] = 15360;
        l[3618098] = 20046;
        l[3683634] = 16512;
        l[3749170] = 18130;
        l[3159602] = 18720;
        l[3225138] = 21924;
        l[3290674] = 20436;
        l[3356210] = 18936;
        l[3421746] = 21648;
        l[3487282] = 20140;
        l[3552818] = 15428;
        l[3618354] = 22962;
        l[3683890] = 21440;
        l[3749426] = 18830;
        l[3159858] = 20520;
        l[3225394] = 22222;
        l[3290930] = 21216;
        l[3356466] = 21294;
        l[3422002] = 22468;
        l[3487538] = 21450;
        l[3553074] = 21528;
        l[3618610] = 18836;
        l[3684146] = 20016;
        l[3749682] = 20088;
        l[3160114] = 17360;
        l[3225650] = 21918;
        l[3291186] = 19176;
        l[3356722] = 19244;
        l[3422258] = 22152;
        l[3487794] = 19380;
        l[3553330] = 19448;
        l[3618866] = 21812;
        l[3684402] = 15552;
        l[3749938] = 17918;
        l[3160370] = 21460;
        l[3225906] = 25026;
        l[3291442] = 23360;
        l[3356978] = 21682;
        l[3422514] = 25284;
        l[3488050] = 23600;
        l[3553586] = 21904;
        l[3619122] = 24948;
        l[3684658] = 23244;
        l[3750194] = 17940;
        l[3158067] = 13200;
        l[3223603] = 24080;
        l[3289139] = 18724;
        l[3354675] = 20604;
        l[3420211] = 24928;
        l[3485747] = 19520;
        l[3551283] = 21420;
        l[3616819] = 25788;
        l[3682355] = 20328;
        l[3747891] = 22248;
        l[3158323] = 19220;
        l[3223859] = 16794;
        l[3289395] = 18096;
        l[3354931] = 18154;
        l[3420467] = 20724;
        l[3486003] = 21420;
        l[3551539] = 21488;
        l[3617075] = 21556;
        l[3682611] = 22260;
        l[3748147] = 22330;
        l[3158579] = 17920;
        l[3224115] = 22470;
        l[3289651] = 15456;
        l[3355187] = 18088;
        l[3420723] = 23976;
        l[3486259] = 19500;
        l[3551795] = 21516;
        l[3617331] = 24852;
        l[3682867] = 20336;
        l[3748403] = 22372;
        l[3158835] = 14520;
        l[3224371] = 17874;
        l[3289907] = 15936;
        l[3355443] = 9990;
        l[3420979] = 19372;
        l[3486515] = 17420;
        l[3552051] = 14112;
        l[3617587] = 20220;
        l[3683123] = 18252;
        l[3748659] = 14916;
        l[3159091] = 22440;
        l[3224627] = 24552;
        l[3290163] = 23256;
        l[3355699] = 23324;
        l[3421235] = 19952;
        l[3486771] = 21390;
        l[3552307] = 21452;
        l[3617843] = 24290;
        l[3683379] = 25056;
        l[3748915] = 25128;
        l[3159347] = 21000;
        l[3224883] = 26676;
        l[3290419] = 23232;
        l[3355955] = 23298;
        l[3421491] = 26196;
        l[3487027] = 18460;
        l[3552563] = 21360;
        l[3618099] = 27846;
        l[3683635] = 22912;
        l[3749171] = 25130;
        l[3159603] = 20160;
        l[3225139] = 24548;
        l[3290675] = 22444;
        l[3356211] = 20328;
        l[3421747] = 24024;
        l[3487283] = 21900;
        l[3552819] = 15372;
        l[3618355] = 25690;
        l[3683891] = 23552;
        l[3749427] = 19926;
        l[3159859] = 25160;
        l[3225395] = 27454;
        l[3290931] = 26040;
        l[3356467] = 26110;
        l[3422003] = 27676;
        l[3487539] = 26250;
        l[3553075] = 26320;
        l[3618611] = 22620;
        l[3684147] = 24192;
        l[3749683] = 24256;
        l[3160115] = 23560;
        l[3225651] = 29718;
        l[3291187] = 25976;
        l[3356723] = 26044;
        l[3422259] = 29952;
        l[3487795] = 26180;
        l[3553331] = 26248;
        l[3618867] = 29412;
        l[3684403] = 20952;
        l[3749939] = 24118;
        l[3160371] = 22620;
        l[3225907] = 27370;
        l[3291443] = 25088;
        l[3356979] = 22794;
        l[3422515] = 27580;
        l[3488051] = 25280;
        l[3553587] = 22968;
        l[3619123] = 26996;
        l[3684659] = 24676;
        l[3750195] = 17556;
        l[3158068] = 24000;
        l[3223604] = 38496;
        l[3289140] = 31356;
        l[3354676] = 33852;
        l[3420212] = 39592;
        l[3485748] = 32400;
        l[3551284] = 34916;
        l[3616820] = 40700;
        l[3682356] = 33456;
        l[3747892] = 35992;
        l[3158324] = 30340;
        l[3223860] = 27126;
        l[3289396] = 28840;
        l[3354932] = 28910;
        l[3420468] = 32292;
        l[3486004] = 33200;
        l[3551540] = 33280;
        l[3617076] = 33360;
        l[3682612] = 34276;
        l[3748148] = 34358;
        l[3158580] = 29400;
        l[3224116] = 35364;
        l[3289652] = 26164;
        l[3355188] = 29610;
        l[3420724] = 37312;
        l[3486260] = 31450;
        l[3551796] = 34080;
        l[3617332] = 38430;
        l[3682868] = 32528;
        l[3748404] = 35178;
        l[3158836] = 32680;
        l[3224372] = 37066;
        l[3289908] = 34560;
        l[3355444] = 26846;
        l[3420980] = 39060;
        l[3486516] = 36540;
        l[3552052] = 32264;
        l[3617588] = 40204;
        l[3683124] = 37668;
        l[3748660] = 33364;
        l[3159092] = 26400;
        l[3224628] = 29106;
        l[3290164] = 27404;
        l[3355700] = 27466;
        l[3421236] = 23088;
        l[3486772] = 24920;
        l[3552308] = 24976;
        l[3617844] = 28608;
        l[3683380] = 29568;
        l[3748916] = 29634;
        l[3159348] = 28800;
        l[3224884] = 36080;
        l[3290420] = 31640;
        l[3355956] = 31710;
        l[3421492] = 35412;
        l[3487028] = 25480;
        l[3552564] = 29184;
        l[3618100] = 37474;
        l[3683636] = 31144;
        l[3749172] = 33966;
        l[3159604] = 32200;
        l[3225140] = 37802;
        l[3290676] = 35112;
        l[3356212] = 32410;
        l[3421748] = 37120;
        l[3487284] = 34410;
        l[3552820] = 26096;
        l[3618356] = 39228;
        l[3683892] = 36504;
        l[3749428] = 31892;
        l[3159860] = 33840;
        l[3225396] = 36738;
        l[3290932] = 34928;
        l[3356468] = 35002;
        l[3422004] = 36972;
        l[3487540] = 35150;
        l[3553076] = 35224;
        l[3618612] = 30528;
        l[3684148] = 32504;
        l[3749684] = 32572;
        l[3160116] = 35520;
        l[3225652] = 43290;
        l[3291188] = 38560;
        l[3356724] = 38640;
        l[3422260] = 43560;
        l[3487796] = 38800;
        l[3553332] = 38880;
        l[3618868] = 42856;
        l[3684404] = 32208;
        l[3749940] = 36186;
        l[3160372] = 39200;
        l[3225908] = 45172;
        l[3291444] = 42312;
        l[3356980] = 39440;
        l[3422516] = 45448;
        l[3488052] = 42570;
        l[3553588] = 39680;
        l[3619124] = 44730;
        l[3684660] = 41832;
        l[3750196] = 32934;
        l[3158069] = 25000;
        l[3223605] = 43086;
        l[3289141] = 34136;
        l[3354677] = 37222;
        l[3420213] = 44352;
        l[3485749] = 35350;
        l[3551285] = 38456;
        l[3616821] = 45630;
        l[3682357] = 36576;
        l[3747893] = 39702;
        l[3158325] = 37740;
        l[3223861] = 33726;
        l[3289397] = 35840;
        l[3354933] = 35910;
        l[3420469] = 40092;
        l[3486005] = 41200;
        l[3551541] = 41280;
        l[3617077] = 41360;
        l[3682613] = 42476;
        l[3748149] = 42558;
        l[3158581] = 33280;
        l[3224117] = 40638;
        l[3289653] = 29232;
        l[3355189] = 33472;
        l[3420725] = 42968;
        l[3486261] = 35700;
        l[3551797] = 38924;
        l[3617333] = 44268;
        l[3682869] = 36960;
        l[3748405] = 40204;
        l[3158837] = 37100;
        l[3224373] = 42480;
        l[3289909] = 39368;
        l[3355445] = 29848;
        l[3420981] = 44856;
        l[3486517] = 41730;
        l[3552053] = 36448;
        l[3617589] = 46182;
        l[3683125] = 43040;
        l[3748661] = 37730;
        l[3159093] = 38880;
        l[3224629] = 42198;
        l[3290165] = 40108;
        l[3355701] = 40182;
        l[3421237] = 34816;
        l[3486773] = 37060;
        l[3552309] = 37128;
        l[3617845] = 41572;
        l[3683381] = 42744;
        l[3748917] = 42822;
        l[3159349] = 27500;
        l[3224885] = 36366;
        l[3290421] = 30912;
        l[3355957] = 30968;
        l[3421493] = 35456;
        l[3487029] = 23310;
        l[3552565] = 27800;
        l[3618101] = 37876;
        l[3683637] = 30132;
        l[3749173] = 33540;
        l[3159605] = 35840;
        l[3225141] = 42636;
        l[3290677] = 39340;
        l[3356213] = 36032;
        l[3421749] = 41736;
        l[3487285] = 38420;
        l[3552821] = 28300;
        l[3618357] = 44226;
        l[3683893] = 40896;
        l[3749429] = 35278;
        l[3159861] = 43320;
        l[3225397] = 46822;
        l[3290933] = 44616;
        l[3356469] = 44694;
        l[3422005] = 47068;
        l[3487541] = 44850;
        l[3553077] = 44928;
        l[3618613] = 39236;
        l[3684149] = 41616;
        l[3749685] = 41688;
        l[3160117] = 35960;
        l[3225653] = 45318;
        l[3291189] = 39576;
        l[3356725] = 39644;
        l[3422261] = 45552;
        l[3487797] = 39780;
        l[3553333] = 39848;
        l[3618869] = 44612;
        l[3684405] = 31752;
        l[3749941] = 36518;
        l[3160373] = 43660;
        l[3225909] = 50826;
        l[3291445] = 47360;
        l[3356981] = 43882;
        l[3422517] = 51084;
        l[3488053] = 47600;
        l[3553589] = 44104;
        l[3619125] = 50148;
        l[3684661] = 46644;
        l[3750197] = 35940;
        l[3158070] = 27600;
        l[3223606] = 49282;
        l[3289142] = 38528;
        l[3354678] = 42210;
        l[3420214] = 50736;
        l[3485750] = 39930;
        l[3551286] = 43632;
        l[3616822] = 52202;
        l[3682358] = 41344;
        l[3747894] = 45066;
        l[3158326] = 40260;
        l[3223862] = 35438;
        l[3289398] = 37944;
        l[3354934] = 38006;
        l[3420470] = 42980;
        l[3486006] = 44280;
        l[3551542] = 44352;
        l[3617078] = 44424;
        l[3682614] = 45732;
        l[3748150] = 45806;
        l[3158582] = 37200;
        l[3224118] = 45954;
        l[3289654] = 32344;
        l[3355190] = 37380;
        l[3420726] = 48672;
        l[3486262] = 40000;
        l[3551798] = 43820;
        l[3617334] = 50160;
        l[3682870] = 41448;
        l[3748406] = 45288;
        l[3158838] = 37800;
        l[3224374] = 44170;
        l[3289910] = 40448;
        l[3355446] = 29118;
        l[3420982] = 46916;
        l[3486518] = 43180;
        l[3552054] = 36888;
        l[3617590] = 48412;
        l[3683126] = 44660;
        l[3748662] = 38340;
        l[3159094] = 40960;
        l[3224630] = 44870;
        l[3290166] = 42372;
        l[3355702] = 42438;
        l[3421238] = 36064;
        l[3486774] = 38700;
        l[3552310] = 38760;
        l[3617846] = 43996;
        l[3683382] = 45360;
        l[3748918] = 45430;
        l[3159350] = 37700;
        l[3224886] = 48174;
        l[3290422] = 41728;
        l[3355958] = 41792;
        l[3421494] = 47088;
        l[3487030] = 32750;
        l[3552566] = 38048;
        l[3618102] = 49932;
        l[3683638] = 40796;
        l[3749174] = 44812;
        l[3159606] = 30360;
        l[3225142] = 38338;
        l[3290678] = 34424;
        l[3356214] = 30498;
        l[3421750] = 37184;
        l[3487286] = 33250;
        l[3552822] = 21312;
        l[3618358] = 40020;
        l[3683894] = 36072;
        l[3749430] = 29436;
        l[3159862] = 45560;
        l[3225398] = 49654;
        l[3290934] = 47040;
        l[3356470] = 47110;
        l[3422006] = 49876;
        l[3487542] = 47250;
        l[3553078] = 47320;
        l[3618614] = 40620;
        l[3684150] = 43392;
        l[3749686] = 43456;
        l[3160118] = 42160;
        l[3225654] = 53118;
        l[3291190] = 46376;
        l[3356726] = 46444;
        l[3422262] = 53352;
        l[3487798] = 46580;
        l[3553334] = 46648;
        l[3618870] = 52212;
        l[3684406] = 37152;
        l[3749942] = 42718;
        l[3160374] = 40020;
        l[3225910] = 48370;
        l[3291446] = 44288;
        l[3356982] = 40194;
        l[3422518] = 48580;
        l[3488054] = 44480;
        l[3553590] = 40368;
        l[3619126] = 47396;
        l[3684662] = 43276;
        l[3750198] = 30756;
        l[3158071] = 43400;
        l[3223607] = 68698;
        l[3289143] = 56160;
        l[3354679] = 60458;
        l[3420215] = 70400;
        l[3485751] = 57810;
        l[3551287] = 62128;
        l[3616823] = 72114;
        l[3682359] = 59472;
        l[3747895] = 63810;
        l[3158327] = 53960;
        l[3223863] = 48348;
        l[3289399] = 51264;
        l[3354935] = 51336;
        l[3420471] = 57120;
        l[3486007] = 58630;
        l[3551543] = 58712;
        l[3617079] = 58794;
        l[3682615] = 60312;
        l[3748151] = 60396;
        l[3158583] = 51840;
        l[3224119] = 62006;
        l[3289655] = 46208;
        l[3355191] = 52056;
        l[3420727] = 65160;
        l[3486263] = 55100;
        l[3551799] = 59532;
        l[3617335] = 66884;
        l[3682871] = 56784;
        l[3748407] = 61236;
        l[3158839] = 56940;
        l[3224375] = 64328;
        l[3289911] = 60024;
        l[3355447] = 46912;
        l[3420983] = 67528;
        l[3486519] = 63210;
        l[3552055] = 55936;
        l[3617591] = 69278;
        l[3683127] = 64944;
        l[3748663] = 57642;
        l[3159095] = 56240;
        l[3224631] = 60762;
        l[3290167] = 57876;
        l[3355703] = 57954;
        l[3421239] = 50592;
        l[3486775] = 53640;
        l[3552311] = 53712;
        l[3617847] = 59760;
        l[3683383] = 61336;
        l[3748919] = 61418;
        l[3159351] = 54000;
        l[3224887] = 66088;
        l[3290423] = 58656;
        l[3355959] = 58734;
        l[3421495] = 64844;
        l[3487031] = 48320;
        l[3552567] = 54432;
        l[3618103] = 68130;
        l[3683639] = 57608;
        l[3749175] = 62238;
        l[3159607] = 59280;
        l[3225143] = 68490;
        l[3290679] = 64008;
        l[3356215] = 59514;
        l[3421751] = 67232;
        l[3487287] = 62730;
        l[3552823] = 49024;
        l[3618359] = 70564;
        l[3683895] = 66048;
        l[3749431] = 58444;
        l[3159863] = 47740;
        l[3225399] = 52428;
        l[3290935] = 49408;
        l[3356471] = 49472;
        l[3422007] = 52632;
        l[3487543] = 49600;
        l[3553079] = 49664;
        l[3618615] = 41958;
        l[3684151] = 45124;
        l[3749687] = 45182;
        l[3160119] = 51480;
        l[3225655] = 64042;
        l[3291191] = 56304;
        l[3356727] = 56376;
        l[3422263] = 64288;
        l[3487799] = 56520;
        l[3553335] = 56592;
        l[3618871] = 62960;
        l[3684407] = 45704;
        l[3749943] = 52074;
        l[3160375] = 56880;
        l[3225911] = 66444;
        l[3291447] = 61776;
        l[3356983] = 57096;
        l[3422519] = 66696;
        l[3488055] = 62010;
        l[3553591] = 57312;
        l[3619127] = 65354;
        l[3684663] = 60648;
        l[3750199] = 46342;
        l[3158072] = 41600;
        l[3223608] = 70488;
        l[3289144] = 56140;
        l[3354680] = 61028;
        l[3420216] = 72360;
        l[3485752] = 57960;
        l[3551288] = 62868;
        l[3616824] = 74244;
        l[3682360] = 59792;
        l[3747896] = 64720;
        l[3158328] = 61560;
        l[3223864] = 55148;
        l[3289400] = 58464;
        l[3354936] = 58536;
        l[3420472] = 65120;
        l[3486008] = 66830;
        l[3551544] = 66912;
        l[3617080] = 66994;
        l[3682616] = 68712;
        l[3748152] = 68796;
        l[3158584] = 54120;
        l[3224120] = 65680;
        l[3289656] = 47676;
        l[3355192] = 54318;
        l[3420728] = 69216;
        l[3486264] = 57750;
        l[3551800] = 62776;
        l[3617336] = 71122;
        l[3682872] = 59616;
        l[3748408] = 64662;
        l[3158840] = 59760;
        l[3224376] = 68142;
        l[3289912] = 63232;
        l[3355448] = 48314;
        l[3420984] = 71724;
        l[3486520] = 66800;
        l[3552056] = 58520;
        l[3617592] = 73656;
        l[3683128] = 68716;
        l[3748664] = 60408;
        l[3159096] = 63840;
        l[3224632] = 68962;
        l[3290168] = 65676;
        l[3355704] = 65754;
        l[3421240] = 57392;
        l[3486776] = 60840;
        l[3552312] = 60912;
        l[3617848] = 67760;
        l[3683384] = 69536;
        l[3748920] = 69618;
        l[3159352] = 56100;
        l[3224888] = 69782;
        l[3290424] = 61344;
        l[3355960] = 61416;
        l[3421496] = 68320;
        l[3487032] = 49590;
        l[3552568] = 56496;
        l[3618104] = 71988;
        l[3683640] = 60060;
        l[3749176] = 65284;
        l[3159608] = 61920;
        l[3225144] = 72324;
        l[3290680] = 67236;
        l[3356216] = 62136;
        l[3421752] = 70848;
        l[3487288] = 65740;
        l[3552824] = 50228;
        l[3618360] = 74562;
        l[3683896] = 69440;
        l[3749432] = 60830;
        l[3159864] = 64380;
        l[3225400] = 69680;
        l[3290936] = 66272;
        l[3356472] = 66348;
        l[3422008] = 69920;
        l[3487544] = 66500;
        l[3553080] = 66576;
        l[3618616] = 57882;
        l[3684152] = 61460;
        l[3749688] = 61530;
        l[3160120] = 45760;
        l[3225656] = 59908;
        l[3291192] = 51156;
        l[3356728] = 51214;
        l[3422264] = 60112;
        l[3487800] = 51330;
        l[3553336] = 51388;
        l[3618872] = 58542;
        l[3684408] = 39072;
        l[3749944] = 46228;
        l[3160376] = 58740;
        l[3225912] = 69498;
        l[3291448] = 64224;
        l[3356984] = 58938;
        l[3422520] = 69732;
        l[3488056] = 64440;
        l[3553592] = 59136;
        l[3619128] = 68172;
        l[3684664] = 62860;
        l[3750200] = 46748;
        l[3158073] = 43200;
        l[3223609] = 75684;
        l[3289145] = 59532;
        l[3354681] = 65016;
        l[3420217] = 77744;
        l[3485753] = 61540;
        l[3551289] = 67044;
        l[3616825] = 79816;
        l[3682361] = 63560;
        l[3747897] = 69084;
        l[3158329] = 61880;
        l[3223865] = 54660;
        l[3289401] = 58368;
        l[3354937] = 58432;
        l[3420473] = 65808;
        l[3486009] = 67710;
        l[3551545] = 67784;
        l[3617081] = 67858;
        l[3682617] = 69768;
        l[3748153] = 69844;
        l[3158585] = 57040;
        l[3224121] = 69996;
        l[3289657] = 49788;
        l[3355193] = 57226;
        l[3420729] = 73920;
        l[3486265] = 61050;
        l[3551801] = 66672;
        l[3617337] = 76014;
        l[3682873] = 63104;
        l[3748409] = 68746;
        l[3158841] = 57660;
        l[3224377] = 67032;
        l[3289913] = 61512;
        l[3355449] = 44784;
        l[3420985] = 70984;
        l[3486521] = 65450;
        l[3552057] = 56160;
        l[3617593] = 73086;
        l[3683129] = 67536;
        l[3748665] = 58218;
        l[3159097] = 63920;
        l[3224633] = 69634;
        l[3290169] = 65940;
        l[3355705] = 66010;
        l[3421241] = 56640;
        l[3486777] = 60480;
        l[3552313] = 60544;
        l[3617849] = 68184;
        l[3683385] = 70152;
        l[3748921] = 70226;
        l[3159353] = 58900;
        l[3224889] = 74178;
        l[3290425] = 64736;
        l[3355961] = 64804;
        l[3421497] = 72504;
        l[3487033] = 51570;
        l[3552569] = 59272;
        l[3618105] = 76560;
        l[3683641] = 63228;
        l[3749177] = 69048;
        l[3159609] = 59520;
        l[3225145] = 71114;
        l[3290681] = 65416;
        l[3356217] = 59706;
        l[3421753] = 69408;
        l[3487289] = 63690;
        l[3552825] = 46368;
        l[3618361] = 73492;
        l[3683897] = 67760;
        l[3749433] = 58140;
        l[3159865] = 64020;
        l[3225401] = 69912;
        l[3290937] = 66096;
        l[3356473] = 66164;
        l[3422009] = 70128;
        l[3487545] = 66300;
        l[3553081] = 66368;
        l[3618617] = 56666;
        l[3684153] = 60636;
        l[3749689] = 60698;
        l[3160121] = 58800;
        l[3225657] = 74556;
        l[3291193] = 64812;
        l[3356729] = 64878;
        l[3422265] = 74784;
        l[3487801] = 65010;
        l[3553337] = 65076;
        l[3618873] = 73038;
        l[3684409] = 51376;
        l[3749945] = 59340;
        l[3160377] = 47520;
        l[3225913] = 59460;
        l[3291449] = 53568;
        l[3356985] = 47664;
        l[3422521] = 59640;
        l[3488057] = 53730;
        l[3553593] = 47808;
        l[3619129] = 57826;
        l[3684665] = 51896;
        l[3750201] = 33966;
    }

    l
};
#[no_mangle]
pub fn run(x: &str) -> impl Display {
    let i = x.as_bytes();
    let codes: &[[u8; 5]; 5] = unsafe { i.as_chunks_unchecked::<5>().try_into().ψ() };
    /*
    for code in 1..1000 {
        let code_ = format!("{code:03}A");
        let code__ = code_.as_bytes();
        let length = num(code_.as_bytes()[..4].try_into().unwrap())
            .into_iter()
            .map(|x| solve(x, 0))
            .min()
            .unwrap() as u64;
        print!(
            "l[{}]={};",
            u32::from_le_bytes(code__.try_into().unwrap()) & 16777215,
            length * code as u64
        );
    }
    return 0;
    */

    codes
        .into_iter()
        .map(|x| C! { P1[u32::from_le_bytes(x[..4].try_into().ψ()) as usize & 0xffffff] })
        .sum::<u64>()
}
