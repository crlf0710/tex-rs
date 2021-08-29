//! ` `

// @<Manufacture a control...@>=
macro_rules! Manufacture_a_control_sequence_name {
    ($globals:expr) => {{
        trace_span!("Manufacture a control...");
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
        // @<Look up the characters of list |r| in the hash table, and set |cur_cs|@>;
        Look_up_the_characters_of_list_r_in_the_hash_table__and_set_cur_cs!
            ($globals, p, r);
        // flush_list(r);
        flush_list($globals, r);
        // if eq_type(cur_cs)=undefined_cs then
        if eq_type!($globals, $globals.cur_cs) == undefined_cs {
            // begin eq_define(cur_cs,relax,256); {N.B.: The |save_stack| might change}
            /// N.B.: The |save_stack| might change
            const _: () = ();

            eq_define($globals, $globals.cur_cs, relax, 256)?;
            // end; {the control sequence will now match `\.{\\relax}'}
            /// the control sequence will now match `\relax}`
            const _ : () = ();
        }
        // cur_tok:=cur_cs+cs_token_flag; back_input;
        $globals.cur_tok = cur_tok_type::from_cs($globals.cur_cs);
        back_input($globals);

        // end
        use crate::section_0115::pointer;
        use crate::section_0120::get_avail;
        use crate::section_0123::flush_list;
        use crate::section_0207::relax;
        use crate::section_0208::end_cs_name;
        use crate::section_0210::undefined_cs;
        use crate::section_0277::eq_define;
        use crate::section_0297::cur_tok_type;
        use crate::section_0325::back_input;
        use crate::section_0380::get_x_token;
    }}
}
