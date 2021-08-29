//! ` `

// @<Finish line, emit a \.{\\par}@>=
macro_rules! Finish_line__emit_a_par {
    ($globals:expr) => {
        // begin loc:=limit+1; cur_cs:=par_loc; cur_cmd:=eq_type(cur_cs);
        loc!($globals) = limit!($globals) + 1;
        $globals.cur_cs = $globals.par_loc;
        $globals.cur_cmd = eq_type!($globals, $globals.cur_cs);
        // cur_chr:=equiv(cur_cs);
        $globals.cur_chr = chr_code_type::new(equiv!($globals, $globals.cur_cs) as _);
        // if cur_cmd>=outer_call then check_outer_validity;
        // end
        use crate::section_0297::chr_code_type;
    }
}
