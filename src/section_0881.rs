//! @ At the end of the following code, |q| will point to the final node on the
//! list about to be justified.
//
// @<Modify the end of the line...@>=
macro_rules! Modify_the_end_of_the_line_to_reflect_the_nature_of_the_break_and_to_include_rightskip__also_set_the_proper_value_of_disc_break {
    ($globals:expr) => {{
        // q:=cur_break(cur_p); disc_break:=false; post_disc_break:=false;
        // if q<>null then {|q| cannot be a |char_node|}
        //   if type(q)=glue_node then
        //     begin delete_glue_ref(glue_ptr(q));
        //     glue_ptr(q):=right_skip;
        //     subtype(q):=right_skip_code+1; add_glue_ref(right_skip);
        //     goto done;
        //     end
        //   else  begin if type(q)=disc_node then
        //       @<Change discretionary to compulsory and set
        //         |disc_break:=true|@>
        //     else if (type(q)=math_node)or(type(q)=kern_node) then width(q):=0;
        //     end
        // else  begin q:=temp_head;
        //   while link(q)<>null do q:=link(q);
        //   end;
        // @<Put the \(r)\.{\\rightskip} glue after node |q|@>;
        // done:
        todo!("modify the end of the line");
    }}
}
