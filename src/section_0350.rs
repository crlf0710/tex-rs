//! ` `
// @<Finish line, |goto switch|@>=
macro_rules! Finish_line__goto_switch {
    ($globals:expr, $lbl_switch:lifetime) => { 
        // begin loc:=limit+1; goto switch;
        loc!($globals) = limit!($globals) + 1;
        goto_backward_label!($lbl_switch);
        // end
    }
}
