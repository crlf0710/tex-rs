//! ` `

// @<Manufacture a control...@>=
macro_rules! Manufacture_a_control_sequence_name {
    ($globals:expr) => {{
        /// for list manipulation
        let (mut p, mut q, r): (pointer, pointer, pointer);
        // begin r:=get_avail; p:=r; {head of the list of characters}
        r = get_avail($globals);
        /// head of the list of characters
        const _ : () = ();
        p = r;
        // repeat get_x_token;
        loop {
            get_x_token($globals)?;
            // if cur_cs=0 then store_new_token(cur_tok);
            if $globals.cur_cs == 0 {
                store_new_token!($globals, $globals.cur_tok.get(), p, q);
            }
            // until cur_cs<>0;
            if $globals.cur_cs != 0 {
                break;
            }
        }
        // if cur_cmd<>end_cs_name then @<Complain about missing \.{\\endcsname}@>;
        if $globals.cur_cmd != end_cs_name {
            todo!("complain");
        }
        todo!("manufacture");
        // @<Look up the characters of list |r| in the hash table, and set |cur_cs|@>;
        // flush_list(r);
        // if eq_type(cur_cs)=undefined_cs then
        //   begin eq_define(cur_cs,relax,256); {N.B.: The |save_stack| might change}
        //   end; {the control sequence will now match `\.{\\relax}'}
        // cur_tok:=cur_cs+cs_token_flag; back_input;
        // end
        use crate::section_0115::pointer;
        use crate::section_0120::get_avail;
        use crate::section_0208::end_cs_name;
        use crate::section_0380::get_x_token;
    }}
}
