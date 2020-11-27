//! ` `

// @<Display the current context@>=
macro_rules! Display_the_current_context {
    ($globals:expr, $nn:expr) => {{
        // begin if (base_ptr=input_ptr) or (state<>token_list) or
        //    (token_type<>backed_up) or (loc<>null) then
        //     {we omit backed-up token lists that have already been read}
        /// we omit backed-up token lists that have already been read
        if $globals.base_ptr == $globals.input_ptr || state!($globals) != token_list ||
            token_type!($globals) != backed_up || loc!($globals) != null {
            // begin tally:=0; {get ready to count characters}
            /// get ready to count characters
            const _ : () = ();
            $globals.tally = 0;
            // old_setting:=selector;
            $globals.old_setting = $globals.selector;
            // if state<>token_list then
            if state!($globals) != token_list {
                todo!("pseudo print location")
                // begin @<Print location of current line@>;
                // @<Pseudoprint the line@>;
                // end
            }
            // else  begin @<Print type of token list@>;
            else {
                todo!("pseudo print token list")
                // @<Pseudoprint the token list@>;
                // end;
            }
            // selector:=old_setting; {stop pseudoprinting}
            /// stop pseudoprinting
            const _ : () = ();
            $globals.selector = $globals.old_setting;
            // @<Print two lines using the tricky pseudoprinted information@>;
            todo!("print two lines");
            // incr(nn);
            incr!($nn);
            // end;
        }
        // end
        use crate::section_0115::null;
        use crate::section_0307::backed_up;
    }}
}
