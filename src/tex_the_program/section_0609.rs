//! ` `
//! When the |movement| procedure gets to the label |found|, the value of
//! |info(p)| will be either |y_here| or |z_here|. If it is, say, |y_here|,
//! the procedure generates a |y0| command (or a |w0| command), and marks
//! all |info| fields between |q| and |p| so that |y| is not OK in that range.
//
// @<Generate a |y0| or |z0| command...@>=
pub(crate) macro Generate_a_y0_or_z0_command_in_order_to_reuse_a_previous_appearance_of_w($globals:expr, $p:expr, $q:expr, $o:expr) {{
    // info(q):=info(p);
    info_inner!($globals, $q) = info_inner!($globals, $p);
    // if info(q)=y_here then
    if info_inner!($globals, $q) as integer == y_here as integer {
        // begin dvi_out(o+y0-down1); {|y0| or |w0|}
        /// `y0` or `w0`
        dvi_out!(
            $globals,
            $o.byte() as integer + y0.byte() as integer - down1.byte() as integer
        );
        // while link(q)<>p do
        while link!($globals, $q) != $p {
            // begin q:=link(q);
            $q = link!($globals, $q);
            // case info(q) of
            // yz_OK: info(q):=z_OK;
            if info_inner!($globals, $q) as integer == yz_OK as integer {
                info_inner!($globals, $q) = z_OK as _;
            }
            // y_OK: info(q):=d_fixed;
            else if info_inner!($globals, $q) as integer == y_OK as integer {
                info_inner!($globals, $q) = d_fixed as _;
            }
            // othercases do_nothing
            else {
                do_nothing!();
            }
            // endcases;
            // end;
        }
        // end
    }
    // else  begin dvi_out(o+z0-down1); {|z0| or |x0|}
    else {
        /// `z0` or `x0`
        dvi_out!(
            $globals,
            $o.byte() as integer + z0.byte() as integer - down1.byte() as integer
        );
        // while link(q)<>p do
        while link!($globals, $q) != $p {
            // begin q:=link(q);
            $q = link!($globals, $q);
            // case info(q) of
            // yz_OK: info(q):=y_OK;
            if info_inner!($globals, $q) as integer == yz_OK as integer {
                info_inner!($globals, $q) = y_OK as _;
            }
            // z_OK: info(q):=d_fixed;
            else if info_inner!($globals, $q) as integer == z_OK as integer {
                info_inner!($globals, $q) = d_fixed as _;
            }
            // othercases do_nothing
            else {
                do_nothing!();
            }
            // endcases;
            // end;
        }
        // end
    }

    use crate::pascal::integer;
    use crate::section_0016::do_nothing;
    use crate::section_0118::info_inner;
    use crate::section_0118::link;
    use crate::section_0586::down1;
    use crate::section_0586::y0;
    use crate::section_0586::z0;
    use crate::section_0598::dvi_out;
    use crate::section_0608::d_fixed;
    use crate::section_0608::y_OK;
    use crate::section_0608::y_here;
    use crate::section_0608::yz_OK;
    use crate::section_0608::z_OK;
}}
