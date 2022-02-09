//! ` `
//! Now at last we're ready to see what happens when a right brace occurs
//! in a math formula. Two special cases are simplified here: Braces are effectively
//! removed when they surround a single Ord without sub/superscripts, or when they
//! surround an accent that is the nucleus of an Ord atom.

// @<Cases of |handle...@>=
pub(crate) macro Cases_of_handle_right_brace_where_a_right_brace_triggers_a_delayed_action_1186($globals:expr) {{
    // math_group: begin unsave; decr(save_ptr);@/
    let processed = if $globals.cur_group == math_group {
        /// for short-term use
        let p;
        unsave($globals)?;
        decr!($globals.save_ptr);
        // math_type(saved(0)):=sub_mlist; p:=fin_mlist(null); info(saved(0)):=p;
        math_type!($globals, saved!($globals, 0) as pointer) = math_type_kind::sub_mlist as _;
        p = fin_mlist($globals, null);
        info_inner!($globals, saved!($globals, 0) as pointer) = p;
        // if p<>null then if link(p)=null then
        if p != null && link!($globals, p) == null {
            // if type(p)=ord_noad then
            let type_p = r#type!($globals, p);
            if type_p == ord_noad {
                // begin if math_type(subscr(p))=empty then
                //  if math_type(supscr(p))=empty then
                if math_type!($globals, subscr!(p)) == math_type_kind::empty as _
                    && math_type!($globals, supscr!(p)) == math_type_kind::empty as _
                {
                    // begin mem[saved(0)].hh:=mem[nucleus(p)].hh;
                    $globals.mem[saved!($globals, 0) as pointer][MEMORY_WORD_HH] =
                        $globals.mem[nucleus!(p)][MEMORY_WORD_HH];
                    // free_node(p,noad_size);
                    free_node($globals, p, noad_size as _);
                    // end;
                }
                // end
                use crate::section_0113::MEMORY_WORD_HH;
            }
            // else if type(p)=accent_noad then if saved(0)=nucleus(tail) then
            //  if type(tail)=ord_noad then @<Replace the tail of the list by |p|@>;
            else if type_p == accent_noad {
                if saved!($globals, 0) == nucleus!(tail!($globals)) as integer
                    && r#type!($globals, tail!($globals)) == ord_noad
                {
                    crate::section_1187::Replace_the_tail_of_the_list_by_p!($globals, p);
                }
            }
        }
        // end;
        true
    } else {
        false
    };
    use crate::pascal::integer;
    use crate::section_0016::decr;
    use crate::section_0115::null;
    use crate::section_0115::pointer;
    use crate::section_0118::info_inner;
    use crate::section_0118::link;
    use crate::section_0130::free_node;
    use crate::section_0133::r#type;
    use crate::section_0213::tail;
    use crate::section_0269::math_group;
    use crate::section_0274::saved;
    use crate::section_0281::unsave;
    use crate::section_0681::math_type;
    use crate::section_0681::math_type_kind;
    use crate::section_0681::noad_size;
    use crate::section_0681::nucleus;
    use crate::section_0681::subscr;
    use crate::section_0681::supscr;
    use crate::section_0682::ord_noad;
    use crate::section_0687::accent_noad;
    use crate::section_1184::fin_mlist;
    processed
}}
