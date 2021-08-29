//! ` `
// @<Get ready to start...@>=
macro_rules! Get_ready_to_start_line_breaking_0834 {
    ($globals:expr) => {{
        // minimum_demerits:=awful_bad;
        $globals.minimum_demerits = awful_bad;
        // minimal_demerits[tight_fit]:=awful_bad;
        $globals.minimal_demerits[fit_class_kind::tight_fit] = awful_bad;
        // minimal_demerits[decent_fit]:=awful_bad;
        $globals.minimal_demerits[fit_class_kind::decent_fit] = awful_bad;
        // minimal_demerits[loose_fit]:=awful_bad;
        $globals.minimal_demerits[fit_class_kind::loose_fit] = awful_bad;
        // minimal_demerits[very_loose_fit]:=awful_bad;
        $globals.minimal_demerits[fit_class_kind::very_loose_fit] = awful_bad;

        use crate::section_0817::fit_class_kind;
        use crate::section_0833::awful_bad;
    }}
}
