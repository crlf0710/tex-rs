//! ` `

// @<Nullify |width(q)| and the tabskip glue following this column@>=
pub(crate) macro Nullify_width_q_and_the_tabskip_glue_following_this_column($globals:expr, $q:expr) {{
    /// registers for the list operations
    let (r, s);
    // begin width(q):=0; r:=link(q); s:=glue_ptr(r);
    width!($globals, $q) = scaled::zero();
    r = link!($globals, $q);
    s = glue_ptr!($globals, r);
    // if s<>zero_glue then
    if s != zero_glue {
        // begin add_glue_ref(zero_glue); delete_glue_ref(s);
        add_glue_ref!($globals, zero_glue);
        delete_glue_ref($globals, s);
        // glue_ptr(r):=zero_glue;
        glue_ptr!($globals, r) = zero_glue;
        // end;
    }
    // end
    use crate::section_0101::scaled;
    use crate::section_0118::link;
    use crate::section_0135::width;
    use crate::section_0149::glue_ptr;
    use crate::section_0162::zero_glue;
    use crate::section_0201::delete_glue_ref;
    use crate::section_0203::add_glue_ref;
}}
