//! ` `
// @<Make sure that |page_max_depth| is not exceeded@>=
macro_rules! Make_sure_that_page_max_depth_is_not_exceeded {
    ($globals:expr) => {{
        // if page_depth>page_max_depth then
        if page_depth!($globals) > $globals.page_max_depth {
            // begin page_total:=@|
            //   page_total+page_depth-page_max_depth;@/
            page_total!($globals) += page_depth!($globals) - $globals.page_max_depth;
            // page_depth:=page_max_depth;
            page_depth!($globals) = $globals.page_max_depth;
            // end;
        }
    }}
}
