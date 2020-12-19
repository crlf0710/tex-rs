//! @ Here we use the fact that the consecutive codes |int_val..mu_val| and
//! |assign_int..assign_mu_glue| correspond to each other nicely.
//
// @<Compute the register location |l| and its type |p|...@>=
macro_rules! Compute_the_register_location_l_and_its_type_p_but_return_if_invalid {
    ($globals:expr, $p:expr, $q:expr, $l:expr) => {{
        // begin if q<>register then
        if $q != register as pointer {
            // begin get_x_token;
            get_x_token($globals)?;
            // if (cur_cmd>=assign_int)and(cur_cmd<=assign_mu_glue) then
            if $globals.cur_cmd >= assign_int && $globals.cur_cmd <= assign_mu_glue {
                todo!("update l and p");
                // begin l:=cur_chr; p:=cur_cmd-assign_int; goto found;
                // end;
            }
            // if cur_cmd<>register then
            //   begin print_err("You can't use `"); print_cmd_chr(cur_cmd,cur_chr);
            // @.You can't use x after ...@>
            //   print("' after "); print_cmd_chr(q,0);
            //   help1("I'm forgetting what you said and not changing anything.");
            //   error; return;
            //   end;
            // end;
        }
        // p:=cur_chr; scan_eight_bit_int;
        $p = ($globals.cur_chr.get() as u8).into();
        scan_eight_bit_int($globals)?;
        // case p of
        match $p {
            // int_val: l:=cur_val+count_base;
            cur_val_level_kind::int_val => {
                $l = ($globals.cur_val + count_base as integer) as pointer;
            },
            // dimen_val: l:=cur_val+scaled_base;
            cur_val_level_kind::dimen_val => {
                $l = ($globals.cur_val + scaled_base as integer) as pointer;
            },
            // glue_val: l:=cur_val+skip_base;
            cur_val_level_kind::glue_val => {
                $l = ($globals.cur_val + skip_base as integer) as pointer;
            },
            // mu_val: l:=cur_val+mu_skip_base;
            cur_val_level_kind::mu_val => {
                $l = ($globals.cur_val + mu_skip_base as integer) as pointer;
            },
            // end; {there are no other cases}
            _ => {
                /// there are no other cases
                unreachable!();
            }
        }
        // end;
        // found:
        use crate::pascal::integer;
        use crate::section_0209::assign_int;
        use crate::section_0209::assign_mu_glue;
        use crate::section_0224::skip_base;
        use crate::section_0224::mu_skip_base;
        use crate::section_0236::count_base;
        use crate::section_0247::scaled_base;
        use crate::section_0380::get_x_token;
        use crate::section_0433::scan_eight_bit_int;
    }}
}