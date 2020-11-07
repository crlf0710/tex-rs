//! @ The previous routine does not have to adjust |a| so that |a mod 4=0|,
//! since the following routines test for the \.{\\global} prefix as follows.
//
// @d global==(a>=4)
macro_rules! global {
    ($a:expr) => {
        $a >= 4
    }
}

// @d define(#)==if global then geq_define(#)@+else eq_define(#)
macro_rules! define {
    ($globals:expr, $a:expr, $p:expr, $t:expr, $v:expr) => {
        if global!($a) {
            use crate::section_0279::geq_define;
            geq_define($globals, $p, $t, $v)
        } else {
            use crate::section_0277::eq_define;
            eq_define($globals, $p, $t, $v)
        }
    }
}
// @d word_define(#)==if global then geq_word_define(#)@+else eq_word_define(#)
macro_rules! word_define {
    ($globals:expr, $a:expr, $p:expr, $v:expr) => {
        if global!($a) {
            use crate::section_0279::geq_word_define;
            geq_word_define($globals, $p, $v)
        } else {
            use crate::section_0278::eq_word_define;
            eq_word_define($globals, $p, $v)
        }
    }
}

// @<Adjust \(f)for the setting of \.{\\globaldefs}@>=
// if global_defs<>0 then
//   if global_defs<0 then
//     begin if global then a:=a-4;
//     end
//   else  begin if not global then a:=a+4;
//     end
//
