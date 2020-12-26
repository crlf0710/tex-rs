//! @ Now comes the harder part: At this point in the program, |cur_val| is a
//! nonnegative integer and $f/2^{16}$ is a nonnegative fraction less than 1;
//! we want to multiply the sum of these two quantities by the appropriate
//! factor, based on the specified units, in order to produce a |scaled|
//! result, and we want to do the calculation with fixed point arithmetic that
//! does not overflow.

//
// @<Scan units and set |cur_val| to $x\cdot(|cur_val|+f/2^{16})$...@>=
macro_rules! Scan_units_and_set_cur_val_to_x_dot_cur_val_f_2_16_where_there_are_x_sp_per_unit__goto_attach_sign_if_the_units_are_internal {
    ($globals:expr, $mu:expr, $inf:expr, $f:expr, $lbl_attach_sign:lifetime) => {{
        region_forward_label!(
        |'done|
        {
        region_forward_label!(
        |'attach_fraction|
        {
        // if inf then @<Scan for \(f)\.{fil} units; |goto attach_fraction| if found@>;
        if $inf {
            Scan_for_f_fil_units_goto_attach_fraction_if_found!($globals, 'attach_fraction);
        }
        // @<Scan for \(u)units that are internal dimensions;
        //   |goto attach_sign| with |cur_val| set if found@>;
        Scan_for_u_units_that_are_internal_dimensions__goto_attach_sign_with_cur_val_set_if_found!
            ($globals, $f, $mu, $lbl_attach_sign);
        // if mu then @<Scan for \(m)\.{mu} units and |goto attach_fraction|@>;
        if $mu {
            Scan_for_m_mu_units_and_goto_attach_fraction!($globals, 'attach_fraction);
        }
        // if scan_keyword("true") then @<Adjust \(f)for the magnification ratio@>;
        if scan_keyword($globals, strpool_str!("true"))? {
            todo!("magnification");
        }
        // @.true@>
        // if scan_keyword("pt") then goto attach_fraction; {the easy case}
        if scan_keyword($globals, strpool_str!("pt"))? {
            goto_forward_label!('attach_fraction);
        }
        // @.pt@>
        // @<Scan for \(a)all other units and adjust |cur_val| and |f| accordingly;
        //   |goto done| in the case of scaled points@>;
        Scan_for_a_all_other_units_and_adjust_cur_val_and_f_accordingly__goto_done_in_the_case_of_scaled_points!
            ($globals, $f, 'done);
        }
        // attach_fraction: if cur_val>=@'40000 then arith_error:=true
        'attach_fraction <-
        );
        if $globals.cur_val >= 0o40000 {
            $globals.arith_error = true;
        }
        // else cur_val:=cur_val*unity+f;
        else {
            $globals.cur_val = $globals.cur_val * unity.inner() + $f;
        }
        }
        // done:
        'done <-
        );
        use crate::section_0101::unity;
        use crate::section_0407::scan_keyword;
    }}
}
