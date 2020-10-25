// @ @d app_lc_hex(#)==l:=#;
//   if l<10 then append_char(l+"0")@+else append_char(l-10+"a")
//
// @<Make the first 256...@>=
macro_rules! Make_the_first_256_strings {
    ($globals:expr, $g: expr) => {
        // for k:=0 to 255 do
        for k in 0..=255 {
            // begin if (@<Character |k| cannot be printed@>) then
            if Character_k_cannot_be_printed!(k) {
                // begin append_char("^"); append_char("^");
                append_char($globals, ASCII_code_literal!(b'^'));
                append_char($globals, ASCII_code_literal!(b'^'));
                // if k<@'100 then append_char(k+@'100)
                // else if k<@'200 then append_char(k-@'100)
                // else begin app_lc_hex(k div 16); app_lc_hex(k mod 16);
                //   end;
                // end
            } else {
                // else append_char(k);
                append_char($globals, ASCII_code_literal!(k));
            }
            // g:=make_string;
            $g = make_string($globals);
        }
        // end
        use crate::section_0042::append_char;
        use crate::section_0043::make_string;
    };
}
