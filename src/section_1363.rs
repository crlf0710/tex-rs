//! ` `
//@<Advance \(p)past a whatsit node in the \(l)|line_break| loop@>=@+
macro_rules! Advance_past_a_whatsit_node_in_the_line_break_loop {
    ($globals:expr) => {{
        // adv_past(cur_p)
        adv_past!($globals, $globals.cur_p);
    }}
}
