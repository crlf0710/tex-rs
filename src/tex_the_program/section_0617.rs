//! ` `
// @<Initialize variables as |ship_out| begins@>=
macro_rules! Initialize_variables_as_ship_out_begins {
    ($globals:expr) => {{
        // dvi_h:=0; dvi_v:=0; cur_h:=h_offset; dvi_f:=null_font;
        $globals.dvi_h = scaled::zero();
        $globals.dvi_v = scaled::zero();
        $globals.cur_h = h_offset!($globals);
        $globals.dvi_f = null_font;
        // ensure_dvi_open;
        ensure_dvi_open!($globals);
        // if total_pages=0 then
        if $globals.total_pages == 0 {
            /// saved `selector` setting
            let old_setting;
            // begin dvi_out(pre); dvi_out(id_byte); {output the preamble}
            /// output the preamble
            const _ : () = ();
            dvi_out!($globals, pre.byte());
            dvi_out!($globals, id_byte);
            // @^preamble of \.{DVI} file@>
            // dvi_four(25400000); dvi_four(473628672); {conversion ratio for sp}
            /// conversion ratio for sp
            const _ : () = ();
            dvi_four($globals, 25400000);
            dvi_four($globals, 473628672);
            // prepare_mag; dvi_four(mag); {magnification factor is frozen}
            /// magnification factor is frozen
            const _: () = ();
            prepare_mag($globals);
            dvi_four($globals, mag!($globals));
            // old_setting:=selector; selector:=new_string;
            old_setting = $globals.selector;
            $globals.selector = new_string.into();
            // print(" TeX output "); print_int(year); print_char(".");
            print($globals, strpool_str!(" TeX output ").get() as _);
            print_int($globals, year!($globals));
            print_char(make_globals_io_string_log_view!($globals), ASCII_code_literal!(b'.'));
            // print_two(month); print_char("."); print_two(day);
            print_two($globals, month!($globals));
            print_char(make_globals_io_string_log_view!($globals), ASCII_code_literal!(b'.'));
            print_two($globals, day!($globals));
            // print_char(":"); print_two(time div 60);
            print_char(make_globals_io_string_log_view!($globals), ASCII_code_literal!(b':'));
            print_two($globals, time!($globals) / 60);
            // print_two(time mod 60);
            print_two($globals, time!($globals) % 60);
            // selector:=old_setting; dvi_out(cur_length);
            $globals.selector = old_setting;
            dvi_out!($globals, cur_length!($globals));
            // for s:=str_start[str_ptr] to pool_ptr-1 do dvi_out(so(str_pool[s]));
            for s in $globals.str_start[$globals.str_ptr].get()..=$globals.pool_ptr.get() - 1 {
                dvi_out!($globals, $globals.str_pool[s].numeric_value());
            }
            // pool_ptr:=str_start[str_ptr]; {flush the current string}
            $globals.pool_ptr = $globals.str_start[$globals.str_ptr];
            // end
        }
        use crate::section_0054::new_string;
        use crate::section_0066::print_two;
        use crate::section_0101::scaled;
        use crate::section_0232::null_font;
        use crate::section_0288::prepare_mag;
        use crate::section_0586::pre;
        use crate::section_0587::id_byte;
    }}
}
