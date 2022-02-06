//! ` `
// @d choose_mlist(#)==begin p:=#(q); #(q):=null;@+end
pub(self) macro choose_mlist($globals:expr, $m:ident, $p:expr, $q:expr) {{
    $p = $m!($globals, $q);
    $m!($globals, $q) = null;
    use crate::section_0115::null;
}}

// @<Change this node to a style node...@>=
pub(crate) macro Change_this_node_to_a_style_node_followed_by_the_correct_choice__then_goto_done_with_node($globals:expr, $q:expr, $lbl_done_with_node:lifetime) {{
    /// temporary registers for list construction
    let mut p;
    // begin case cur_style div 2 of
    let cur_style_div_2 = $globals.cur_style.get() / 2;
    // 0: choose_mlist(display_mlist); {|display_style=0|}
    if cur_style_div_2 == 0 {
        /// `display_style=0`
        const _: () = ();
        choose_mlist!($globals, display_mlist, p, $q);
    }
    // 1: choose_mlist(text_mlist); {|text_style=2|}
    else if cur_style_div_2 == 1 {
        /// `text_style=2`
        const _: () = ();
        choose_mlist!($globals, text_mlist, p, $q);
    }
    // 2: choose_mlist(script_mlist); {|script_style=4|}
    else if cur_style_div_2 == 2 {
        /// `script_style=4`
        const _: () = ();
        choose_mlist!($globals, script_mlist, p, $q);
    }
    // 3: choose_mlist(script_script_mlist); {|script_script_style=6|}
    else if cur_style_div_2 == 3 {
        /// `script_script_style=6`
        const _: () = ();
        choose_mlist!($globals, script_script_mlist, p, $q);
    }
    // end; {there are no other cases}
    else {
        /// there are no other cases
        const _: () = ();
        unreachable!();
    }
    // flush_node_list(display_mlist(q));
    flush_node_list($globals, display_mlist!($globals, $q))?;
    // flush_node_list(text_mlist(q));
    flush_node_list($globals, text_mlist!($globals, $q))?;
    // flush_node_list(script_mlist(q));
    flush_node_list($globals, script_mlist!($globals, $q))?;
    // flush_node_list(script_script_mlist(q));@/
    flush_node_list($globals, script_script_mlist!($globals, $q))?;
    // type(q):=style_node; subtype(q):=cur_style; width(q):=0; depth(q):=0;
    r#type!($globals, $q) = style_node;
    subtype!($globals, $q) = $globals.cur_style.get();
    width!($globals, $q) = scaled::zero();
    depth!($globals, $q) = scaled::zero();
    // if p<>null then
    if p != null {
        /// temporary registers for list construction
        let z;
        // begin z:=link(q); link(q):=p;
        z = link!($globals, $q);
        link!($globals, $q) = p;
        // while link(p)<>null do p:=link(p);
        while link!($globals, p) != null {
            p = link!($globals, p);
        }
        // link(p):=z;
        link!($globals, p) = z;
        // end;
    }
    // goto done_with_node;
    crate::goto_forward_label!($lbl_done_with_node);
    // end
    use crate::section_0101::scaled;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0133::r#type;
    use crate::section_0133::subtype;
    use crate::section_0135::depth;
    use crate::section_0135::width;
    use crate::section_0202::flush_node_list;
    use crate::section_0688::style_node;
    use crate::section_0689::display_mlist;
    use crate::section_0689::script_mlist;
    use crate::section_0689::script_script_mlist;
    use crate::section_0689::text_mlist;
}}
