//! ` `
// @<Read extensible character recipes@>=
macro_rules! Read_extensible_character_recipes {
    ($globals:expr, $f:expr, $bc:expr, $ec:expr, $lbl_bad_tfm:lifetime) => {{ 
        // for k:=exten_base[f] to param_base[f]-1 do
        for k in $globals.exten_base[$f]..= $globals.param_base[$f] - 1 {
            let k = k as pointer;
            let qqqq;
            // begin store_four_quarters(font_info[k].qqqq);
            store_four_quarters!($globals, qqqq);
            $globals.font_info[k][MEMORY_WORD_QQQQ] = qqqq;
            // if a<>0 then check_existence(a);
            if qqqq[FOUR_QUARTERS_B0] != 0 {
                check_existence!($globals, $f, qqqq[FOUR_QUARTERS_B0], $bc, $ec, $lbl_bad_tfm);
            }
            // if b<>0 then check_existence(b);
            if qqqq[FOUR_QUARTERS_B1] != 0 {
                check_existence!($globals, $f, qqqq[FOUR_QUARTERS_B1], $bc, $ec, $lbl_bad_tfm);
            }
            // if c<>0 then check_existence(c);
            if qqqq[FOUR_QUARTERS_B2] != 0 {
                check_existence!($globals, $f, qqqq[FOUR_QUARTERS_B2], $bc, $ec, $lbl_bad_tfm);
            }
            // check_existence(d);
            check_existence!($globals, $f, qqqq[FOUR_QUARTERS_B3], $bc, $ec, $lbl_bad_tfm);
            // end
        }

        use crate::section_0113::MEMORY_WORD_QQQQ;
        use crate::section_0113::FOUR_QUARTERS_B0;
        use crate::section_0113::FOUR_QUARTERS_B1;
        use crate::section_0113::FOUR_QUARTERS_B2;
        use crate::section_0113::FOUR_QUARTERS_B3;
    }}
}
