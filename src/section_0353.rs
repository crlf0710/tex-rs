//! ` `

// @<Process an active-character...@>=
macro_rules! Process_an_active_character_control_sequence_and_set_state_mid_line {
    ($globals:expr) => {{
        trace_span!("Process an active-character...");
        // begin cur_cs:=cur_chr+active_base;
        $globals.cur_cs = ($globals.cur_chr.get() + active_base) as _;
        // cur_cmd:=eq_type(cur_cs); cur_chr:=equiv(cur_cs); state:=mid_line;
        $globals.cur_cmd = eq_type!($globals, $globals.cur_cs);
        $globals.cur_chr = cur_chr_type::new(equiv!($globals, $globals.cur_cs) as _);
        state!($globals) = mid_line;
        // if cur_cmd>=outer_call then check_outer_validity;
        if $globals.cur_cmd >= outer_call {
            check_outer_validity($globals);
        }
        // end
    }}
}