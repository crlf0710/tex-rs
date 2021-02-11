//! ` `
// @<Make sure that |pi| is in the proper range@>=
macro_rules! Make_sure_that_pi_is_in_the_proper_range {
    ($globals:expr, $pi:expr) => {{
        // if abs(pi)>=inf_penalty then
        if $pi.abs() >= inf_penalty {
            // if pi>0 then return {this breakpoint is inhibited by infinite penalty}
            if $pi > 0 {
                /// this breakpoint is inhibited by infinite penalty
                const _ : () = ();
                return_nojump!();
            }
            // else pi:=eject_penalty {this breakpoint will be forced}
            else {
                /// this breakpoint will be forced
                const _ : () = ();
                $pi = eject_penalty;
            }
        }
        use crate::section_0157::inf_penalty;
        use crate::section_0157::eject_penalty;
    }}
}
