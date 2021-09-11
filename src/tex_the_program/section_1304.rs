//! @ The user is not allowed to dump a format file unless |save_ptr=0|.
//! This condition implies that |cur_level=level_one|, hence
//! the |xeq_level| array is constant and it need not be dumped.
//
// @<If dumping is not allowed, abort@>=
pub(crate) macro If_dumping_is_not_allowed__abort($globals:expr) {{
    // if save_ptr<>0 then
    if $globals.save_ptr != 0 {
        // begin print_err("You can't dump inside a group");
        print_err!(
            $globals,
            crate::strpool_str!("You can't dump inside a group")
        );
        // @.You can't dump...@>
        // help1("`{...\dump}' is a no-no."); succumb;
        help1!($globals, crate::strpool_str!("`{...\\dump}' is a no-no."));
        succumb($globals)?;
        // end
    }
    use crate::section_0073::print_err;
    use crate::section_0079::help1;
    use crate::section_0093::succumb;
}}
