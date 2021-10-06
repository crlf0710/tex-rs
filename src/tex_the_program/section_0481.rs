//! ` `

// @<Set init...@>=
pub(crate) macro Set_initial_values_of_key_variables_0481($globals:expr) {{
    // for k:=0 to 16 do read_open[k]:=closed;
    for k in 0..=16 {
        $globals.read_open[k] = read_open_kind::closed;
    }

    use crate::section_0480::read_open_kind;
}}
