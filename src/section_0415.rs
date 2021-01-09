//! ` `

// @<Fetch a token list...@>=
macro_rules! Fetch_a_token_list_or_font_identifier__provided_that_level_is_tok_val {
    ($globals:expr, $level:expr, $m:expr) => {{
        // if level<>tok_val then
        if $level.get() as integer != cur_val_level_kind::tok_val as integer {
            todo!("fetch 1");
            //   begin print_err("Missing number, treated as zero");
            // @.Missing number...@>
            //   help3("A number should have been here; I inserted `0'.")@/
            //     ("(If you can't figure out why I needed to see a number,")@/
            //     ("look up `weird error' in the index to The TeXbook.)");
            // @:TeXbook}{\sl The \TeX book@>
            //   back_error; scanned_result(0)(dimen_val);
            //   end
        }
        // else if cur_cmd<=assign_toks then
        else if $globals.cur_cmd <= assign_toks {
            // begin if cur_cmd<assign_toks then {|cur_cmd=toks_register|}
            if $globals.cur_cmd < assign_toks {
                /// `cur_cmd=toks_register`
                const _ : () = ();
                // begin scan_eight_bit_int; m:=toks_base+cur_val;
                scan_eight_bit_int($globals)?;
                $m = chr_code_type::new((toks_base as integer + $globals.cur_val) as _);
                // end;
            }
            // scanned_result(equiv(m))(tok_val);
            scanned_result!(
                $globals,
                equiv!($globals, $m.get() as pointer) as _,
                cur_val_level_kind::tok_val
            );
            // end
        }
        // else  begin back_input; scan_font_ident;
        else {
            back_input($globals);
            scan_font_ident($globals)?;
            // scanned_result(font_id_base+cur_val)(ident_val);
            scanned_result!(
                $globals,
                (font_id_base as integer + $globals.cur_val as integer) as _,
                cur_val_level_kind::ident_val
            );
            // end
        }
        use crate::pascal::integer;
        use crate::section_0222::font_id_base;
        use crate::section_0230::toks_base;
        use crate::section_0325::back_input;
        use crate::section_0433::scan_eight_bit_int;
        use crate::section_0577::scan_font_ident;
    }}
}