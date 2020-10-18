//! @ When a new token has just been fetched at |big_switch|, we have an
//! ideal place to monitor \TeX's activity.
//! @^debugging@>
//
// @<Give diagnostic information, if requested@>=
macro_rules! Give_diagnostic_information_if_requested {
    ($globals:expr, $big_switch:lifetime) => {
        // if interrupt<>0 then if OK_to_interrupt then
        if $globals.interrupt != 0 {
            if $globals.OK_to_interrupt {
                // begin back_input; check_interrupt; goto big_switch;
                back_input($globals);
                check_interrupt!($globals);
                goto_backward_label!($big_switch);
                // end;
            }
        }
        // @!debug if panicking then check_mem(false);@+@;@+gubed
        region_debug! {
            if $globals.panicking {
                check_mem($globals, false);
            }

            use crate::section_0167::check_mem;
        };
        // if tracing_commands>0 then show_cur_cmd_chr

        use crate::section_0325::back_input;
    }
}
