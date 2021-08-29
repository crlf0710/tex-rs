//! @ Having |font_glue| allocated for each text font saves both time and memory.
//! If any of the three spacing parameters are subsequently changed by the
//! use of \.{\\fontdimen}, the |find_font_dimen| procedure deallocates the
//! |font_glue| specification allocated here.

//
// @<Find the glue specification...@>=
macro_rules! Find_the_glue_specification_main_p_for_text_spaces_in_the_current_font {
    ($globals:expr) => {{
        // begin main_p:=font_glue[cur_font];
        $globals.main_p = $globals.font_glue[cur_font!($globals)];
        // if main_p=null then
        if $globals.main_p == null {
            // begin main_p:=new_spec(zero_glue); main_k:=param_base[cur_font]+space_code;
            $globals.main_p = new_spec($globals, zero_glue)?;
            $globals.main_k = font_index::new($globals.param_base[cur_font!($globals)] as _)
                + font_index::new(space_code as _);
            // width(main_p):=font_info[main_k].sc; {that's |space(cur_font)|}
            width!($globals, $globals.main_p) = $globals.font_info[$globals.main_k][MEMORY_WORD_SC];
            /// that's `space(cur_font)`
            const _: () = ();
            // stretch(main_p):=font_info[main_k+1].sc; {and |space_stretch(cur_font)|}
            stretch!($globals, $globals.main_p) = $globals.font_info[$globals.main_k + 1][MEMORY_WORD_SC];
            /// and `space_stretch(cur_font)`
            const _: () = ();
            // shrink(main_p):=font_info[main_k+2].sc; {and |space_shrink(cur_font)|}
            shrink!($globals, $globals.main_p) = $globals.font_info[$globals.main_k + 2][MEMORY_WORD_SC];
            /// and `space_shrink(cur_font)`
            const _: () = ();
            // font_glue[cur_font]:=main_p;
            $globals.font_glue[cur_font!($globals)] = $globals.main_p;
            // end;
        }
        // end
        use crate::section_0101::MEMORY_WORD_SC;
        use crate::section_0115::null;
        use crate::section_0115::pointer;
        use crate::section_0151::new_spec;
        use crate::section_0547::space_code;
        use crate::section_0548::font_index;
    }};
}
