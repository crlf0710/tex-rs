//! @ The page builder is ready to start a fresh page if we initialize
//! the following state variables. (However, the page insertion list is initialized
//! elsewhere.)
//
// @<Start a new current page@>=
macro_rules! Start_a_new_current_page {
    ($globals:expr) => {{
        // page_contents:=empty; page_tail:=page_head; link(page_head):=null;@/
        $globals.page_contents = page_contents_kind::empty;
        $globals.page_tail = page_head;
        link!($globals, page_head) = null;
        // last_glue:=max_halfword; last_penalty:=0; last_kern:=0;
        $globals.last_glue = max_halfword;
        $globals.last_penalty = 0;
        $globals.last_kern = scaled::zero();
        // page_depth:=0; page_max_depth:=0
        page_depth!($globals) = scaled::zero();
        $globals.page_max_depth = scaled::zero();

        use crate::section_0101::scaled;
        use crate::section_0110::max_halfword;
        use crate::section_0115::null;
        use crate::section_0162::page_head;
        use crate::section_0980::page_contents_kind;
    }}
}
