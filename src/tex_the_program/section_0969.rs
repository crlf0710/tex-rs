//! ` `

// @<Insert glue for |split_top_skip|...@>=
pub(crate) macro Insert_glue_for_split_top_skip_and_set_p_to_null($globals:expr, $p:expr, $q:expr, $prev_p:expr) {{
    // begin q:=new_skip_param(split_top_skip_code); link(prev_p):=q; link(q):=p;
    //   {now |temp_ptr=glue_ptr(q)|}
    $q = new_skip_param($globals, split_top_skip_code.into())?;
    link!($globals, $prev_p) = $q;
    link!($globals, $q) = $p;
    /// now `temp_ptr=glue_ptr(q)`
    const _: () = ();
    // if width(temp_ptr)>height(p) then width(temp_ptr):=width(temp_ptr)-height(p)
    let height_p = height!($globals, $p);
    if width!($globals, $globals.temp_ptr) > height_p {
        width!($globals, $globals.temp_ptr) -= height_p;
    }
    // else width(temp_ptr):=0;
    else {
        width!($globals, $globals.temp_ptr) = scaled::zero();
    }
    // p:=null;
    $p = null;
    // end
    use crate::section_0101::scaled;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0135::height;
    use crate::section_0135::width;
    use crate::section_0154::new_skip_param;
    use crate::section_0224::split_top_skip_code;
}}
