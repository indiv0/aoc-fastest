// Original by: alion02
//                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           .
#![feature(thread_local, portable_simd, core_intrinsics)]
#![allow(
    clippy::precedence,
    clippy::missing_transmute_annotations,
    clippy::pointers_in_nomem_asm_block,
    clippy::erasing_op,
    static_mut_refs,
    internal_features,
    clippy::missing_safety_doc,
    clippy::identity_op,
    clippy::zero_prefixed_literal
)]

#[allow(unused)]
use std::{
    arch::{
        asm,
        x86_64::{
            __m128i, __m256i, _bextr2_u32, _mm256_madd_epi16, _mm256_maddubs_epi16,
            _mm256_movemask_epi8, _mm256_shuffle_epi8, _mm_hadd_epi16, _mm_madd_epi16,
            _mm_maddubs_epi16, _mm_minpos_epu16, _mm_movemask_epi8, _mm_packus_epi32,
            _mm_shuffle_epi8, _mm_testc_si128, _pext_u32, _pext_u64,
        },
    },
    array,
    fmt::Display,
    hint::assert_unchecked,
    intrinsics::{likely, unlikely},
    mem::{offset_of, transmute, MaybeUninit},
    ptr,
    simd::prelude::*,
    slice,
};

#[inline]
unsafe fn inner1(s: &[u8]) -> &str {
    static mut BUF: u8x64 = Simd::from_array([
        10,
        1,
        10,
        1,
        10,
        1,
        10,
        1,
        100,
        0,
        1,
        0,
        100,
        0,
        1,
        0,
        16,
        39,
        1,
        0,
        16,
        39,
        1,
        0,
        b'0'.wrapping_neg(),
        0,
        0,
        0,
        b'1',
        b',',
        b',',
        b',',
        b',',
        b',',
        b',',
        b',',
        b',',
        b',',
        b',',
        b',',
        b',',
        b',',
        b',',
        b',',
        b',',
        b',',
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ]);

    let r: *const u8;

    asm!(
        "vpbroadcastb {chunk}, [rip + {buf}+24]",
        "vpaddb {chunk}, {chunk}, [{ptr} + 12]",
        "vpmaddubsw {chunk}, {chunk}, [rip + {buf}]",
        "vpmaddwd {chunk}, {chunk}, [rip + {buf}+8]",
        "vpackusdw {chunk}, {chunk}, {chunk}",
        "vpmaddwd {chunk}, {chunk}, [rip + {buf}+16]",
        "vmovd edx, {chunk}",
        "movzx {imm1:e}, byte ptr[{ptr} + 65]",
        "sub {imm1:e}, {ascii0}",
        "vpbroadcastd {chunk}, [rip + {buf}+25]",
        "vpcmpeqb {chunk}, {chunk}, [{ptr} + 64]",
        "vpmovmskb {imm2:e}, {chunk}",
        "tzcnt {imm2:e}, {imm2:e}",
        "movzx {imm2:e}, byte ptr[{ptr} + {imm2} + 66]",
        "lea {out}, [rip + {buf}+29]",
    "20:",
        "bextr ecx, edx, {mask:e}",
        "xor ecx, {imm1:e}",
        "shrx {c:e}, edx, ecx",
        "xor ecx, {c:e}",
        "xor ecx, {imm2:e}",
        "and ecx, 7",
        "add ecx, 48",
        "mov byte ptr[{out} + {len} - 91], cl",
        "add {len:e}, 2",
        "shr edx, 3",
        "jnz 20b",
        imm1 = out(reg) _,
        imm2 = out(reg) _,
        out("edx") _,
        out("ecx") _,
        c = out(reg) _,
        mask = in(reg) 3 << 8,
        chunk = out(xmm_reg) _,
        buf = sym BUF,
        ptr = inout(reg) s.as_ptr() => _,
        len = inout(reg) s.len() => _,
        ascii0 = const b'0',
        out = out(reg) r,
        options(nostack),
    );

    std::str::from_utf8_unchecked(std::slice::from_raw_parts(r, 17))
}

