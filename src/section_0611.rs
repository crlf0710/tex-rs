//! @ As we search through the stack, we are in one of three states,
//! |y_seen|, |z_seen|, or |none_seen|, depending on whether we have
//! encountered |y_here| or |z_here| nodes. These states are encoded as
//! multiples of 6, so that they can be added to the |info| fields for quick
//! decision-making.
//! @^inner loop@>
//
// @d none_seen=0 {no |y_here| or |z_here| nodes have been encountered yet}
/// no `y_here` or `z_here` nodes have been encountered yet
pub(crate) const none_seen: quarterword = 0;
// @d y_seen=6 {we have seen |y_here| but not |z_here|}
/// we have seen `y_here` but not `z_here`
pub(crate) const y_seen: quarterword = 6;
// @d z_seen=12 {we have seen |z_here| but not |y_here|}
/// we have seen `z_here` but not `y_here`
pub(crate) const z_seen: quarterword = 12;

// @<Look at the other stack entries until deciding...@>=
macro_rules! Look_at_the_other_stack_entries_until_deciding_what_sort_of_DVI_command_to_generate__goto_found_if_node_p_is_a_hit {
    ($globals:expr, $q:expr, $w:expr, $mstate:expr) => {{
        /// current and top nodes on the stack
        let (mut p, _): (pointer, pointer);
        // p:=link(q); mstate:=none_seen;
        p = link!($globals, $q);
        $mstate = none_seen;
        // while p<>null do
        while p != null {
            // begin if width(p)=w then @<Consider a node with matching width;
            //   |goto found| if it's a hit@>
            if width!($globals, p) == $w {
                todo!("consider a node");
            }
            // else  case mstate+info(p) of
            else {
                let mstate_plus_info_p = $mstate + info_inner!($globals, p) as quarterword;
                // none_seen+y_here: mstate:=y_seen;
                if mstate_plus_info_p == none_seen + y_here {
                    $mstate = y_seen;
                }
                // none_seen+z_here: mstate:=z_seen;
                else if mstate_plus_info_p == none_seen + z_here {
                    $mstate = y_seen;
                }
                // y_seen+z_here,z_seen+y_here: goto not_found;
                else if mstate_plus_info_p == y_seen + z_here || mstate_plus_info_p == z_seen + y_here {
                    todo!("goto not found");
                }
                // othercases do_nothing
                else {
                    do_nothing!();
                }
                // endcases;
            }
            // p:=link(p);
            p = link!($globals, p);
            // end;
        }
        // not_found:
        use crate::section_0113::quarterword;
        use crate::section_0115::null;
        use crate::section_0608::y_here;
        use crate::section_0608::z_here;
        use crate::section_0611::none_seen;
        use crate::section_0611::y_seen;
        use crate::section_0611::z_seen;
    }}
}

use crate::section_0113::quarterword;
