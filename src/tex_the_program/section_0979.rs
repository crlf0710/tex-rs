//! @ It's possible that the box begins with a penalty node that is the
//! ``best'' break, so we must be careful to handle this special case correctly.
//
// @<Look at all the marks...@>=
pub(crate) macro Look_at_all_the_marks_in_nodes_before_the_break__and_set_the_final_link_to_null_at_the_break($globals:expr, $p:expr, $q:expr, $v:expr) {{
    crate::region_forward_label! {
        |'done|
        {
            // p:=list_ptr(v);
            $p = list_ptr!($globals, $v);
            // if p=q then list_ptr(v):=null
            if $p == $q {
                list_ptr!($globals, $v) = null;
            }
            // else loop@+begin if type(p)=mark_node then
            else {
                loop {
                    if r#type!($globals, $p) == mark_node {
                        // if split_first_mark=null then
                        //   begin split_first_mark:=mark_ptr(p);
                        //   split_bot_mark:=split_first_mark;
                        //   token_ref_count(split_first_mark):=@|
                        //     token_ref_count(split_first_mark)+2;
                        //   end
                        // else  begin delete_token_ref(split_bot_mark);
                        //   split_bot_mark:=mark_ptr(p);
                        //   add_token_ref(split_bot_mark);
                        //   end;
                        todo!("mark_node");
                    }
                    // if link(p)=q then
                    if link!($globals, $p) == $q {
                        // begin link(p):=null; goto done;
                        link!($globals, $p) = null;
                        crate::goto_forward_label!('done);
                        // end;
                    }
                    // p:=link(p);
                    $p = link!($globals, $p);
                    // end;
                }
            }
        }
        // done:
        'done <-
    }
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0133::r#type;
    use crate::section_0135::list_ptr;
    use crate::section_0141::mark_node;
}}