static LUT: [u64; 1 << 14] = {
    let mut lut = [0; 1 << 14];
    lut[11844] = 174271416563282;
    lut[10364] = 202730212714147;
    lut[1780] = 191025017546403;
    lut[1428] = 202806544852643;
    lut[6172] = 189432989771427;
    lut[12228] = 259246506806003;
    lut[11860] = 174271403980370;
    lut[10492] = 202730346931875;
    lut[10708] = 189120954525347;
    lut[6868] = 189432876525219;
    lut[3828] = 190992268420771;
    lut[3476] = 202815134787235;
    lut[6300] = 189171097429667;
    lut[11532] = 174270842402386;
    lut[10620] = 202730615367331;
    lut[5876] = 202858726188707;
    lut[5524] = 202832314656419;
    lut[6428] = 189433392424611;
    lut[12260] = 259234225883891;
    lut[10748] = 202730883802787;
    lut[6900] = 202744693548707;
    lut[12020] = 203728106841763;
    lut[7924] = 202743298942627;
    lut[7572] = 202743898728099;
    lut[12276] = 259246498679539;
    lut[11916] = 174271345718866;
    lut[11004] = 202730078496419;
    lut[10644] = 189121157949091;
    lut[6804] = 202744691451555;
    lut[11924] = 203728104744611;
    lut[11668] = 202797954918051;
    lut[6812] = 189171231647395;
    lut[11812] = 174271408174674;
    lut[11132] = 202799737497251;
    lut[11828] = 174271412368978;
    lut[10676] = 190805422663331;
    lut[16116] = 189136842548899;
    lut[15764] = 189137442334371;
    lut[7068] = 189433660860067;
    lut[12212] = 259234217757427;
    lut[12220] = 259233547193075;
    lut[6725] = 202366621067818;
    lut[1653] = 202356708354602;
    lut[6157] = 203197932644906;
    lut[6173] = 164541160582845;
    lut[11293] = 164279024971453;
    lut[12741] = 247839653009594;
    lut[6741] = 202366625262122;
    lut[6285] = 202992055757354;
    lut[11421] = 164278764924605;
    lut[6757] = 202366618446378;
    lut[5749] = 202631586261546;
    lut[11549] = 164278496489149;
    lut[6773] = 202366623164970;
    lut[7573] = 164542125272765;
    lut[6557] = 164541017976509;
    lut[11677] = 164278899142333;
    lut[12797] = 247839539763386;
    lut[6661] = 202991746427434;
    lut[9845] = 202975183645226;
    lut[6669] = 202991774214698;
    lut[9621] = 164516454365621;
    lut[6685] = 164540221058749;
    lut[11805] = 164278228053693;
    lut[8957] = 202404828555818;
    lut[6677] = 202367015856682;
    lut[11893] = 202322348616234;
    lut[11653] = 202322936867370;
    lut[6797] = 202992189975082;
    lut[6813] = 164540892147389;
    lut[11933] = 165522963263165;
    lut[12693] = 247839661398202;
    lut[8613] = 202972175280682;
    lut[6693] = 202366627359274;
    lut[13941] = 203181342075434;
    lut[6941] = 164540489494205;
    lut[12061] = 165523634351805;
    lut[6709] = 202367025818154;
    lut[7069] = 164540623711933;
    lut[12189] = 164278630706877;
    lut[13309] = 247839002892474;
    lut[12725] = 247839648815290;
    lut[11718] = 37221263785460;
    lut[11390] = 37221871304180;
    lut[11734] = 37221261688308;
    lut[11518] = 37221468650996;
    lut[4086] = 190615597431823;
    lut[3510] = 190354906758159;
    lut[11750] = 38886108872180;
    lut[11646] = 37222005521908;
    lut[14718] = 190384709385231;
    lut[6462] = 190389435055119;
    lut[11766] = 37221267979764;
    lut[11774] = 37221334433268;
    lut[14846] = 190626778856463;
    lut[14838] = 190384609508367;
    lut[6590] = 190389300050959;
    lut[11902] = 37222273957364;
    lut[14726] = 190384623401999;
    lut[6718] = 190593310997519;
    lut[11670] = 37221270076916;
    lut[15102] = 190384433347599;
    lut[14742] = 190384615275535;
    lut[6846] = 190593446001679;
    lut[11686] = 37221276368372;
    lut[11702] = 37221274271220;
    lut[12286] = 37221737086452;
    lut[15358] = 190384113204239;
    lut[16374] = 190624228719631;
    lut[15798] = 190561065188367;
    lut[1143] = 87765778397121;
    lut[8319] = 266931276004369;
    lut[1655] = 280189034740753;
    lut[10367] = 236581377105517;
    lut[3191] = 87800138135489;
    lut[3703] = 266943355599889;
    lut[10495] = 236581108670061;
    lut[11991] = 233391028921023;
    lut[3831] = 216148338630253;
    lut[3479] = 216549846240877;
    lut[383] = 87768596969409;
    lut[5239] = 87815170521025;
    lut[8575] = 266930722356241;
    lut[5751] = 266960535469073;
    lut[10727] = 236555995274861;
    lut[5527] = 216584205979245;
    lut[511] = 87768089491393;
    lut[503] = 87768066086849;
    lut[6263] = 87780816771009;
    lut[7287] = 87782958266305;
    lut[7799] = 266908995861521;
    lut[10751] = 236580836040301;
    lut[7575] = 236548287712877;
    lut[8831] = 266930990791697;
    lut[9847] = 266926175730705;
    lut[10631] = 236556005760621;
    lut[9975] = 236539226447469;
    lut[11807] = 233390776344173;
    lut[8599] = 266931037159697;
    lut[11895] = 280206214609937;
    lut[10647] = 236555999469165;
    lut[12023] = 236556406316653;
    lut[11935] = 233391044779629;
    lut[895] = 87794366773185;
    lut[10663] = 236556001566317;
    lut[1023] = 87768372238273;
    lut[15991] = 266932601404433;
    lut[10679] = 236555997372013;
    lut[12409] = 109019476330651;
    lut[1905] = 105840740174234;
    lut[1441] = 105843716614554;
    lut[12537] = 109020013201563;
    lut[6993] = 105734763776155;
    lut[3953] = 105849330108826;
    lut[3489] = 105981155568026;
    lut[11433] = 107413700225434;
    lut[6001] = 105832150239642;
    lut[5537] = 106086382266778;
    lut[11561] = 107416870455451;
    lut[12793] = 109019619953050;
    lut[7025] = 105734774294938;
    lut[8049] = 105875099912602;
    lut[7585] = 105706277661082;
    lut[6569] = 105735268690330;
    lut[11689] = 107416732707226;
    lut[12921] = 109019205798043;
    lut[6913] = 105734783666586;
    lut[12033] = 108534028601754;
    lut[10097] = 108056943298970;
    lut[13049] = 109019742668955;
    lut[12689] = 109019930331546;
    lut[12145] = 105866509978010;
    lut[11681] = 109541683456410;
    lut[12705] = 109019846445466;
    lut[6945] = 105734716557722;
    lut[14193] = 109130685122970;
    lut[13729] = 136904920099226;
    lut[6953] = 136936180680859;
    lut[6961] = 105734765873307;
    lut[16241] = 109593014864282;
    lut[15777] = 109550273391002;
    lut[7081] = 105702138980762;
    lut[12201] = 108399378442650;
    lut[7122] = 47910082096018;
    lut[4082] = 47951183653778;
    lut[14842] = 47921188613010;
    lut[7042] = 47910079998866;
    lut[11810] = 90938893795561;
    lut[11826] = 90938843463913;
    lut[14459] = 258394985014171;
    lut[2035] = 265660930925467;
    lut[6203] = 258411949917083;
    lut[11859] = 109685330781408;
    lut[4083] = 265652340990875;
    lut[3507] = 265601188299675;
    lut[6331] = 258393608225691;
    lut[14715] = 258400890594203;
    lut[6459] = 258411144610715;
    lut[11659] = 109552863562265;
    lut[14971] = 267265166222235;
    lut[9651] = 265618099733403;
    lut[6715] = 258411681481627;
    lut[15099] = 258395521885083;
    lut[11699] = 265635548038043;
    lut[6843] = 258411811505051;
    lut[6971] = 265061364597659;
    lut[7099] = 258411413046171;
    lut
};

#[inline]
unsafe fn inner2(s: &[u8]) -> u64 {
    let out: u64;
    asm!(
        "movzx {out:e}, byte ptr[{ptr} + 73]",
        "shl {out:e}, 16",
        "xor {out:l}, byte ptr[{ptr} + 65]",
        "xor {out}, qword ptr[{ptr} + 74]",
        "pext {out}, {out}, {mask}",
        "mov {out}, qword ptr[rdx + {out} * 8]",
        out = out(reg) out,
        out("ecx") _,
        inout("rdx") &LUT => _,
        ptr = in(reg) s.as_ptr(),
        options(nostack),
        mask = in(reg) 0x07_00_04_00_07_07_04_07u64,
    );
    out
}

#[inline]
pub fn part1(s: &str) -> &str {
    unsafe { inner1(s.as_bytes()) }
}

#[inline]
pub fn run(s: &str) -> u64 {
    unsafe { inner2(s.as_bytes()) }
}
