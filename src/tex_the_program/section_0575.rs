//! @ We check to see that the \.{TFM} file doesn't end prematurely; but
//! no error message is given for files having more than |lf| words.
//
// @<Read font parameters@>=
pub(crate) macro Read_font_parameters($globals:expr, $f:expr, $z:expr, $alpha:expr, $beta:expr, $np:expr, $lbl_bad_tfm:lifetime) {{
    // begin for k:=1 to np do
    for k in 1..=$np {
        let k = k as pointer;
        // if k=1 then {the |slant| parameter is a pure number}
        if k == 1 {
            /// the `slant` parameter is a pure number
            const _: () = ();

            /// accumulators
            let mut sw: integer;

            // begin fget; sw:=fbyte; if sw>127 then sw:=sw-256;
            fget!($globals);
            sw = fbyte!($globals) as integer;
            if sw > 127 {
                sw = sw - 256;
            }
            // fget; sw:=sw*@'400+fbyte; fget; sw:=sw*@'400+fbyte;
            fget!($globals);
            sw = sw * 0o400 + fbyte!($globals) as integer;
            fget!($globals);
            sw = sw * 0o400 + fbyte!($globals) as integer;
            // fget; font_info[param_base[f]].sc:=
            //   (sw*@'20)+(fbyte div@'20);
            fget!($globals);
            $globals.font_info[$globals.param_base[$f] as pointer][MEMORY_WORD_SC] =
                scaled::new_from_inner(sw * 0o20 + fbyte!($globals) as integer / 0o20);
        // end
        }
        // else store_scaled(font_info[param_base[f]+k-1].sc);
        else {
            store_scaled!(
                $globals,
                $globals.font_info[$globals.param_base[$f] as pointer + k as pointer - 1]
                    [MEMORY_WORD_SC],
                $z.inner(),
                $alpha,
                $beta,
                $lbl_bad_tfm
            );
        }
    }
    // if eof(tfm_file) then abort;
    if eof(&mut $globals.tfm_file) {
        crate::goto_forward_label!($lbl_bad_tfm);
    }
    // for k:=np+1 to 7 do font_info[param_base[f]+k-1].sc:=0;
    for k in $np + 1..=7 {
        $globals.font_info[$globals.param_base[$f] as pointer + k as pointer - 1][MEMORY_WORD_SC] =
            scaled::zero();
    }
    // end
    use crate::pascal::eof;
    use crate::pascal::integer;
    use crate::section_0101::scaled;
    use crate::section_0101::MEMORY_WORD_SC;
    use crate::section_0115::pointer;
    use crate::section_0564::fbyte;
    use crate::section_0564::fget;
    use crate::section_0571::store_scaled;
}}
