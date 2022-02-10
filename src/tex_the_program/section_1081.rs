//! ` `

// @<Remove the last box...@>=
pub(crate) macro Remove_the_last_box__unless_its_part_of_a_discretionary($globals:expr) {{
    /// run through the current list
    let (mut p, mut q);
    // begin q:=head;
    q = head!($globals);
    crate::region_forward_label! {
        |'done|
        {
            // repeat p:=q;
            loop {
                p = q;
                // if not is_char_node(q) then if type(q)=disc_node then
                if !is_char_node!($globals, q) && r#type!($globals, q) == disc_node {
                    // begin for m:=1 to replace_count(q) do p:=link(p);
                    for _ in 1..=replace_count!($globals, q) {
                        p = link!($globals, p);
                    }
                    // if p=tail then goto done;
                    if p == tail!($globals) {
                        crate::goto_forward_label!('done);
                    }
                    // end;
                }
                // q:=link(p);
                q = link!($globals, p);
                // until q=tail;
                if q == tail!($globals) {
                    break;
                }
            }
            // cur_box:=tail; shift_amount(cur_box):=0;
            $globals.cur_box = tail!($globals);
            shift_amount!($globals, $globals.cur_box) = scaled::zero();
            // tail:=p; link(p):=null;
            tail!($globals) = p;
            link!($globals, p) = null;
        }
        // done:end
        'done <-
    }
    use crate::section_0101::scaled;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0133::r#type;
    use crate::section_0134::is_char_node;
    use crate::section_0135::shift_amount;
    use crate::section_0145::disc_node;
    use crate::section_0145::replace_count;
    use crate::section_0213::head;
    use crate::section_0213::tail;
}}
