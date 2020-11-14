//! @ At the end of the program, we must finish things off by writing the
//! post\-amble. If |total_pages=0|, the \.{DVI} file was never opened.
//! If |total_pages>=65536|, the \.{DVI} file will lie. And if
//! |max_push>=65536|, the user deserves whatever chaos might ensue.
//!
//! An integer variable |k| will be declared for use by this routine.
//
// @<Finish the \.{DVI} file@>=
macro_rules! Finish_the_DVI_file {
    ($globals:expr) => {{
        // while cur_s>-1 do
        //   begin if cur_s>0 then dvi_out(pop)
        //   else  begin dvi_out(eop); incr(total_pages);
        //     end;
        //   decr(cur_s);
        //   end;
        // if total_pages=0 then print_nl("No pages of output.")
        // @.No pages of output@>
        // else  begin dvi_out(post); {beginning of the postamble}
        //   dvi_four(last_bop); last_bop:=dvi_offset+dvi_ptr-5; {|post| location}
        //   dvi_four(25400000); dvi_four(473628672); {conversion ratio for sp}
        //   prepare_mag; dvi_four(mag); {magnification factor}
        //   dvi_four(max_v); dvi_four(max_h);@/
        //   dvi_out(max_push div 256); dvi_out(max_push mod 256);@/
        //   dvi_out((total_pages div 256) mod 256); dvi_out(total_pages mod 256);@/
        //   @<Output the font definitions for all fonts that were used@>;
        //   dvi_out(post_post); dvi_four(last_bop); dvi_out(id_byte);@/
        //   k:=4+((dvi_buf_size-dvi_ptr) mod 4); {the number of 223's}
        //   while k>0 do
        //     begin dvi_out(223); decr(k);
        //     end;
        //   @<Empty the last bytes out of |dvi_buf|@>;
        //   print_nl("Output written on "); slow_print(output_file_name);
        // @.Output written on x@>
        //   print(" ("); print_int(total_pages); print(" page");
        //   if total_pages<>1 then print_char("s");
        //   print(", "); print_int(dvi_offset+dvi_ptr); print(" bytes).");
        //   b_close(dvi_file);
        //   end
    }}
}