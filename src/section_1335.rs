//! @ We get to the |final_cleanup| routine when \.{\\end} or \.{\\dump} has
//! been scanned and |its_all_over|\kern-2pt.

// @<Last-minute...@>=
// procedure final_cleanup;
#[allow(unused_variables)]
pub(crate) fn final_cleanup(globals: &mut TeXGlobals) {
    // label exit;
    // var c:small_number; {0 for \.{\\end}, 1 for \.{\\dump}}
    // begin c:=cur_chr; if c<>1 then new_line_char:=-1;
    // if job_name=0 then open_log_file;
    // while input_ptr>0 do
    //   if state=token_list then end_token_list@+else end_file_reading;
    // while open_parens>0 do
    //   begin print(" )"); decr(open_parens);
    //   end;
    // if cur_level>level_one then
    //   begin print_nl("("); print_esc("end occurred ");
    //   print("inside a group at level ");
    // @:end_}{\.{(\\end occurred...)}@>
    //   print_int(cur_level-level_one); print_char(")");
    //   end;
    // while cond_ptr<>null do
    //   begin print_nl("("); print_esc("end occurred ");
    //   print("when "); print_cmd_chr(if_test,cur_if);
    //   if if_line<>0 then
    //     begin print(" on line "); print_int(if_line);
    //     end;
    //   print(" was incomplete)");
    //   if_line:=if_line_field(cond_ptr);
    //   cur_if:=subtype(cond_ptr); temp_ptr:=cond_ptr;
    //   cond_ptr:=link(cond_ptr); free_node(temp_ptr,if_node_size);
    //   end;
    // if history<>spotless then
    //  if ((history=warning_issued)or(interaction<error_stop_mode)) then
    //   if selector=term_and_log then
    //   begin selector:=term_only;
    //   print_nl("(see the transcript file for additional information)");
    // @.see the transcript file...@>
    //   selector:=term_and_log;
    //   end;
    // if c=1 then
    //   begin @!init for c:=top_mark_code to split_bot_mark_code do
    //     if cur_mark[c]<>null then delete_token_ref(cur_mark[c]);
    //   if last_glue<>max_halfword then delete_glue_ref(last_glue);
    //   store_fmt_file; return;@+tini@/
    //   print_nl("(\dump is performed only by INITEX)"); return;
    // @:dump_}{\.{\\dump...only by INITEX}@>
    //   end;
    // exit:end;
}

use crate::section_0004::TeXGlobals;
