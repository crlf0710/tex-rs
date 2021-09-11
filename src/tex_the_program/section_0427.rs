//! ` `

// @<Fetch a register@>=
pub(crate) macro Fetch_a_register($globals:expr, $m:expr) {{
    // begin scan_eight_bit_int;
    scan_eight_bit_int($globals)?;
    // case m of
    // int_val:cur_val:=count(cur_val);
    if $m.get() == int_val as chr_code_repr {
        $globals.cur_val = count!($globals, $globals.cur_val);
    }
    // dimen_val:cur_val:=dimen(cur_val);
    else if $m.get() == dimen_val as chr_code_repr {
        $globals.cur_val = dimen!($globals, $globals.cur_val).inner();
    }
    // glue_val: cur_val:=skip(cur_val);
    else if $m.get() == glue_val as chr_code_repr {
        $globals.cur_val = skip!($globals, $globals.cur_val) as _;
    }
    // mu_val: cur_val:=mu_skip(cur_val);
    else if $m.get() == mu_val as chr_code_repr {
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
    use crate::section_0224::mu_skip;
    use crate::section_0224::skip;
    use crate::section_0236::count;
    use crate::section_0247::dimen;
    use crate::section_0297::chr_code_repr;
    use crate::section_0410::cur_val_level_kind::*;
    use crate::section_0433::scan_eight_bit_int;
}}
