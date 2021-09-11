//! ` `

// @<Display the current context@>=
pub(crate) macro Display_the_current_context($globals:expr, $nn:expr, $old_setting:expr) {{
    /// length of descriptive information on line 1
    let l: u8_from_0_to_n<U255>;

    // begin if (base_ptr=input_ptr) or (state<>token_list) or
    //    (token_type<>backed_up) or (loc<>null) then
    //     {we omit backed-up token lists that have already been read}
    /// we omit backed-up token lists that have already been read
    if $globals.base_ptr == $globals.input_ptr
        || state!($globals) != token_list
        || token_type!($globals) != backed_up
        || loc!($globals) != null
    {
        // begin tally:=0; {get ready to count characters}
        /// get ready to count characters
        const _: () = ();
        $globals.tally = 0;
        // old_setting:=selector;
        $old_setting = $globals.selector;
        // if state<>token_list then
        if state!($globals) != token_list {
            // begin @<Print location of current line@>;
            crate::section_0313::Print_location_of_current_line!($globals);
            // @<Pseudoprint the line@>;
            crate::section_0318::Pseudoprint_the_line!($globals, l);
            // end
        }
        // else  begin @<Print type of token list@>;
        else {
            crate::section_0314::Print_type_of_token_list!($globals);
            // @<Pseudoprint the token list@>;
            crate::section_0319::Pseudoprint_the_token_list!($globals, l);
            // end;
        }
        // selector:=old_setting; {stop pseudoprinting}
        /// stop pseudoprinting
        const _: () = ();
        $globals.selector = $old_setting;
        // @<Print two lines using the tricky pseudoprinted information@>;
        crate::section_0317::Print_two_lines_using_the_tricky_pseudoprinted_information!(
            $globals, l
        );
        // incr(nn);
        incr!($nn);
        // end;
    }
    // end
    use crate::pascal::u8_from_0_to_n;
    use crate::section_0016::incr;
    use crate::section_0036::loc;
    use crate::section_0115::null;
    use crate::section_0302::state;
    use crate::section_0307::backed_up;
    use crate::section_0307::token_list;
    use crate::section_0307::token_type;
    use typenum::U255;
}}
