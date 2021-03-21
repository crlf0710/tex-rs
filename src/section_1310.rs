//! ` `
// @d undump_four_ASCII==
macro_rules! undump_four_ASCII {
    ($globals:expr, $k:expr) => {{
        let w;
        // undump_qqqq(w);
        undump_qqqq!($globals, w);
        // str_pool[k]:=si(qo(w.b0)); str_pool[k+1]:=si(qo(w.b1));
        $globals.str_pool[$k] = packed_ASCII_code(w[FOUR_QUARTERS_B0]);
        $globals.str_pool[$k + 1] = packed_ASCII_code(w[FOUR_QUARTERS_B1]);
        // str_pool[k+2]:=si(qo(w.b2)); str_pool[k+3]:=si(qo(w.b3))
        $globals.str_pool[$k + 2] = packed_ASCII_code(w[FOUR_QUARTERS_B2]);
        $globals.str_pool[$k + 3] = packed_ASCII_code(w[FOUR_QUARTERS_B3]);

        use crate::section_0038::packed_ASCII_code;
        use crate::section_0113::FOUR_QUARTERS_B0;
        use crate::section_0113::FOUR_QUARTERS_B1;
        use crate::section_0113::FOUR_QUARTERS_B2;
        use crate::section_0113::FOUR_QUARTERS_B3;
    }}
}

// @<Undump the string pool@>=
macro_rules! Undump_the_string_pool {
    ($globals:expr, $lbl_bad_fmt:lifetime) => {{
        // undump_size(0)(pool_size)('string pool size')(pool_ptr);
        undump_size!($globals, 0, pool_size, "string pool size", $globals.pool_ptr, pool_pointer::new, $lbl_bad_fmt);
        // undump_size(0)(max_strings)('max strings')(str_ptr);
        undump_size!($globals, 0, max_strings, "max strings", $globals.str_ptr, str_number::new, $lbl_bad_fmt);
        // for k:=0 to str_ptr do undump(0)(pool_ptr)(str_start[k]);
        for k in 0..=$globals.str_ptr.get() {
            undump!($globals, 0, $globals.pool_ptr.get(), $globals.str_start[k], pool_pointer::new, $lbl_bad_fmt);
        }
        // k:=0;
        {
            let mut k = 0;
            // while k+4<pool_ptr do
            while k + 4 < $globals.pool_ptr.get() {
                // begin undump_four_ASCII; k:=k+4;
                undump_four_ASCII!($globals, k);
                k += 4;
                // end;
            }
            // k:=pool_ptr-4; undump_four_ASCII;
            k = $globals.pool_ptr.get() - 4;
            undump_four_ASCII!($globals, k);
        }
        // init_str_ptr:=str_ptr; init_pool_ptr:=pool_ptr
        $globals.init_str_ptr = $globals.str_ptr;
        $globals.init_pool_ptr = $globals.pool_ptr;
        use crate::section_0011::max_strings;
        use crate::section_0011::pool_size;
        use crate::section_0038::str_number;
        use crate::section_0038::pool_pointer;
    }}
}
