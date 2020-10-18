//! @ When a new token has just been fetched at |big_switch|, we have an
//! ideal place to monitor \TeX's activity.
//! @^debugging@>
//
// @<Give diagnostic information, if requested@>=
macro_rules! Give_diagnostic_information_if_requested {
    ($globals:expr) => {
        // if interrupt<>0 then if OK_to_interrupt then
        //   begin back_input; check_interrupt; goto big_switch;
        //   end;
        // @!debug if panicking then check_mem(false);@+@;@+gubed
        // if tracing_commands>0 then show_cur_cmd_chr
    }
}
