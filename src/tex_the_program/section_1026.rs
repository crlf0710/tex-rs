//! @ When the user's output routine finishes, it has constructed a vlist
//! in internal vertical mode, and \TeX\ will do the following:
//
// @<Resume the page builder after an output routine has come to an end@>=
pub(crate) macro Resume_the_page_builder_after_an_output_routine_has_come_to_an_end($globals:expr) {{
    // begin if (loc<>null) or
    //  ((token_type<>output_text)and(token_type<>backed_up)) then
    if loc!($globals) != null
        || (token_type!($globals) != output_text && token_type!($globals) != backed_up)
    {
        // @<Recover from an unbalanced output routine@>;
        todo!("recover");
    }
    // end_token_list; {conserve stack space in case more outputs are triggered}
    /// conserve stack space in case more outputs are triggered
    const _: () = ();
    end_token_list($globals);
    // end_graf; unsave; output_active:=false; insert_penalties:=0;@/
    end_graf($globals)?;
    unsave($globals)?;
    $globals.output_active = false;
    $globals.insert_penalties = 0;
    // @<Ensure that box 255 is empty after output@>;
    crate::section_1028::Ensure_that_box_255_is_empty_after_output!($globals);
    // if tail<>head then {current list goes after heldover insertions}
    if tail!($globals) != head!($globals) {
        /// current list goes after heldover insertions
        const _: () = ();
        // begin link(page_tail):=link(head);
        link!($globals, $globals.page_tail) = link!($globals, head!($globals));
        // page_tail:=tail;
        $globals.page_tail = tail!($globals);
        // end;
    }
    // if link(page_head)<>null then {and both go before heldover contributions}
    if link!($globals, page_head) != null {
        /// and both go before heldover contributions
        const _: () = ();
        // begin if link(contrib_head)=null then contrib_tail:=page_tail;
        if link!($globals, contrib_head) == null {
            contrib_tail!($globals) = $globals.page_tail;
        }
        // link(page_tail):=link(contrib_head);
        link!($globals, $globals.page_tail) = link!($globals, contrib_head);
        // link(contrib_head):=link(page_head);
        link!($globals, contrib_head) = link!($globals, page_head);
        // link(page_head):=null; page_tail:=page_head;
        link!($globals, page_head) = null;
        $globals.page_tail = page_head;
        // end;
    }
    // pop_nest; build_page;
    pop_nest($globals);
    build_page($globals)?;
    // end
    use crate::section_0036::loc;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0162::contrib_head;
    use crate::section_0162::page_head;
    use crate::section_0213::head;
    use crate::section_0213::tail;
    use crate::section_0217::pop_nest;
    use crate::section_0281::unsave;
    use crate::section_0307::backed_up;
    use crate::section_0307::output_text;
    use crate::section_0307::token_type;
    use crate::section_0324::end_token_list;
    use crate::section_0994::build_page;
    use crate::section_0995::contrib_tail;
    use crate::section_1096::end_graf;
}}
