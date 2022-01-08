//! ` `
//! A box made from spanned columns will be followed by tabskip glue nodes and
//! by empty boxes as if there were no spanning. This permits perfect alignment
//! of subsequent entries, and it prevents values that depend on floating point
//! arithmetic from entering into the dimensions of any boxes.
//
// @<Set the glue in node |r|...@>=
pub(crate) macro Set_the_glue_in_node_r_and_change_it_from_an_unset_node($globals:expr, $q:expr, $r:expr, $s:expr) {{
    /// registers for the list operations
    let (mut u,);
    /// width of column
    let (t, w);
    /// matching span amount
    let mut n;
    // n:=span_count(r); t:=width(s); w:=t; u:=hold_head;
    n = span_count!($globals, $r);
    t = width!($globals, $s);
    w = t;
    u = hold_head;
    // while n>min_quarterword do
    while n > min_quarterword {
        // begin decr(n);
        decr!(n);
        // @<Append tabskip glue and an empty box to list |u|,
        //   and update |s| and |t| as the prototype nodes are passed@>;
        todo!("Append tabskip glue");
        // end;
    }
    // if mode=-vmode then
    if mode!($globals) == -vmode {
        // @<Make the unset node |r| into an |hlist_node| of width |w|,
        //   setting the glue as if the width were |t|@>
        crate::section_0810::Make_the_unset_node_r_into_an_hlist_node_of_width_w__setting_the_glue_as_if_the_width_were_t!($globals, $q, $r, w, t);
    }
    // else @<Make the unset node |r| into a |vlist_node| of height |w|,
    //     setting the glue as if the height were |t|@>;
    else {
        crate::section_0811::Make_the_unset_node_r_into_a_vlist_node_of_height_w__setting_the_glue_as_if_the_height_were_t!($globals, $q, $r, w, t);
    }
    // shift_amount(r):=0;
    shift_amount!($globals, $r) = scaled::zero();
    // if u<>hold_head then {append blank boxes to account for spanned nodes}
    if u != hold_head {
        /// append blank boxes to account for spanned nodes
        const _: () = ();
        // begin link(u):=link(r); link(r):=link(hold_head); r:=u;
        link!($globals, u) = link!($globals, $r);
        link!($globals, $r) = link!($globals, hold_head);
        $r = u;
        // end
    }
    use crate::section_0016::decr;
    use crate::section_0101::scaled;
    use crate::section_0110::min_quarterword;
    use crate::section_0118::link;
    use crate::section_0135::shift_amount;
    use crate::section_0135::width;
    use crate::section_0159::span_count;
    use crate::section_0162::hold_head;
    use crate::section_0211::vmode;
    use crate::section_0213::mode;
}}
