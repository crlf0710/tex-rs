//! ` `
// @<Make sure that |page_max_depth| is not exceeded@>=
pub(crate) macro Make_sure_that_page_max_depth_is_not_exceeded($globals:expr) {{
    // if page_depth>page_max_depth then
    if page_depth!($globals) > $globals.page_max_depth {
        // begin page_total:=@|
        //   page_total+page_depth-page_max_depth;@/
        page_total!($globals) += page_depth!($globals) - $globals.page_max_depth;
        // page_depth:=page_max_depth;
        page_depth!($globals) = $globals.page_max_depth;
        // end;
    }
    use crate::section_0982::page_depth;
    use crate::section_0982::page_total;
}}
