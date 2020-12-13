//! @ A specification like `\.{filllll}' or `\.{fill L L L}' will lead to two
//! error messages (one for each additional keyword \.{"l"}).
//
// @<Scan for \(f)\.{fil} units...@>=
macro_rules! Scan_for_f_fil_units_goto_attach_fraction_if_found {
    ($globals:expr, $lbl_attach_fraction:lifetime) => {{
        // if scan_keyword("fil") then
        if scan_keyword($globals, strpool_str!("fil"))? {
            // @.fil@>
            // begin cur_order:=fil;
            $globals.cur_order = glue_ord::fil;
            // while scan_keyword("l") do
            while scan_keyword($globals, strpool_str!("l"))? {
                // begin if cur_order=filll then
                if $globals.cur_order == glue_ord::filll {
                    todo!("error");
                    // begin print_err("Illegal unit of measure (");
                    // @.Illegal unit of measure@>
                    // print("replaced by filll)");
                    // help1("I dddon't go any higher than filll."); error;
                    // end
                }
                // else incr(cur_order);
                else {
                    $globals.cur_order = glue_ord::from($globals.cur_order as u8 + 1);
                }
                // end;
            }
            // goto attach_fraction;
            goto_forward_label!($lbl_attach_fraction);
            // end
        }
    }}
}
