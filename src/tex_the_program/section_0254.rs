//! ` `
// @<Set init...@>=
pub(crate) macro Set_initial_values_of_key_variables_0254($globals:expr) {{
    // for k:=int_base to eqtb_size do xeq_level[k]:=level_one;
    for k in int_base..=eqtb_size {
        let k = k as pointer;
        $globals.xeq_level[k] = level_one;
    }

    use crate::section_0115::pointer;
    use crate::section_0221::level_one;
    use crate::section_0230::int_base;
    use crate::section_0247::eqtb_size;
}}
