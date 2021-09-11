//! @ At the end of the program, we must finish things off by writing the
//! post\-amble. If |total_pages=0|, the \.{DVI} file was never opened.
//! If |total_pages>=65536|, the \.{DVI} file will lie. And if
//! |max_push>=65536|, the user deserves whatever chaos might ensue.
//!
//! An integer variable |k| will be declared for use by this routine.
//
// @<Finish the \.{DVI} file@>=
pub(crate) macro Finish_the_DVI_file($globals:expr) {{
    // while cur_s>-1 do
    while $globals.cur_s > -1 {
        // begin if cur_s>0 then dvi_out(pop)
        if $globals.cur_s > 0 {
            dvi_out!($globals, pop.byte());
        }
        // else  begin dvi_out(eop); incr(total_pages);
        else {
            dvi_out!($globals, eop.byte());
            incr!($globals.total_pages);
            // end;
        }
        // decr(cur_s);
        decr!($globals.cur_s);
        // end;
    }
    // if total_pages=0 then print_nl("No pages of output.")
    if $globals.total_pages == 0 {
        print_nl($globals, crate::strpool_str!("No pages of output."));
    }
    // @.No pages of output@>
    // else  begin dvi_out(post); {beginning of the postamble}
    else {
        /// beginning of the postamble
        dvi_out!($globals, post.byte());
        // dvi_four(last_bop); last_bop:=dvi_offset+dvi_ptr-5; {|post| location}
        /// `post` location
        const _: () = ();
        dvi_four($globals, $globals.last_bop);
        $globals.last_bop = $globals.dvi_offset + $globals.dvi_ptr.get() as integer - 5;
        // dvi_four(25400000); dvi_four(473628672); {conversion ratio for sp}
        /// conversion ratio for sp
        const _: () = ();
        dvi_four($globals, 25400000);
        dvi_four($globals, 473628672);
        // prepare_mag; dvi_four(mag); {magnification factor}
        /// magnification factor
        const _: () = ();
        prepare_mag($globals);
        dvi_four($globals, mag!($globals));
        // dvi_four(max_v); dvi_four(max_h);@/
        dvi_four($globals, $globals.max_v.inner());
        dvi_four($globals, $globals.max_h.inner());
        // dvi_out(max_push div 256); dvi_out(max_push mod 256);@/
        dvi_out!($globals, $globals.max_push / 256);
        dvi_out!($globals, $globals.max_push % 256);
        // dvi_out((total_pages div 256) mod 256); dvi_out(total_pages mod 256);@/
        dvi_out!($globals, ($globals.total_pages / 256) % 256);
        dvi_out!($globals, $globals.total_pages % 256);
        // @<Output the font definitions for all fonts that were used@>;
        crate::section_0643::Output_the_font_definitions_for_all_fonts_that_were_used!($globals);
        // dvi_out(post_post); dvi_four(last_bop); dvi_out(id_byte);@/
        dvi_out!($globals, post_post.byte());
        dvi_four($globals, $globals.last_bop);
        dvi_out!($globals, id_byte);
        /// all-purpose index
        let mut k;
        // k:=4+((dvi_buf_size-dvi_ptr) mod 4); {the number of 223's}
        /// the number of 223's
        const _: () = ();
        k = 4 + ((dvi_buf_size - $globals.dvi_ptr.get()) % 4);
        // while k>0 do
        while k > 0 {
            // begin dvi_out(223); decr(k);
            dvi_out!($globals, 223);
            decr!(k);
            // end;
        }
        // @<Empty the last bytes out of |dvi_buf|@>;
        crate::section_0599::Empty_the_last_bytes_out_of_dvi_buf!($globals);
        // print_nl("Output written on "); slow_print(output_file_name);
        print_nl($globals, crate::strpool_str!("Output written on "));
        slow_print($globals, $globals.output_file_name.get() as _);
        // @.Output written on x@>
        // print(" ("); print_int(total_pages); print(" page");
        print($globals, crate::strpool_str!(" (").get() as _);
        print_int($globals, $globals.total_pages);
        print($globals, crate::strpool_str!(" page").get() as _);
        // if total_pages<>1 then print_char("s");
        if $globals.total_pages != 1 {
            print_char(
                make_globals_io_string_log_view!($globals),
                ASCII_code_literal!(b's'),
            );
        }
        // print(", "); print_int(dvi_offset+dvi_ptr); print(" bytes).");
        print($globals, crate::strpool_str!(", ").get() as _);
        print_int(
            $globals,
            $globals.dvi_offset + $globals.dvi_ptr.get() as integer,
        );
        print($globals, crate::strpool_str!(" bytes).").get() as _);
        // b_close(dvi_file);
        b_close(&mut $globals.dvi_file);
        // end
    }
    use crate::pascal::integer;
    use crate::section_0004::make_globals_io_string_log_view;
    use crate::section_0011::dvi_buf_size;
    use crate::section_0016::decr;
    use crate::section_0016::incr;
    use crate::section_0018::ASCII_code_literal;
    use crate::section_0028::b_close;
    use crate::section_0058::print_char;
    use crate::section_0059::print;
    use crate::section_0060::slow_print;
    use crate::section_0062::print_nl;
    use crate::section_0065::print_int;
    use crate::section_0236::mag;
    use crate::section_0288::prepare_mag;
    use crate::section_0586::eop;
    use crate::section_0586::pop;
    use crate::section_0586::post;
    use crate::section_0586::post_post;
    use crate::section_0587::id_byte;
    use crate::section_0598::dvi_out;
    use crate::section_0600::dvi_four;
}}
