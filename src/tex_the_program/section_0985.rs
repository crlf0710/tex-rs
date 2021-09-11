//! ` `
// @d print_plus_end(#)==print(#);@+end
// @d print_plus(#)==if page_so_far[#]<>0 then
//   begin print(" plus "); print_scaled(page_so_far[#]); print_plus_end
pub(crate) macro print_plus($globals:expr, $idx:expr, $s:expr) {{
    if $globals.page_so_far[$idx] != scaled::zero() {
        print($globals, crate::strpool_str!(" plus ").get() as _);
        print_scaled($globals, $globals.page_so_far[$idx]);
        print($globals, $s.get() as _);
    }
}}

// @p procedure print_totals;
pub(crate) fn print_totals(globals: &mut TeXGlobals) {
    // begin print_scaled(page_total);
    print_scaled(globals, page_total!(globals));
    // print_plus(2)("");
    print_plus!(globals, 2, crate::strpool_str!(""));
    // print_plus(3)("fil");
    print_plus!(globals, 3, crate::strpool_str!("fil"));
    // print_plus(4)("fill");
    print_plus!(globals, 4, crate::strpool_str!("fill"));
    // print_plus(5)("filll");
    print_plus!(globals, 5, crate::strpool_str!("filll"));
    // if page_shrink<>0 then
    if page_shrink!(globals) != scaled::zero() {
        // begin print(" minus "); print_scaled(page_shrink);
        print(globals, crate::strpool_str!(" minus ").get() as _);
        print_scaled(globals, page_shrink!(globals));
        // end;
    }
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0059::print;
use crate::section_0101::scaled;
use crate::section_0103::print_scaled;
use crate::section_0982::page_shrink;
use crate::section_0982::page_total;
