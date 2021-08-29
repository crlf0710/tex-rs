//! @ At the end of the following code, |q| will point to the final node on the
//! list about to be justified.
//
// @<Modify the end of the line...@>=
macro_rules! Modify_the_end_of_the_line_to_reflect_the_nature_of_the_break_and_to_include_rightskip__also_set_the_proper_value_of_disc_break {
    ($globals:expr, $q:expr, $disc_break:expr, $post_disc_break:expr) => {{
        // q:=cur_break(cur_p); disc_break:=false; post_disc_break:=false;
        $q = cur_break!($globals, $globals.cur_p);
        $disc_break = false;
        $post_disc_break = false;
        // if q<>null then {|q| cannot be a |char_node|}
        if $q != null {
            /// `q` cannot be a `char_node`
            const _ : () = ();
            // if type(q)=glue_node then
            if r#type!($globals, $q) == glue_node {
                // begin delete_glue_ref(glue_ptr(q));
                // glue_ptr(q):=right_skip;
                // subtype(q):=right_skip_code+1; add_glue_ref(right_skip);
                // goto done;
                // end
                todo!("type(q)=glue_node");
            }
            // else  begin if type(q)=disc_node then
            else {
                //   @<Change discretionary to compulsory and set
                //     |disc_break:=true|@>
                // else if (type(q)=math_node)or(type(q)=kern_node) then width(q):=0;
                // end
                todo!("type(q)!=glue_node");
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
        Put_the_rightskip_glue_after_node_q!($globals, $q);
        // done:
        use crate::section_0149::glue_node;
    }}
}
