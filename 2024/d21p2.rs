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
        l[0x313030] = 0x12d50fe222;
        l[0x323030] = 0x1be99d58d8;
        l[0x333030] = 0x2f01b8a0a0;
        l[0x343030] = 0x4b543f8890;
        l[0x353030] = 0x45c8095e26;
        l[0x363030] = 0x5e0371414c;
        l[0x373030] = 0x83d36f2f0a;
        l[0x383030] = 0x6fa6756380;
        l[0x393030] = 0x8d0529e204;
        l[0x303130] = 0xdd96e52be8;
        l[0x313130] = 0xcf27aeb776;
        l[0x323130] = 0xfdf6b4fc50;
        l[0x333130] = 0x10d1c6e5a40;
        l[0x343130] = 0x137bd68f344;
        l[0x353130] = 0x15fb5735d0a;
        l[0x363130] = 0x16fc05b13a0;
        l[0x373130] = 0x17a8a91b9d0;
        l[0x383130] = 0x1a60cf0d630;
        l[0x393130] = 0x1b4b46c2774;
        l[0x303230] = 0x1506734d768;
        l[0x313230] = 0x1a78f8e4488;
        l[0x323230] = 0x13309c2d148;
        l[0x333230] = 0x182dd165e1e;
        l[0x343230] = 0x2042a112050;
        l[0x353230] = 0x1b2c74ae158;
        l[0x363230] = 0x1f0b2af3038;
        l[0x373230] = 0x244af534490;
        l[0x383230] = 0x1e6f3b006d8;
        l[0x393230] = 0x22a0260f116;
        l[0x303330] = 0x26997489aec;
        l[0x313330] = 0x2b4b1c39d46;
        l[0x323330] = 0x2ab3090fe00;
        l[0x333330] = 0x20512eee6e0;
        l[0x343330] = 0x3253273a080;
        l[0x353330] = 0x31a0b351d62;
        l[0x363330] = 0x2afbc5507b0;
        l[0x373330] = 0x36c3e6ec58a;
        l[0x383330] = 0x35e1acbf450;
        l[0x393330] = 0x2e90c06c8a2;
        l[0x303430] = 0x3765b94aff0;
        l[0x313430] = 0x3ca96f99750;
        l[0x323430] = 0x3cbe5af6368;
        l[0x333430] = 0x3cf233a4598;
        l[0x343430] = 0x33c9ebade30;
        l[0x353430] = 0x3b85d26b286;
        l[0x363430] = 0x3b83d37b5dc;
        l[0x373430] = 0x4168e297a30;
        l[0x383430] = 0x46577d79080;
        l[0x393430] = 0x4663d16ec7c;
        l[0x303530] = 0x34902041ae8;
        l[0x313530] = 0x44288a4615e;
        l[0x323530] = 0x3d170ac7948;
        l[0x333530] = 0x3e43cafca62;
        l[0x343530] = 0x44128001e5c;
        l[0x353530] = 0x2ff98670ba2;
        l[0x363530] = 0x3adedc72860;
        l[0x373530] = 0x4c9e3e8ad30;
        l[0x383530] = 0x3f0afa37be4;
        l[0x393530] = 0x4671f5277ba;
        l[0x303630] = 0x4d32e913650;
        l[0x313630] = 0x59d108564cc;
        l[0x323630] = 0x576ea0caf1c;
        l[0x333630] = 0x50c1a19b3f2;
        l[0x343630] = 0x59614287f00;
        l[0x353630] = 0x56bbaa68472;
        l[0x363630] = 0x40a25ddce44;
        l[0x373630] = 0x632b63e3546;
        l[0x383630] = 0x606b6b056e0;
        l[0x393630] = 0x52628f84f46;
        l[0x303730] = 0x60f20443470;
        l[0x313730] = 0x690c701643e;
        l[0x323730] = 0x6821c0818b0;
        l[0x333730] = 0x6777757059a;
        l[0x343730] = 0x6d7cbcef8b4;
        l[0x353730] = 0x6c787ddc462;
        l[0x363730] = 0x6bb7fc04bf8;
        l[0x373730] = 0x5aa15c7056e;
        l[0x383730] = 0x672c3986940;
        l[0x393730] = 0x6635e0111fc;
        l[0x303830] = 0x5419cd35ee0;
        l[0x313830] = 0x6c4063240ec;
        l[0x323830] = 0x605590ffaf8;
        l[0x333830] = 0x61825134c14;
        l[0x343830] = 0x7042c5a0a70;
        l[0x353830] = 0x63dbd19ee4c;
        l[0x363830] = 0x650891d3f68;
        l[0x373830] = 0x6dac071f8b4;
        l[0x383830] = 0x4cc270b4680;
        l[0x393830] = 0x5d8fe77f326;
        l[0x303930] = 0x73cc5d9d22c;
        l[0x313930] = 0x85fd14d4b8a;
        l[0x323930] = 0x81bcee988b0;
        l[0x333930] = 0x773637b4750;
        l[0x343930] = 0x8a67e2e15c4;
        l[0x353930] = 0x85f7f6604cc;
        l[0x363930] = 0x7b0ead1d600;
        l[0x373930] = 0x877768d613e;
        l[0x383930] = 0x82c44bc0b68;
        l[0x393930] = 0x60f38ccb62c;
        l[0x303031] = 0x69e18ab6488;
        l[0x313031] = 0xabe165cbb82;
        l[0x323031] = 0x8e7f6e60d58;
        l[0x333031] = 0x9aeb7c510a8;
        l[0x343031] = 0xb0fc5eae560;
        l[0x353031] = 0x92b05b09616;
        l[0x363031] = 0x9f6e9dc34c4;
        l[0x373031] = 0xb6175790f4a;
        l[0x383031] = 0x96e147b1ee0;
        l[0x393031] = 0xa3f1bf358ec;
        l[0x303131] = 0x747818954fc;
        l[0x313131] = 0x5e72fc5f8d0;
        l[0x323131] = 0x6f9ea03ee60;
        l[0x333131] = 0x6d5907d3f6e;
        l[0x343131] = 0x7979a87cdf8;
        l[0x353131] = 0x8305dcbf2ec;
        l[0x363131] = 0x80ce9739ca0;
        l[0x373131] = 0x7cac0480376;
        l[0x383131] = 0x8670de0db04;
        l[0x393131] = 0x842361c1f66;
        l[0x303231] = 0x8d125174b80;
        l[0x313231] = 0xa793b6e9436;
        l[0x323231] = 0x7995ee8da84;
        l[0x333231] = 0x90992d17a30;
        l[0x343231] = 0xb6188539b50;
        l[0x353231] = 0x97690b9bdbe;
        l[0x363231] = 0xa61bcb529ec;
        l[0x373231] = 0xba8056e6852;
        l[0x383231] = 0x9b0b4f78c00;
        l[0x393231] = 0xaa1043f938c;
        l[0x303331] = 0xa5c0aeb9b30;
        l[0x313331] = 0xb56d253c138;
        l[0x323331] = 0xae999c6d878;
        l[0x333331] = 0x80b38c9c986;
        l[0x343331] = 0xc4c7eacef14;
        l[0x353331] = 0xbdda014227c;
        l[0x363331] = 0xa0ccfbfbcd0;
        l[0x373331] = 0xc92fbc7bc30;
        l[0x383331] = 0xc2120caa17c;
        l[0x393331] = 0xa45909125d4;
        l[0x303431] = 0xb249d8cfc68;
        l[0x313431] = 0xc0e764859e8;
        l[0x323431] = 0xbd8bc3c0688;
        l[0x333431] = 0xbabe9d9e680;
        l[0x343431] = 0x9971409dbc0;
        l[0x353431] = 0xafa2d006b76;
        l[0x363431] = 0xac9fd246c94;
        l[0x373431] = 0xbc326793118;
        l[0x383431] = 0xc864ab40d40;
        l[0x393431] = 0xc5700066704;
        l[0x303531] = 0xc5bf9086edc;
        l[0x313531] = 0xf220eb1b1e0;
        l[0x323531] = 0xdb2a0ba3180;
        l[0x333531] = 0xdc9b2a07890;
        l[0x343531] = 0xeb42745edc0;
        l[0x353531] = 0xb09898d528c;
        l[0x363531] = 0xcda881d4018;
        l[0x373531] = 0xfcf6f60e1ce;
        l[0x383531] = 0xd5eeee2cfe0;
        l[0x393531] = 0xe84e7c3a6fc;
        l[0x303631] = 0xe2d6e277b40;
        l[0x313631] = 0x10229e13ee62;
        l[0x323631] = 0xf9b08dbad30;
        l[0x333631] = 0xe64f648685c;
        l[0x343631] = 0xfa8943686c8;
        l[0x353631] = 0xf1ccbf500b8;
        l[0x363631] = 0xb853c6c10b0;
        l[0x373631] = 0x10d13bc5a198;
        l[0x383631] = 0x1043cd7837b0;
        l[0x393631] = 0xdff06e69e14;
        l[0x303731] = 0xd87e3e20f40;
        l[0x313731] = 0xe9f27f5b52e;
        l[0x323731] = 0xe59743a4a28;
        l[0x333731] = 0xe1ebf9c34da;
        l[0x343731] = 0xee0d353d7cc;
        l[0x353731] = 0xe9986a08402;
        l[0x363731] = 0xe5d6e960960;
        l[0x373731] = 0xbc9b34c1f7e;
        l[0x383731] = 0xd79bba83eb0;
        l[0x393731] = 0xd3a4623e534;
        l[0x303831] = 0xed4c4708670;
        l[0x313831] = 0x1223bcd8650a;
        l[0x323831] = 0x1066b9b686cc;
        l[0x333831] = 0x107dcb9ccdde;
        l[0x343831] = 0x1270b4a91070;
        l[0x353831] = 0x10abef695c02;
        l[0x363831] = 0x10c3014fa314;
        l[0x373831] = 0x11dac1f97d96;
        l[0x383831] = 0xd631a7342e8;
        l[0x393831] = 0xf929b0fc05c;
        l[0x303931] = 0x10d5f2cee3d8;
        l[0x313931] = 0x13244c3aa1dc;
        l[0x323931] = 0x127edb175380;
        l[0x333931] = 0x110b2d08c876;
        l[0x343931] = 0x1371440b4d48;
        l[0x353931] = 0x12c8d683b0ce;
        l[0x363931] = 0x114eff4f3858;
        l[0x373931] = 0x12cf2eea16f4;
        l[0x383931] = 0x12228e59359c;
        l[0x393931] = 0xdcf87a9a686;
        l[0x303032] = 0xb8084431a60;
        l[0x313032] = 0x13a30d3948b4;
        l[0x323032] = 0xfe31c61d82c;
        l[0x333032] = 0x1152e8a7f986;
        l[0x343032] = 0x13ee15101d88;
        l[0x353032] = 0x101f83750be0;
        l[0x363032] = 0x11947307c898;
        l[0x373032] = 0x14391ce6f268;
        l[0x383032] = 0x105bea883fa0;
        l[0x393032] = 0x11d5fd6797b6;
        l[0x303132] = 0x118db7bc33c8;
        l[0x313132] = 0xee53064c5a6;
        l[0x323132] = 0x10e582d341b0;
        l[0x333132] = 0x109757055ba0;
        l[0x343132] = 0x11fa7718b574;
        l[0x353132] = 0x130db28f6b5a;
        l[0x363132] = 0x12c06befddf0;
        l[0x373132] = 0x123afc5adb30;
        l[0x383132] = 0x1351c22643b0;
        l[0x393132] = 0x1303181a50f4;
        l[0x303232] = 0xca6f7e369d0;
        l[0x313232] = 0xf99e8ad7dba;
        l[0x323232] = 0xa48a1e26d34;
        l[0x333232] = 0xcd322d50f54;
        l[0x343232] = 0x10fbad537f00;
        l[0x353232] = 0xd710d5af272;
        l[0x363232] = 0xf0366626b8c;
        l[0x373232] = 0x1135e7a5c39e;
        l[0x383232] = 0xd9eeed03850;
        l[0x393232] = 0xf366b244cc8;
        l[0x303332] = 0x11a535957004;
        l[0x313332] = 0x134f16e8a12a;
        l[0x323332] = 0x127d93ff5660;
        l[0x333332] = 0xd665ca5370c;
        l[0x343332] = 0x14c835e4aff8;
        l[0x353332] = 0x13f50cef8156;
        l[0x363332] = 0x10bd2824b320;
        l[0x373332] = 0x150c6b18db76;
        l[0x383332] = 0x143645bf5eb8;
        l[0x393332] = 0x10f3a10f7a86;
        l[0x303432] = 0x1550a04d0160;
        l[0x313432] = 0x16d43b09ffb4;
        l[0x323432] = 0x166b9ce6b790;
        l[0x333432] = 0x1612df42c004;
        l[0x343432] = 0x127fdccd3ca0;
        l[0x353432] = 0x14ce74f9b26a;
        l[0x363432] = 0x147259dbdc04;
        l[0x373432] = 0x160a803ecb6c;
        l[0x383432] = 0x174603b19820;
        l[0x393432] = 0x16eacdc21a60;
        l[0x303532] = 0x11bb5c6c4cf0;
        l[0x313532] = 0x1646cbb10ada;
        l[0x323532] = 0x13d1d24c7088;
        l[0x333532] = 0x13e5f4a82be6;
        l[0x343532] = 0x1556bd3540a4;
        l[0x353532] = 0xf3bdbef9b4e;
        l[0x363532] = 0x12284e41d800;
        l[0x373532] = 0x16eef13a14f4;
        l[0x383532] = 0x12dfef15a0bc;
        l[0x393532] = 0x14ae3bbad176;
        l[0x303632] = 0x164426412dc0;
        l[0x313632] = 0x1961c8890f00;
        l[0x323632] = 0x1875e8f46bc4;
        l[0x333632] = 0x1671b82406be;
        l[0x343632] = 0x186c1e54a200;
        l[0x353632] = 0x177c0bb6b9e6;
        l[0x363632] = 0x11aba5d0b97c;
        l[0x373632] = 0x1a1838869dc2;
        l[0x383632] = 0x19267fdcd1d0;
        l[0x393632] = 0x157aa4d4cfaa;
        l[0x303732] = 0x17fab456a3a8;
        l[0x313732] = 0x19abba6bea6a;
        l[0x323732] = 0x193322998aa0;
        l[0x333732] = 0x18cc82b99dce;
        l[0x343732] = 0x19f47a5c1b4c;
        l[0x353732] = 0x197a4991d2be;
        l[0x363732] = 0x191246458098;
        l[0x373732] = 0x15005e561e12;
        l[0x383732] = 0x179be5e82358;
        l[0x393732] = 0x17308521f258;
        l[0x303832] = 0x13dc15980670;
        l[0x313832] = 0x18f0679b5ff0;
        l[0x323832] = 0x162dd90c67c0;
        l[0x333832] = 0x1641fb682320;
        l[0x343832] = 0x193490cc6840;
        l[0x353832] = 0x166a401f99e0;
        l[0x363832] = 0x167e627b5540;
        l[0x373832] = 0x181c770ccf50;
        l[0x383832] = 0x11348bf98a80;
        l[0x393832] = 0x147f8058571a;
        l[0x303932] = 0x18d5dbe63f24;
        l[0x313932] = 0x1c4ca7ad6b46;
        l[0x323932] = 0x1b42ec2dbae0;
        l[0x333932] = 0x19011fe20fa4;
        l[0x343932] = 0x1c975797744c;
        l[0x353932] = 0x1b8a9fb375c8;
        l[0x363932] = 0x1942aa41dd20;
        l[0x373932] = 0x1b79a21f3892;
        l[0x383932] = 0x1a68b731f530;
        l[0x393932] = 0x13dcda2555b8;
        l[0x303033] = 0xf75e3b8ce88;
        l[0x313033] = 0x1b9ba8468df6;
        l[0x323033] = 0x15f2966114d0;
        l[0x333033] = 0x180c1e610df4;
        l[0x343033] = 0x1be2195e3b00;
        l[0x353033] = 0x162a66b520ba;
        l[0x363033] = 0x18491201b53c;
        l[0x373033] = 0x1c288a75e816;
        l[0x383033] = 0x166237092cb0;
        l[0x393033] = 0x188605a25c90;
        l[0x303133] = 0x162241f0ced0;
        l[0x313133] = 0x1229f107195a;
        l[0x323133] = 0x151047a7e680;
        l[0x333133] = 0x1490b59493f4;
        l[0x343133] = 0x168d57b283b4;
        l[0x353133] = 0x1813775d952e;
        l[0x363133] = 0x1794ca789b48;
        l[0x373133] = 0x16c4807bf994;
        l[0x383133] = 0x184e2a7bbda8;
        l[0x393133] = 0x17ce1a2a5e70;
        l[0x303233] = 0x1506734d7680;
        l[0x313233] = 0x194a6dc7ce68;
        l[0x323233] = 0x118debf4dfd8;
        l[0x333233] = 0x1538e92efd36;
        l[0x343233] = 0x1b3837e73438;
        l[0x353233] = 0x16141ecd7178;
        l[0x363233] = 0x1853d41e7048;
        l[0x373233] = 0x1b78bd295ad0;
        l[0x383233] = 0x16484b329950;
        l[0x393233] = 0x188d23d0337e;
        l[0x303333] = 0x1101adb1affc;
        l[0x313333] = 0x1354f91e9a92;
        l[0x323333] = 0x12197c079c10;
        l[0x333333] = 0xac3be7f1aac;
        l[0x343333] = 0x15407b499548;
        l[0x353333] = 0x14035826b2ee;
        l[0x363333] = 0xf5eaf142a80;
        l[0x373333] = 0x157159232a1e;
        l[0x383333] = 0x1431399bf9a8;
        l[0x393333] = 0xf81d0a45b3e;
        l[0x303433] = 0x1a0d48543958;
        l[0x313433] = 0x1c252a0f4388;
        l[0x323433] = 0x1b842cd171a8;
        l[0x333433] = 0x1afa08e80da0;
        l[0x343433] = 0x15e35ec97ca0;
        l[0x353433] = 0x1917fb284386;
        l[0x363433] = 0x188a79c500a4;
        l[0x373433] = 0x1abc223285f8;
        l[0x383433] = 0x1c6a89d9ae40;
        l[0x393433] = 0x1bddeda4c404;
        l[0x303533] = 0x18d31afe0550;
        l[0x313533] = 0x1f26d368c9d2;
        l[0x323533] = 0x1baf3e21a140;
        l[0x333533] = 0x1bc3607d5c9e;
        l[0x343533] = 0x1dbd6c7c9c9c;
        l[0x353533] = 0x1535373478d6;
        l[0x363533] = 0x19400cd39060;
        l[0x373533] = 0x1fdb5b0af444;
        l[0x383533] = 0x1a30c6caa974;
        l[0x393533] = 0x1caa56bebf7e;
        l[0x303633] = 0x176204719350;
        l[0x313633] = 0x1ba38ffbddb2;
        l[0x323633] = 0x1a4edcf1d370;
        l[0x333633] = 0x177804cb493c;
        l[0x343633] = 0x1a24aa255c58;
        l[0x353633] = 0x18cbc4120d38;
        l[0x363633] = 0x10bdfeb5f770;
        l[0x373633] = 0x1c469baf2a18;
        l[0x383633] = 0x1aec0f8ff720;
        l[0x393633] = 0x15d496f88774;
        l[0x303733] = 0x1c59bfa6f600;
        l[0x313733] = 0x1e9f0cba489e;
        l[0x323733] = 0x1dee15cd5f18;
        l[0x333733] = 0x1d560fa805ca;
        l[0x343733] = 0x1ede7031c8dc;
        l[0x353733] = 0x1e2be04cf692;
        l[0x363733] = 0x1d9276bb37f0;
        l[0x373733] = 0x17fce722c7ce;
        l[0x383733] = 0x1b7e72e71e30;
        l[0x393733] = 0x1ae1abdb80b4;
        l[0x303833] = 0x1af3d429bf98;
        l[0x313833] = 0x21d06f531fb0;
        l[0x323833] = 0x1e0b44e19940;
        l[0x333833] = 0x1e1f673d54a0;
        l[0x343833] = 0x221498842800;
        l[0x353833] = 0x1e47abf4cb60;
        l[0x363833] = 0x1e5bce5086c0;
        l[0x373833] = 0x208326542c10;
        l[0x383833] = 0x172de73e68d0;
        l[0x393833] = 0x1b973eea1042;
        l[0x303933] = 0x1954da25b7f8;
        l[0x313933] = 0x1def8f2f4d3c;
        l[0x323933] = 0x1c7d003a35d0;
        l[0x333933] = 0x19688c986566;
        l[0x343933] = 0x1e2a5be7a4e8;
        l[0x353933] = 0x1cb4d08e3f5e;
        l[0x363933] = 0x199a33c68188;
        l[0x373933] = 0x1c836acd54d4;
        l[0x383933] = 0x1b09ac6aaa6c;
        l[0x393933] = 0x12406fe7f596;
        l[0x303034] = 0x1a7862ad9540;
        l[0x313034] = 0x2aa6a9e4c81c;
        l[0x323034] = 0x2319be0328ac;
        l[0x333034] = 0x25e248cedbee;
        l[0x343034] = 0x2af85972f458;
        l[0x353034] = 0x235ccccdb3c8;
        l[0x363034] = 0x262a7ae60268;
        l[0x373034] = 0x2b4a090120a0;
        l[0x383034] = 0x239fdb983ef0;
        l[0x393034] = 0x2672acfd28ee;
        l[0x303134] = 0x22e275db4140;
        l[0x313134] = 0x1da0fb0e646a;
        l[0x323134] = 0x2173f590a2a0;
        l[0x333134] = 0x20c99ce7038c;
        l[0x343134] = 0x236660bea92c;
        l[0x353134] = 0x2566044d362e;
        l[0x363134] = 0x24bc90d1efc0;
        l[0x373134] = 0x23a80c1ccf0c;
        l[0x383134] = 0x25ab3a000ea8;
        l[0x393134] = 0x2500631862e8;
        l[0x303234] = 0x221a3feceb60;
        l[0x313234] = 0x27b122729866;
        l[0x323234] = 0x1d8b43f2dcec;
        l[0x333234] = 0x22589bcf86a8;
        l[0x343234] = 0x2a308d1c9620;
        l[0x353234] = 0x2374d93cae4e;
        l[0x363234] = 0x2663c93243f4;
        l[0x373234] = 0x2a7cf85fd212;
        l[0x383234] = 0x23b4eba2eb80;
        l[0x393234] = 0x26a8fee51c84;
        l[0x303334] = 0x259626c123c0;
        l[0x313334] = 0x28a27dde8710;
        l[0x323334] = 0x270ca8309420;
        l[0x333334] = 0x1d8806106ff6;
        l[0x343334] = 0x2b2f47fcab8c;
        l[0x353334] = 0x2997cc42d4c4;
        l[0x363334] = 0x2396007f5168;
        l[0x373334] = 0x2b7bb33fe798;
        l[0x383334] = 0x29e13b21c2b4;
        l[0x393334] = 0x23d4af79295c;
        l[0x303434] = 0x1d1e06255760;
        l[0x313434] = 0x1fcaa1007e6e;
        l[0x323434] = 0x1ef1b6ca33d4;
        l[0x333434] = 0x1e369ebd7452;
        l[0x343434] = 0x179cbf17e6b8;
        l[0x353434] = 0x1bb7d1cb0fbc;
        l[0x363434] = 0x1af95c447160;
        l[0x373434] = 0x1dc4f8de9da2;
        l[0x383434] = 0x1fe6b6dc3280;
        l[0x393434] = 0x1f292683ecca;
        l[0x303534] = 0x21104b175ea4;
        l[0x313534] = 0x292cf397f778;
        l[0x323534] = 0x24b3695e0e60;
        l[0x333534] = 0x24c832a99778;
        l[0x343534] = 0x274c290ad070;
        l[0x353534] = 0x1c5746affbf4;
        l[0x363534] = 0x2181268bbc10;
        l[0x373534] = 0x29f1c6f2149e;
        l[0x383534] = 0x22ac4785c0f0;
        l[0x393534] = 0x25d1c1b88a04;
        l[0x303634] = 0x24a826a428b8;
        l[0x313634] = 0x2a16aceecaa2;
        l[0x323634] = 0x286237ed47c0;
        l[0x333634] = 0x24c1c9ee86c4;
        l[0x343634] = 0x2829bff00020;
        l[0x353634] = 0x267117e53860;
        l[0x363634] = 0x1c2f0490fba0;
        l[0x373634] = 0x2adcbd4b6b10;
        l[0x383634] = 0x29226f34bf78;
        l[0x393634] = 0x22a86a8bd0ac;
        l[0x303734] = 0x2568a334c0c0;
        l[0x313734] = 0x284618bb3ae6;
        l[0x323734] = 0x2760a428e350;
        l[0x333734] = 0x269b19333932;
        l[0x343734] = 0x2887c4195d84;
        l[0x353734] = 0x27a0b68f1d2a;
        l[0x363734] = 0x26d9c82d0db8;
        l[0x373734] = 0x1fc47260aba6;
        l[0x383734] = 0x242fe3cc6ed0;
        l[0x393734] = 0x236597f08084;
        l[0x303834] = 0x278cb6816680;
        l[0x313834] = 0x30348a5b58e2;
        l[0x323834] = 0x2b6fb391cfe4;
        l[0x333834] = 0x2b86c57816f6;
        l[0x343834] = 0x3081822c0448;
        l[0x353834] = 0x2bb4e944a51a;
        l[0x363834] = 0x2bcbfb2aec2c;
        l[0x373834] = 0x2e7f862b486e;
        l[0x383834] = 0x22bfe29d9270;
        l[0x393834] = 0x284a8d20a06c;
        l[0x303934] = 0x2b6b2158ed48;
        l[0x313934] = 0x313519bd980c;
        l[0x323934] = 0x2f65116bc5f8;
        l[0x333934] = 0x2b89508c7cbe;
        l[0x343934] = 0x3182118e4378;
        l[0x353934] = 0x2faf0cd82346;
        l[0x363934] = 0x2bcd22d2eca0;
        l[0x373934] = 0x2f73f31be424;
        l[0x383934] = 0x2d9cbb5c7f14;
        l[0x393934] = 0x22a1797c613e;
        l[0x303035] = 0x1cc14aa7c5d8;
        l[0x313035] = 0x30f21d23d00e;
        l[0x323035] = 0x277b5fe16e40;
        l[0x333035] = 0x2aecf6143f8c;
        l[0x343035] = 0x313d24faa740;
        l[0x353035] = 0x27b7c6f4a452;
        l[0x363035] = 0x2b2e807410fc;
        l[0x373035] = 0x31882cd17e7e;
        l[0x383035] = 0x27f42e07da70;
        l[0x393035] = 0x2b700ad3e278;
        l[0x303135] = 0x2d0c2d2381f4;
        l[0x313135] = 0x267edc32e53c;
        l[0x323135] = 0x2b3bce55fc00;
        l[0x333135] = 0x2a6402d577e2;
        l[0x343135] = 0x2d9e3c263ae0;
        l[0x353135] = 0x3014b757aae8;
        l[0x363135] = 0x2f3dd1057f70;
        l[0x373135] = 0x2de26557473a;
        l[0x383135] = 0x305c6add69dc;
        l[0x393135] = 0x2f84211ed912;
        l[0x303235] = 0x27bce13692d0;
        l[0x313235] = 0x2ea17b563630;
        l[0x323235] = 0x220d0fb84f08;
        l[0x333235] = 0x27f791c6265e;
        l[0x343235] = 0x31a2f536af88;
        l[0x353235] = 0x294a761f43e0;
        l[0x363235] = 0x2ce370481198;
        l[0x373235] = 0x31ebb526e488;
        l[0x383235] = 0x2986dd327a20;
        l[0x393235] = 0x2d24faa7e336;
        l[0x303335] = 0x2cc0e89701dc;
        l[0x313335] = 0x307c599f704a;
        l[0x323335] = 0x2e8239951b50;
        l[0x333335] = 0x22c4c117e6c4;
        l[0x343335] = 0x338c966075b8;
        l[0x353335] = 0x3190d04a3ce6;
        l[0x363335] = 0x2a27f0105100;
        l[0x373335] = 0x33d6b9bd1026;
        l[0x383335] = 0x31d7f7428938;
        l[0x393335] = 0x2a6457238756;
        l[0x303435] = 0x2d23462d1ba8;
        l[0x313435] = 0x306bbc0483f0;
        l[0x323435] = 0x2f6206b1e4f8;
        l[0x333435] = 0x2e7d1c5dee58;
        l[0x343435] = 0x2667290d8880;
        l[0x353435] = 0x2b6fd3f137fe;
        l[0x363435] = 0x2a878c236284;
        l[0x373435] = 0x2df43ec65a10;
        l[0x383435] = 0x309074f67fe0;
        l[0x393435] = 0x2fa91257030c;
        l[0x303535] = 0x1fa16bb88cd4;
        l[0x313535] = 0x2982471c53dc;
        l[0x323535] = 0x24020abd0420;
        l[0x333535] = 0x2412bdc5b4d4;
        l[0x343535] = 0x271be15640a8;
        l[0x353535] = 0x19b594b61558;
        l[0x363535] = 0x1ff9c19bd7e8;
        l[0x373535] = 0x2a3af6fe7f02;
        l[0x383535] = 0x2155cf337c58;
        l[0x393535] = 0x252276725258;
        l[0x303635] = 0x2af66c343680;
        l[0x313635] = 0x318d64da4688;
        l[0x323635] = 0x2f74a57c619c;
        l[0x333635] = 0x2b061940806e;
        l[0x343635] = 0x2f24d78476a0;
        l[0x353635] = 0x2d07e51d4cd6;
        l[0x363635] = 0x208cfb6bffb4;
        l[0x373635] = 0x325b4782c282;
        l[0x383635] = 0x303caf0fb4e0;
        l[0x393635] = 0x285b95f05d8a;
        l[0x303735] = 0x329f7cb6e7b8;
        l[0x313735] = 0x3616b43f02b2;
        l[0x323735] = 0x34fe5b95b658;
        l[0x333735] = 0x340ce55e34b6;
        l[0x343735] = 0x365f742f3394;
        l[0x353735] = 0x3545828dfe76;
        l[0x363735] = 0x3452a8ea1780;
        l[0x373735] = 0x2bbf26783cca;
        l[0x383735] = 0x31161719e608;
        l[0x393735] = 0x301fdffc2038;
        l[0x303835] = 0x2923514d31e8;
        l[0x313835] = 0x33907ec29f30;
        l[0x323835] = 0x2dc61c8bfc40;
        l[0x333835] = 0x2dda3ee7b7a0;
        l[0x343835] = 0x33d4a7f3a780;
        l[0x353835] = 0x2e02839f2e60;
        l[0x363835] = 0x2e16a5fae9c0;
        l[0x373835] = 0x315084e2e590;
        l[0x383835] = 0x23209dc82570;
        l[0x393835] = 0x29c6bc0d8292;
        l[0x303935] = 0x3286f458d8bc;
        l[0x313935] = 0x39795f18f19e;
        l[0x323935] = 0x37450c6abd80;
        l[0x333935] = 0x329b2d4e5414;
        l[0x343935] = 0x39c40f02faa4;
        l[0x353935] = 0x378cbff07868;
        l[0x363935] = 0x32dcb7ae2190;
        l[0x373935] = 0x373a503995ea;
        l[0x383935] = 0x34fece1dced0;
        l[0x393935] = 0x27cab5e0a098;
        l[0x303036] = 0x1eebc771a1c0;
        l[0x313036] = 0x371fd5853d10;
        l[0x323036] = 0x2bbff734d2a4;
        l[0x333036] = 0x2fdb49217bba;
        l[0x343036] = 0x3766469cec78;
        l[0x353036] = 0x2bf7c788e0ec;
        l[0x363036] = 0x30183cc22560;
        l[0x373036] = 0x37acb7b49bec;
        l[0x383036] = 0x2c2f97dcef40;
        l[0x393036] = 0x30553062cf12;
        l[0x303136] = 0x2e71f0a380b4;
        l[0x313136] = 0x2694d6209ca8;
        l[0x323136] = 0x2c37cc760488;
        l[0x333136] = 0x2b2e9ab013ee;
        l[0x343136] = 0x2f02560b6cd8;
        l[0x353136] = 0x31ebb5713874;
        l[0x363136] = 0x30e368d9a080;
        l[0x373136] = 0x2f3d22c3c956;
        l[0x383136] = 0x322a0c7e478c;
        l[0x393136] = 0x31205c7a4a46;
        l[0x303236] = 0x2bacd6ebb370;
        l[0x313236] = 0x33deda2bece2;
        l[0x323236] = 0x24db92f679c4;
        l[0x333236] = 0x2be2f0bc1e6c;
        l[0x343236] = 0x376177d6c100;
        l[0x353236] = 0x2d6bdf0e713a;
        l[0x363236] = 0x31ae94f116bc;
        l[0x373236] = 0x37a5a107ce36;
        l[0x383236] = 0x2da3af627fb0;
        l[0x393236] = 0x31eb8891c090;
        l[0x303336] = 0x2c612cf7c2b8;
        l[0x313336] = 0x30c8e54115bc;
        l[0x323336] = 0x2e67a8303830;
        l[0x333336] = 0x206e86abccba;
        l[0x343336] = 0x34541ca68884;
        l[0x353336] = 0x31f13989c720;
        l[0x363336] = 0x291e722f4c28;
        l[0x373336] = 0x3493801e0dbc;
        l[0x383336] = 0x322da09cfe3c;
        l[0x393336] = 0x2950195d6d48;
        l[0x303436] = 0x2db21485d100;
        l[0x313436] = 0x318ed15b4524;
        l[0x323436] = 0x304cbcee1c70;
        l[0x333436] = 0x2f366c54b954;
        l[0x343436] = 0x259cd15b45e0;
        l[0x353436] = 0x2b8b8071467a;
        l[0x363436] = 0x2a71d25e0484;
        l[0x373436] = 0x2e78070b91fc;
        l[0x383436] = 0x318721701360;
        l[0x393436] = 0x306e588b2a10;
        l[0x303536] = 0x2ab51a355dc8;
        l[0x313536] = 0x366057b9cbce;
        l[0x323536] = 0x2fdf98728e38;
        l[0x333536] = 0x2ff26475df52;
        l[0x343536] = 0x3386e47336cc;
        l[0x353536] = 0x23b55ccb2d72;
        l[0x363536] = 0x2b1a05f86b40;
        l[0x373536] = 0x3731ff94d9b0;
        l[0x383536] = 0x2cb35ea8a0d4;
        l[0x393536] = 0x312d6230fc8a;
        l[0x303636] = 0x22035b636520;
        l[0x313636] = 0x29bd16939198;
        l[0x323636] = 0x273a5d07f8f4;
        l[0x333636] = 0x21f802bda5e6;
        l[0x343636] = 0x26c7d972c740;
        l[0x353636] = 0x2440ecdde9be;
        l[0x363636] = 0x15877cfe3a8c;
        l[0x373636] = 0x2a70ac9fff0a;
        l[0x383636] = 0x27e819ff3db0;
        l[0x393636] = 0x1e9a3c982c22;
        l[0x303736] = 0x335668e23a00;
        l[0x313736] = 0x3761e76860d6;
        l[0x323736] = 0x36112fa48ac0;
        l[0x333736] = 0x34ee53279ca2;
        l[0x343736] = 0x37a14adfe114;
        l[0x353736] = 0x364efa24223a;
        l[0x363736] = 0x352aba3acec8;
        l[0x373736] = 0x2b13901fe676;
        l[0x383736] = 0x315084f3e0d0;
        l[0x393736] = 0x3028e790ae84;
        l[0x303836] = 0x303b0fdeeb10;
        l[0x313836] = 0x3c70867a5ef0;
        l[0x323836] = 0x35a388612dc0;
        l[0x333836] = 0x35b7aabce920;
        l[0x343836] = 0x3cb4afab6740;
        l[0x353836] = 0x35dfef745fe0;
        l[0x363836] = 0x35f411d01b40;
        l[0x373836] = 0x39b7342a4250;
        l[0x383836] = 0x2919f90d03c0;
        l[0x393836] = 0x30de7a9f3bba;
        l[0x303936] = 0x2cd1332f0a68;
        l[0x313936] = 0x34e787318c6c;
        l[0x323936] = 0x324a610df148;
        l[0x333936] = 0x2ccdda9b62ae;
        l[0x343936] = 0x352253e9e418;
        l[0x353936] = 0x32823161fad6;
        l[0x363936] = 0x2cff81c97ed0;
        l[0x373936] = 0x320f597e6b04;
        l[0x383936] = 0x2f6b03ed3ce4;
        l[0x393936] = 0x1ff98c39f94e;
        l[0x303037] = 0x2e52acafcaa8;
        l[0x313037] = 0x4a8f3d6cd966;
        l[0x323037] = 0x3d4b852048b0;
        l[0x333037] = 0x4215d9d8abe4;
        l[0x343037] = 0x4ae0ecfb0800;
        l[0x353037] = 0x3d8e93ead62a;
        l[0x363037] = 0x425e0befd4bc;
        l[0x373037] = 0x4b329c8936a6;
        l[0x383037] = 0x3dd1a2b563b0;
        l[0x393037] = 0x42a63e06fda0;
        l[0x303137] = 0x3c68f18e704c;
        l[0x313137] = 0x334176836e20;
        l[0x323137] = 0x39cfda155050;
        l[0x333137] = 0x389aab141c6e;
        l[0x343137] = 0x3d0d51843478;
        l[0x353137] = 0x406efe2884dc;
        l[0x363137] = 0x3f3ab455a9a0;
        l[0x373137] = 0x3d4efce25cb6;
        l[0x383137] = 0x40b433db5fb4;
        l[0x393137] = 0x3f7e869c1f26;
        l[0x303237] = 0x3a7624719920;
        l[0x313237] = 0x43f9d91830f0;
        l[0x323237] = 0x328bfe90fa28;
        l[0x333237] = 0x3ab48054346e;
        l[0x343237] = 0x480a735eb8b8;
        l[0x353237] = 0x3c7c092b4d20;
        l[0x363237] = 0x416cc30d92b8;
        l[0x373237] = 0x4856dea1f708;
        l[0x383237] = 0x3cbc1b918cb0;
        l[0x393237] = 0x41b1f8c06da6;
        l[0x303337] = 0x3fcf47befaf4;
        l[0x313337] = 0x44eb34842206;
        l[0x323337] = 0x422ac7a7ab60;
        l[0x333337] = 0x31fdea56f878;
        l[0x343337] = 0x49092e3ed090;
        l[0x353337] = 0x46471b567612;
        l[0x363337] = 0x3c125a165b80;
        l[0x373337] = 0x495599820efa;
        l[0x383337] = 0x46908a356660;
        l[0x393337] = 0x3c51091035d2;
        l[0x303437] = 0x3ef664538e48;
        l[0x313437] = 0x436e07d62e3c;
        l[0x323437] = 0x41fa33fd9bc0;
        l[0x333437] = 0x40b91ccdec1c;
        l[0x343437] = 0x35a279da8b00;
        l[0x353437] = 0x3c7dccd1fcaa;
        l[0x363437] = 0x3b3958286e2c;
        l[0x373437] = 0x3fdfae8fb184;
        l[0x383437] = 0x43684cd7ae70;
        l[0x393437] = 0x4224bd5c7898;
        l[0x303537] = 0x3ce5bb4baa2c;
        l[0x313537] = 0x4a5d394bd050;
        l[0x323537] = 0x42ddd577b600;
        l[0x333537] = 0x42f498d0b8a0;
        l[0x343537] = 0x47165395ece0;
        l[0x353537] = 0x34db6f41169c;
        l[0x363537] = 0x3d627310e0c8;
        l[0x373537] = 0x4b530f4227ae;
        l[0x383537] = 0x3f3cd38fc9c0;
        l[0x393537] = 0x446611bcbc4c;
        l[0x303637] = 0x426e97d85cf0;
        l[0x313637] = 0x4b5ab7296522;
        l[0x323637] = 0x487da506d810;
        l[0x333637] = 0x42771e44d26c;
        l[0x343637] = 0x4807af01de38;
        l[0x353637] = 0x452669d60c48;
        l[0x363637] = 0x343e155eba50;
        l[0x373637] = 0x4c51ca223fc8;
        l[0x383637] = 0x496edeea8a00;
        l[0x393637] = 0x3ec3ded27d84;
        l[0x303737] = 0x32f48ac15eec;
        l[0x313737] = 0x3794c267a2a0;
        l[0x323737] = 0x360c1dab53d0;
        l[0x333737] = 0x34b84d0b0a38;
        l[0x343737] = 0x37cc1fcca540;
        l[0x353737] = 0x3641e2186dac;
        l[0x363737] = 0x34ecae0bbec0;
        l[0x373737] = 0x29524e69d9d4;
        l[0x383737] = 0x3075b992364c;
        l[0x393737] = 0x2f1d280ba886;
        l[0x303837] = 0x394f711776f0;
        l[0x313837] = 0x474dd7c8bf22;
        l[0x323837] = 0x3f7ee4bccd6c;
        l[0x333837] = 0x3f93ae085686;
        l[0x343837] = 0x4793f5c930a0;
        l[0x353837] = 0x3fbd409f68ba;
        l[0x363837] = 0x3fd209eaf1d4;
        l[0x373837] = 0x441f16a711de;
        l[0x383837] = 0x31152e772298;
        l[0x393837] = 0x39f8ba460324;
        l[0x303937] = 0x3ef4425cbb68;
        l[0x313937] = 0x4837911f94f4;
        l[0x323937] = 0x453aa9048130;
        l[0x333937] = 0x3ef48cb9bbce;
        l[0x343937] = 0x487daf200678;
        l[0x353937] = 0x457dcaa0a496;
        l[0x363937] = 0x3f31852ff1c8;
        l[0x373937] = 0x44fcad8c443c;
        l[0x383937] = 0x41f896039d7c;
        l[0x393937] = 0x3052d05832ee;
        l[0x303038] = 0x2e02110c7600;
        l[0x313038] = 0x4e412d0e5c18;
        l[0x323038] = 0x3f13a3610904;
        l[0x333038] = 0x448703808a42;
        l[0x343038] = 0x4e8c34e535a8;
        l[0x353038] = 0x3f500a744174;
        l[0x363038] = 0x44c88de05e10;
        l[0x373038] = 0x4ed73cbc0f44;
        l[0x383038] = 0x3f8c718779f0;
        l[0x393038] = 0x450a184031ea;
        l[0x303138] = 0x478bcf386b50;
        l[0x313138] = 0x3d187e09a942;
        l[0x323138] = 0x4490d93c6400;
        l[0x333138] = 0x432e37644b14;
        l[0x343138] = 0x483e534d807c;
        l[0x353138] = 0x4c16d794b3e6;
        l[0x363138] = 0x4ab51aeaf3a0;
        l[0x373138] = 0x48827c7e8f34;
        l[0x383138] = 0x4c5e8b1a7538;
        l[0x393138] = 0x4afb6b044fa0;
        l[0x303238] = 0x3ea9d94c3cb0;
        l[0x313238] = 0x497b458ccada;
        l[0x323238] = 0x359edde76864;
        l[0x333238] = 0x3ee489dbd044;
        l[0x343238] = 0x4e0def09ce40;
        l[0x353238] = 0x40e2b99eded2;
        l[0x363238] = 0x467d7db45c7c;
        l[0x373238] = 0x4e56aefa059e;
        l[0x383238] = 0x411f20b21770;
        l[0x393238] = 0x46bf08143078;
        l[0x303338] = 0x4615f37d6a00;
        l[0x313338] = 0x4be0fa2d9c30;
        l[0x323338] = 0x48bc42f4c380;
        l[0x333338] = 0x36568f470036;
        l[0x343338] = 0x5082668b2bac;
        l[0x353338] = 0x4d5c09466f24;
        l[0x363338] = 0x41c0338fec08;
        l[0x373338] = 0x50cc89e7c878;
        l[0x383338] = 0x4da3303ebdd4;
        l[0x393338] = 0x41fc9aa324bc;
        l[0x303438] = 0x4a322c3a8240;
        l[0x313438] = 0x4f420a29b582;
        l[0x323438] = 0x4d99caa520bc;
        l[0x333438] = 0x4c2b409e8c0e;
        l[0x343438] = 0x3f94e970a4c8;
        l[0x353438] = 0x475a3408ef00;
        l[0x363438] = 0x45e84c887b78;
        l[0x373438] = 0x4b2c1868dc2e;
        l[0x383438] = 0x4f2b8e53bc20;
        l[0x393438] = 0x4dba8c01a13e;
        l[0x303538] = 0x40f4bee80078;
        l[0x313538] = 0x50334cf27ff6;
        l[0x323538] = 0x47b014202a28;
        l[0x333538] = 0x47c59e5d7f8a;
        l[0x343538] = 0x4c6f637931cc;
        l[0x353538] = 0x37c5f2062fda;
        l[0x363538] = 0x416a200727a0;
        l[0x373538] = 0x512e2e5be838;
        l[0x383538] = 0x437b27720674;
        l[0x393538] = 0x494e6fd23112;
        l[0x303638] = 0x489e74947400;
        l[0x313638] = 0x52b30640ea50;
        l[0x323638] = 0x4f71a9c1fb34;
        l[0x333638] = 0x489d04c2d55e;
        l[0x343638] = 0x4ee45dc25e00;
        l[0x353638] = 0x4b9ece3a2a06;
        l[0x363638] = 0x387da365c7ac;
        l[0x373638] = 0x53b1eb85a082;
        l[0x383638] = 0x506ab5f188b0;
        l[0x393638] = 0x4458a16313aa;
        l[0x303738] = 0x48b8d49e08b0;
        l[0x313738] = 0x4df4e7408d9e;
        l[0x323738] = 0x4c3b7767ea70;
        l[0x333738] = 0x4abdd48069ba;
        l[0x343738] = 0x4e39a4277fb4;
        l[0x353738] = 0x4c7e9b56f3c2;
        l[0x363738] = 0x4aff95030db8;
        l[0x373738] = 0x3de921b6508e;
        l[0x383738] = 0x45fa250f3380;
        l[0x393738] = 0x4477c1416e9c;
        l[0x303838] = 0x329bdf8db500;
        l[0x313838] = 0x425e79222b66;
        l[0x323838] = 0x3988d3f0d30c;
        l[0x333838] = 0x399986f983c2;
        l[0x343838] = 0x4298545a13b8;
        l[0x353838] = 0x39baed0ae52e;
        l[0x363838] = 0x39cba01395e4;
        l[0x373838] = 0x3e9dd9ff08ca;
        l[0x383838] = 0x29228789c2b0;
        l[0x393838] = 0x33206062a5b0;
        l[0x303938] = 0x4447a2d30224;
        l[0x313938] = 0x4eb363f149ce;
        l[0x323938] = 0x4b523179d400;
        l[0x333938] = 0x443df6f1ee6c;
        l[0x343938] = 0x4ef73a0b18ec;
        l[0x353938] = 0x4b930b2f5500;
        l[0x363938] = 0x4478a7818200;
        l[0x373938] = 0x4afa9820514a;
        l[0x383938] = 0x4792363b4880;
        l[0x393938] = 0x33b39a32cd90;
        l[0x303039] = 0x2e61ab2a79a8;
        l[0x313039] = 0x52a402c3f0da;
        l[0x323039] = 0x418d58089528;
        l[0x333039] = 0x47aa73e1ee30;
        l[0x343039] = 0x52ea73dba2a0;
        l[0x353039] = 0x41c5285ca5ce;
        l[0x363039] = 0x47e767829a34;
        l[0x373039] = 0x5330e4f35472;
        l[0x383039] = 0x41fcf8b0b680;
        l[0x393039] = 0x48245b234644;
        l[0x303139] = 0x454973936ac8;
        l[0x313139] = 0x398658d26166;
        l[0x323139] = 0x41e4b8376d40;
        l[0x333139] = 0x4050b019e7d8;
        l[0x343139] = 0x45fa4e0db32c;
        l[0x353139] = 0x4a45b689422a;
        l[0x363139] = 0x48b2939a1568;
        l[0x373139] = 0x46351ac61208;
        l[0x383139] = 0x4a840d9653a0;
        l[0x393139] = 0x48ef873ac18c;
        l[0x303239] = 0x40ceec558510;
        l[0x313239] = 0x4cedc1b6a94c;
        l[0x323239] = 0x36a27e79bae0;
        l[0x333239] = 0x41050625f012;
        l[0x343239] = 0x52018efe0778;
        l[0x353239] = 0x43393fe233ec;
        l[0x363239] = 0x497dbfb18960;
        l[0x373239] = 0x5245b82f170c;
        l[0x383239] = 0x4371103644c0;
        l[0x393239] = 0x49bab3523592;
        l[0x303339] = 0x41834261946c;
        l[0x313339] = 0x47fa9052ab32;
        l[0x323339] = 0x446ebc1349f0;
        l[0x333339] = 0x2fcd5f5e4fbc;
        l[0x343339] = 0x4d16f754a808;
        l[0x353339] = 0x49897d0962ee;
        l[0x363339] = 0x3c83c03250c0;
        l[0x373339] = 0x4d565acc2f9e;
        l[0x383339] = 0x49c5e41c9c68;
        l[0x393339] = 0x3cb56760743e;
        l[0x303439] = 0x479233de9b50;
        l[0x313439] = 0x4d3658cbda6e;
        l[0x323439] = 0x4b55ba2cbbec;
        l[0x333439] = 0x49b5c9e0bac2;
        l[0x343439] = 0x3b9bcb09c5e0;
        l[0x353439] = 0x444719d46134;
        l[0x363439] = 0x42a3cc0e8130;
        l[0x373439] = 0x488119f977d2;
        l[0x383439] = 0x4cf37418b358;
        l[0x393439] = 0x4b510b812bfa;
        l[0x303539] = 0x42ebee79b324;
        l[0x313539] = 0x53f13e152bb4;
        l[0x323539] = 0x4a69e1cb3a20;
        l[0x333539] = 0x4a7de47381fc;
        l[0x343539] = 0x4faf656c51f8;
        l[0x353539] = 0x38971861c410;
        l[0x363539] = 0x4358221a8928;
        l[0x373539] = 0x54ef5419632a;
        l[0x383539] = 0x459f337e9d48;
        l[0x393539] = 0x4c1c37989fb0;
        l[0x303639] = 0x43a04485c280;
        l[0x313639] = 0x4eda75e38066;
        l[0x323639] = 0x4b31fc5e08b8;
        l[0x333639] = 0x438c66779c28;
        l[0x343639] = 0x4a87b50f7b18;
        l[0x353639] = 0x46db0880be8c;
        l[0x363639] = 0x317e34a52548;
        l[0x373639] = 0x4fd03d772af4;
        l[0x383639] = 0x4c21eadc8a90;
        l[0x393639] = 0x3ea5ef2d8678;
        l[0x303739] = 0x4541e71ad858;
        l[0x313739] = 0x4b1240bb6922;
        l[0x323739] = 0x492071c83c38;
        l[0x333739] = 0x4771689b4f06;
        l[0x343739] = 0x4b4da129aa94;
        l[0x353739] = 0x495a393e94e6;
        l[0x363739] = 0x47a9cca54260;
        l[0x373739] = 0x390fb1af779a;
        l[0x383739] = 0x4206b93aaba8;
        l[0x393739] = 0x4052ef277a48;
        l[0x303839] = 0x4063c11d4a38;
        l[0x313839] = 0x51f0bcd2679c;
        l[0x323839] = 0x481a94b92168;
        l[0x333839] = 0x482d60bc7284;
        l[0x343839] = 0x5230e2fa3120;
        l[0x353839] = 0x4852f8c314bc;
        l[0x363839] = 0x4865c4c665d8;
        l[0x373839] = 0x4dc35b1ea464;
        l[0x383839] = 0x35dccda18040;
        l[0x393839] = 0x40fb22c1de7e;
        l[0x303939] = 0x330509151f6c;
        l[0x313939] = 0x3e938cbd8386;
        l[0x323939] = 0x3ac860185a00;
        l[0x333939] = 0x32e05782028c;
        l[0x343939] = 0x3ec40b7cbb34;
        l[0x353939] = 0x3af5e2734390;
        l[0x363939] = 0x3307b0b6feb0;
        l[0x373939] = 0x3a3ab9c6f922;
        l[0x383939] = 0x36685db43ca0;
        l[0x393939] = 0x204b3b7d5fa0;
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
