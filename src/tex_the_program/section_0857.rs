//! ` `
// @<Print the list between |printed_node| and |cur_p|...@>=
#[cfg(feature = "statistics")]
pub(crate) macro Print_the_list_between_printed_node_and_cur_p__then_set_printed_node_to_cur_p($globals:expr) {{
    // begin print_nl("");
    print_nl($globals, crate::strpool_str!(""));
    // if cur_p=null then short_display(link(printed_node))
    if $globals.cur_p == null {
        short_display($globals, link!($globals, $globals.printed_node) as _);
    }
    // else  begin save_link:=link(cur_p);
    else {
        /// temporarily holds value of `link(cur_p)`
        let save_link: pointer;
        save_link = link!($globals, $globals.cur_p);
        // link(cur_p):=null; print_nl(""); short_display(link(printed_node));
        link!($globals, $globals.cur_p) = null;
        print_nl($globals, crate::strpool_str!(""));
        short_display($globals, link!($globals, $globals.printed_node) as _);
        // link(cur_p):=save_link;
        link!($globals, $globals.cur_p) = save_link;
        // end;
    }
    // printed_node:=cur_p;
    $globals.printed_node = $globals.cur_p;
    // end
    use crate::section_0174::short_display;
}}
