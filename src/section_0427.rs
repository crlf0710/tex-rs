//! ` `

// @<Fetch a register@>=
macro_rules! Fetch_a_register {
    ($globals:expr, $m:expr) => {{
        // begin scan_eight_bit_int;
        scan_eight_bit_int($globals)?;
        // case m of
        // int_val:cur_val:=count(cur_val);
        if $m.get() == int_val as _ {
            $globals.cur_val = count!($globals, $globals.cur_val);
        }
        // dimen_val:cur_val:=dimen(cur_val);
        else if $m.get() == dimen_val as _ {
            $globals.cur_val = dimen!($globals, $globals.cur_val).inner();
        }
        // glue_val: cur_val:=skip(cur_val);
        else if $m.get() == glue_val as _ {
            $globals.cur_val = skip!($globals, $globals.cur_val) as _;
        }
        // mu_val: cur_val:=mu_skip(cur_val);
        else if $m.get() == mu_val as _ {
            $globals.cur_val = mu_skip!($globals, $globals.cur_val) as _;
        }
        // end; {there are no other cases}
        else {
            /// there are no other cases
            unreachable!();
        }
        // cur_val_level:=m;
        $globals.cur_val_level = ($m.get() as u8).into();
        // end
        use crate::section_0433::scan_eight_bit_int;
        use crate::section_0410::cur_val_level_kind::*;
    }}
}
