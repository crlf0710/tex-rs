//! ` `
// @<Set init...@>=
pub(crate) macro Set_initial_values_of_key_variables_0593($globals:expr) {{
    let globals = &mut *$globals;
    // total_pages:=0; max_v:=0; max_h:=0; max_push:=0; last_bop:=-1;
    globals.total_pages = 0;
    globals.max_v = scaled::zero();
    globals.max_h = scaled::zero();
    globals.max_push = 0;
    globals.last_bop = -1;
    // doing_leaders:=false; dead_cycles:=0; cur_s:=-1;
    globals.doing_leaders = false;
    globals.dead_cycles = 0;
    globals.cur_s = -1;

    use crate::section_0101::scaled;
}}
