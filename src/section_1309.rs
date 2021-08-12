//! ` `
// @d dump_four_ASCII==
macro_rules! dump_four_ASCII {
    ($globals:expr, $k:expr) => {{
        let mut w = four_quarters::default();
        // w.b0:=qi(so(str_pool[k])); w.b1:=qi(so(str_pool[k+1]));
        w[FOUR_QUARTERS_B0] = $globals.str_pool[$k].into();
        w[FOUR_QUARTERS_B1] = $globals.str_pool[$k + 1].into();
        // w.b2:=qi(so(str_pool[k+2])); w.b3:=qi(so(str_pool[k+3]));
        w[FOUR_QUARTERS_B2] = $globals.str_pool[$k + 2].into();
        w[FOUR_QUARTERS_B3] = $globals.str_pool[$k + 3].into();
        // dump_qqqq(w)
        dump_qqqq!($globals, w);
        use crate::section_0113::four_quarters;
        use crate::section_0113::FOUR_QUARTERS_B0;
        use crate::section_0113::FOUR_QUARTERS_B1;
        use crate::section_0113::FOUR_QUARTERS_B2;
        use crate::section_0113::FOUR_QUARTERS_B3;
    }}
}

// @<Dump the string pool@>=
macro_rules! Dump_the_string_pool {
    ($globals:expr) => {{
        debug_assert!($globals.pool_ptr.get() >= 4);
        // dump_int(pool_ptr);
        dump_int!($globals, $globals.pool_ptr.get() as _);
        // dump_int(str_ptr);
        dump_int!($globals, $globals.str_ptr.get() as _);
        // for k:=0 to str_ptr do dump_int(str_start[k]);
        for k in 0..=$globals.str_ptr.get() {
            dump_int!($globals, $globals.str_start[k].get() as _);
        }
        {
            // k:=0;
            let mut k = 0;
            // while k+4<pool_ptr do
            while k + 4 < $globals.pool_ptr.get() {
                // begin dump_four_ASCII; k:=k+4;
                dump_four_ASCII!($globals, k);
                k += 4;
                // end;
            }
        }
        // k:=pool_ptr-4; dump_four_ASCII;
        let k = $globals.pool_ptr.get() - 4;
        dump_four_ASCII!($globals, k);
        // print_ln; print_int(str_ptr); print(" strings of total length ");
        print_ln(make_globals_io_string_log_view!($globals));
        print_int($globals, $globals.str_ptr.get() as _);
        print($globals, strpool_str!(" strings of total length ").get() as _);
        // print_int(pool_ptr)
        print_int($globals, $globals.pool_ptr.get() as _);
        use crate::section_0004::TeXGlobalsIoStringLogView;
        use crate::section_0057::print_ln;
        use crate::section_0059::print;
        use crate::section_0065::print_int;
    }}
}
