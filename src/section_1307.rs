//! @ The next few sections of the program should make it clear how we use the
//! dump/undump macros.
//
// @<Dump constants for consistency check@>=
macro_rules! Dump_constants_for_consistency_check {
    ($globals:expr) => {{
        // dump_int(@$);@/
        dump_int!($globals, *CHECKSUM as _);
        // dump_int(mem_bot);@/
        dump_int!($globals, mem_bot as _);
        // dump_int(mem_top);@/
        dump_int!($globals, mem_top as _);
        // dump_int(eqtb_size);@/
        dump_int!($globals, eqtb_size as _);
        // dump_int(hash_prime);@/
        dump_int!($globals, hash_prime as _);
        // dump_int(hyph_size)
        dump_int!($globals, hyph_size as _);
        use crate::string_pool::CHECKSUM;
        use crate::section_0012::mem_bot;
        use crate::section_0012::mem_top;
        use crate::section_0247::eqtb_size;
        use crate::section_0012::hash_prime;
        use crate::section_0012::hyph_size;
    }}
}
