// ` `
// @d app_lc_hex(#)==l:=#;
macro_rules! app_lc_hex {
    ($globals:expr, $v:expr) => {{
        let l = $v;
        // if l<10 then append_char(l+"0")@+else append_char(l-10+"a")
        if l < 10 {
            append_char($globals, ASCII_code_literal!(l + b'0'));
        } else {
            append_char($globals, ASCII_code_literal!(l - 10 + b'a'));
        }
    }}
}

// @<Make the first 256...@>=
macro_rules! Make_the_first_256_strings {
    ($globals:expr, $g: expr) => {{
        let mut globals = make_globals_string_view!($globals);
        // for k:=0 to 255 do
        for k in 0..=255 {
            // begin if (@<Character |k| cannot be printed@>) then
            if Character_k_cannot_be_printed!(k) {
                // begin append_char("^"); append_char("^");
                append_char(globals.reborrow(), ASCII_code_literal!(b'^'));
                append_char(globals.reborrow(), ASCII_code_literal!(b'^'));
                // if k<@'100 then append_char(k+@'100)
                if k < 0o100 {
                    append_char(globals.reborrow(), ASCII_code_literal!(k + 0o100));
                }
                // else if k<@'200 then append_char(k-@'100)
                else if k < 0o200 {
                    append_char(globals.reborrow(), ASCII_code_literal!(k - 0o100));
                }
                // else begin app_lc_hex(k div 16); app_lc_hex(k mod 16);
                else {
                    app_lc_hex!(globals.reborrow(), k / 16);
                    app_lc_hex!(globals.reborrow(), k % 16);
                    // end;
                }
                // end
            } else {
                // else append_char(k);
                append_char(globals.reborrow(), ASCII_code_literal!(k));
            }
            // g:=make_string;
            $g = make_string(globals.reborrow());
        }
        // end
        use crate::section_0004::TeXGlobalsStringView;
        use crate::section_0042::append_char;
        use crate::section_0043::make_string;
    }};
}
