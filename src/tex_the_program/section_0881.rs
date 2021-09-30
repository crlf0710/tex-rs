//! @ At the end of the following code, |q| will point to the final node on the
//! list about to be justified.
//
// @<Modify the end of the line...@>=
pub(crate) macro Modify_the_end_of_the_line_to_reflect_the_nature_of_the_break_and_to_include_rightskip__also_set_the_proper_value_of_disc_break($globals:expr, $q:expr, $disc_break:expr, $post_disc_break:expr) {{
    crate::region_forward_label! {
        |'done|
        {
            // q:=cur_break(cur_p); disc_break:=false; post_disc_break:=false;
            $q = cur_break!($globals, $globals.cur_p);
            $disc_break = false;
            $post_disc_break = false;
            // if q<>null then {|q| cannot be a |char_node|}
            if $q != null {
                /// `q` cannot be a `char_node`
                const _: () = ();
                // if type(q)=glue_node then
                if r#type!($globals, $q) == glue_node {
                    // begin delete_glue_ref(glue_ptr(q));
                    delete_glue_ref($globals, glue_ptr!($globals, $q));
                    // glue_ptr(q):=right_skip;
                    glue_ptr!($globals, $q) = right_skip!($globals);
                    // subtype(q):=right_skip_code+1; add_glue_ref(right_skip);
                    subtype!($globals, $q) = right_skip_code + 1;
                    add_glue_ref!($globals, right_skip!($globals));
                    // goto done;
                    crate::goto_forward_label!('done);
                    // end
                }
                // else  begin if type(q)=disc_node then
                else {
                    if r#type!($globals, $q) == disc_node {
                        // @<Change discretionary to compulsory and set
                        //   |disc_break:=true|@>
                        crate::section_0882::Change_discretionary_to_compulsory_and_set_disc_break_to_true!($globals, $q, $disc_break);
                    }
                    // else if (type(q)=math_node)or(type(q)=kern_node) then width(q):=0;
                    else if r#type!($globals, $q) == math_node || r#type!($globals, $q) == kern_node {
                        width!($globals, $q) = scaled::zero();
                    }
                    // end
                }
            }
            // else  begin q:=temp_head;
            else {
                $q = temp_head;
                // while link(q)<>null do q:=link(q);
                while link!($globals, $q) != null {
                    $q = link!($globals, $q);
                }
                // end;
            }
            // @<Put the \(r)\.{\\rightskip} glue after node |q|@>;
            crate::section_0886::Put_the_rightskip_glue_after_node_q!($globals, $q);
        }
        // done:
        'done <-
    };
    use crate::section_0101::scaled;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0133::r#type;
    use crate::section_0133::subtype;
    use crate::section_0135::width;
    use crate::section_0145::disc_node;
    use crate::section_0147::math_node;
    use crate::section_0149::glue_node;
    use crate::section_0149::glue_ptr;
    use crate::section_0155::kern_node;
    use crate::section_0162::temp_head;
    use crate::section_0201::delete_glue_ref;
    use crate::section_0203::add_glue_ref;
    use crate::section_0224::right_skip;
    use crate::section_0224::right_skip_code;
    use crate::section_0821::cur_break;
}}
