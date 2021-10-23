//! ` `
// @<Copy the tabskip glue...@>=
pub(crate) macro Copy_the_tabskip_glue_between_columns($globals:expr) {{
    // tail_append(new_glue(glue_ptr(link(cur_align))));
    tail_append!(
        $globals,
        new_glue(
            $globals,
            glue_ptr!($globals, link!($globals, $globals.cur_align)),
        )?
    );
    // subtype(tail):=tab_skip_code+1
    subtype!($globals, tail!($globals)) = tab_skip_code + 1;
    use crate::section_0118::link;
    use crate::section_0133::subtype;
    use crate::section_0149::glue_ptr;
    use crate::section_0153::new_glue;
    use crate::section_0213::tail;
    use crate::section_0214::tail_append;
    use crate::section_0224::tab_skip_code;
}}
