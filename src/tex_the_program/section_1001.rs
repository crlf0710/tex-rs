//! ` `
// @<Initialize the current page, insert the \.{\\topskip} glue...@>=
pub(crate) macro Initialize_the_current_page__insert_the_topskip_glue_ahead_of_p__and_goto_continue($globals:expr, $p:expr, $lbl_continue:lifetime) {{
    let q: pointer;
    // begin if page_contents=empty then freeze_page_specs(box_there)
    if $globals.page_contents == page_contents_kind::empty {
        freeze_page_specs($globals, page_contents_kind::box_there);
    }
    // else page_contents:=box_there;
    else {
        $globals.page_contents = page_contents_kind::box_there;
    }
    // q:=new_skip_param(top_skip_code); {now |temp_ptr=glue_ptr(q)|}
    q = new_skip_param($globals, top_skip_code.into())?;
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
    // link(q):=p; link(contrib_head):=q; goto continue;
    link!($globals, q) = $p;
    link!($globals, contrib_head) = q;
    crate::goto_backward_label!($lbl_continue);
    // end
    use crate::section_0101::scaled;
    use crate::section_0115::pointer;
    use crate::section_0118::link;
    use crate::section_0135::height;
    use crate::section_0135::width;
    use crate::section_0154::new_skip_param;
    use crate::section_0162::contrib_head;
    use crate::section_0224::top_skip_code;
    use crate::section_0980::page_contents_kind;
    use crate::section_0987::freeze_page_specs;
}}
