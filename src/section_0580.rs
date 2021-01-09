//! ` `
// @<Increase the number of parameters...@>=
macro_rules! Increase_the_number_of_parameters_in_the_last_font {
    ($globals:expr, $n:expr, $f:expr) => {{
        // begin repeat if fmem_ptr=font_mem_size then
        loop {
            if $globals.fmem_ptr.get() == font_mem_size {
                panic!("overflow");
                // overflow("font memory",font_mem_size);
            }
            // @:TeX capacity exceeded font memory}{\quad font memory@>
            // font_info[fmem_ptr].sc:=0; incr(fmem_ptr); incr(font_params[f]);
            $globals.font_info[$globals.fmem_ptr][MEMORY_WORD_SC] = scaled::zero();
            incr!($globals.fmem_ptr);
            incr!($globals.font_params[$f]);
            // until n=font_params[f];
            if $n == $globals.font_params[$f].get() as integer {
                break;
            }
        }
        // cur_val:=fmem_ptr-1; {this equals |param_base[f]+font_params[f]|}
        $globals.cur_val = $globals.fmem_ptr.get() as integer - 1;
        /// this equals `param_base[f]+font_params[f]`
        const _ : () = ();
        // end

        use crate::pascal::integer;
        use crate::section_0011::font_mem_size;
        use crate::section_0101::scaled;
        use crate::section_0101::MEMORY_WORD_SC;
    }}
}
