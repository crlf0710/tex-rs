//! @ Here is where \.{\\lastpenalty}, \.{\\lastkern}, and \.{\\lastskip} are
//! implemented. The reference count for \.{\\lastskip} will be updated later.
//!
//! We also handle \.{\\inputlineno} and \.{\\badness} here, because they are
//! legal in similar contexts.
//
// @<Fetch an item in the current node...@>=
macro_rules! Fetch_an_item_in_the_current_node__if_appropriate {
    ($globals:expr) => {{
        // if cur_chr>glue_val then
        if $globals.cur_chr.get() as integer > last_item_command_kind::glue_val as integer {
            // begin if cur_chr=input_line_no_code then cur_val:=line
            if $globals.cur_chr.get() as integer == last_item_command_kind::input_line_no_code as integer {
                $globals.cur_val = $globals.line;
            }
            // else cur_val:=last_badness; {|cur_chr=badness_code|}
            else {
                /// `cur_chr=badness_code`
                const _ : () = ();
                $globals.cur_val = $globals.last_badness;
            }
            // cur_val_level:=int_val;
            $globals.cur_val_level = cur_val_level_kind::int_val;
            // end
        }
        // else begin if cur_chr=glue_val then cur_val:=zero_glue@+else cur_val:=0;
        else {
            todo!("fetch an item 2");
            // cur_val_level:=cur_chr;
            // if not is_char_node(tail)and(mode<>0) then
            //   case cur_chr of
            //   int_val: if type(tail)=penalty_node then cur_val:=penalty(tail);
            //   dimen_val: if type(tail)=kern_node then cur_val:=width(tail);
            //   glue_val: if type(tail)=glue_node then
            //     begin cur_val:=glue_ptr(tail);
            //     if subtype(tail)=mu_glue then cur_val_level:=mu_val;
            //     end;
            //   end {there are no other cases}
            // else if (mode=vmode)and(tail=head) then
            //   case cur_chr of
            //   int_val: cur_val:=last_penalty;
            //   dimen_val: cur_val:=last_kern;
            //   glue_val: if last_glue<>max_halfword then cur_val:=last_glue;
            //   end; {there are no other cases}
            // end
        }
        use crate::pascal::integer;
        use crate::section_0416::last_item_command_kind;
    }}
}