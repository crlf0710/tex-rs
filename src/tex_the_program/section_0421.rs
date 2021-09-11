//! @ Inside an \.{\\output} routine, a user may wish to look at the page totals
//! that were present at the moment when output was triggered.
//
// @d max_dimen==@'7777777777 {$2^{30}-1$}
/// `2^{30}-1`
pub(crate) const max_dimen: integer = 0o7777777777;

// @<Fetch something on the |page_so_far|@>=
pub(crate) macro Fetch_something_on_the_page_so_far($globals:expr, $m:expr) {{
    // begin if (page_contents=empty) and (not output_active) then
    if $globals.page_contents == page_contents_kind::empty && !$globals.output_active {
        // if m=0 then cur_val:=max_dimen@+else cur_val:=0
        if $m.get() == 0 {
            $globals.cur_val = crate::section_0421::max_dimen;
        } else {
            $globals.cur_val = 0;
        }
    }
    // else cur_val:=page_so_far[m];
    else {
        $globals.cur_val = $globals.page_so_far[$m.get() as usize].inner();
    }
    // cur_val_level:=dimen_val;
    $globals.cur_val_level = cur_val_level_kind::dimen_val;
    // end
    use crate::section_0410::cur_val_level_kind;
    use crate::section_0980::page_contents_kind;
}}

use crate::pascal::integer;
