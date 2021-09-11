//! ` `
// @<Fetch the |dead_cycles| or the |insert_penalties|@>=
pub(crate) macro Fetch_the_dead_cycles_or_the_insert_penalties($globals:expr, $m:expr) {{
    // begin if m=0 then cur_val:=dead_cycles@+else cur_val:=insert_penalties;
    if $m.get() == 0 {
        $globals.cur_val = $globals.dead_cycles;
    } else {
        $globals.cur_val = $globals.insert_penalties;
    }
    // cur_val_level:=int_val;
    $globals.cur_val_level = cur_val_level_kind::int_val;
    // end
    use crate::section_0410::cur_val_level_kind;
}}
