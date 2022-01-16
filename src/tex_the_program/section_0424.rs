//! @ Here is where \.{\\lastpenalty}, \.{\\lastkern}, and \.{\\lastskip} are
//! implemented. The reference count for \.{\\lastskip} will be updated later.
//!
//! We also handle \.{\\inputlineno} and \.{\\badness} here, because they are
//! legal in similar contexts.
//
// @<Fetch an item in the current node...@>=
pub(crate) macro Fetch_an_item_in_the_current_node__if_appropriate($globals:expr) {{
    // if cur_chr>glue_val then
    if $globals.cur_chr.get() as integer > last_item_command_kind::glue_val as integer {
        // begin if cur_chr=input_line_no_code then cur_val:=line
        if $globals.cur_chr.get() as integer
            == last_item_command_kind::input_line_no_code as integer
        {
            $globals.cur_val = $globals.line;
        }
        // else cur_val:=last_badness; {|cur_chr=badness_code|}
        else {
            /// `cur_chr=badness_code`
            const _: () = ();
            $globals.cur_val = $globals.last_badness;
        }
        // cur_val_level:=int_val;
        $globals.cur_val_level = cur_val_level_kind::int_val;
        // end
    }
    // else begin if cur_chr=glue_val then cur_val:=zero_glue@+else cur_val:=0;
    else {
        if $globals.cur_chr.get() == last_item_command_kind::glue_val as _ {
            $globals.cur_val = zero_glue as _;
        } else {
            $globals.cur_val = 0;
        }
        // cur_val_level:=cur_chr;
        $globals.cur_val_level = cur_val_level_kind::from($globals.cur_chr.get() as u8);
        // if not is_char_node(tail)and(mode<>0) then
        if !is_char_node!($globals, tail!($globals)) && mode!($globals) != 0 {
            // case cur_chr of
            // int_val: if type(tail)=penalty_node then cur_val:=penalty(tail);
            // dimen_val: if type(tail)=kern_node then cur_val:=width(tail);
            // glue_val: if type(tail)=glue_node then
            //   begin cur_val:=glue_ptr(tail);
            //   if subtype(tail)=mu_glue then cur_val_level:=mu_val;
            //   end;
            // end {there are no other cases}
            todo!("fetch an item 2a");
        }
        // else if (mode=vmode)and(tail=head) then
        else if mode!($globals) == vmode && tail!($globals) == head!($globals) {
            // case cur_chr of
            // int_val: cur_val:=last_penalty;
            if $globals.cur_chr.get() == last_item_command_kind::int_val as _ {
                $globals.cur_val = $globals.last_penalty;
            }
            // dimen_val: cur_val:=last_kern;
            else if $globals.cur_chr.get() == last_item_command_kind::dimen_val as _ {
                $globals.cur_val = $globals.last_kern.inner();
            }
            // glue_val: if last_glue<>max_halfword then cur_val:=last_glue;
            else if $globals.cur_chr.get() == last_item_command_kind::glue_val as _ {
                if $globals.last_glue != max_halfword {
                    $globals.cur_val = $globals.last_glue as _;
                }
            } else {
                // end; {there are no other cases}
                /// there are no other cases
                unreachable!();
            }
        }
        // end
    }
    use crate::pascal::integer;
    use crate::section_0110::max_halfword;
    use crate::section_0134::is_char_node;
    use crate::section_0162::zero_glue;
    use crate::section_0211::vmode;
    use crate::section_0213::head;
    use crate::section_0213::mode;
    use crate::section_0213::tail;
    use crate::section_0410::cur_val_level_kind;
    use crate::section_0416::last_item_command_kind;
}}
