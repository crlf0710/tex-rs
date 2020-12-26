//! ` `

// @<Fetch a token list...@>=
macro_rules! Fetch_a_token_list_or_font_identifier__provided_that_level_is_tok_val {
    ($globals:expr) => {{
        todo!("fetch");
        // if level<>tok_val then
        //   begin print_err("Missing number, treated as zero");
        // @.Missing number...@>
        //   help3("A number should have been here; I inserted `0'.")@/
        //     ("(If you can't figure out why I needed to see a number,")@/
        //     ("look up `weird error' in the index to The TeXbook.)");
        // @:TeXbook}{\sl The \TeX book@>
        //   back_error; scanned_result(0)(dimen_val);
        //   end
        // else if cur_cmd<=assign_toks then
        //   begin if cur_cmd<assign_toks then {|cur_cmd=toks_register|}
        //     begin scan_eight_bit_int; m:=toks_base+cur_val;
        //     end;
        //   scanned_result(equiv(m))(tok_val);
        //   end
        // else  begin back_input; scan_font_ident;
        //   scanned_result(font_id_base+cur_val)(ident_val);
        //   end
    }}
}