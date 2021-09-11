//! ` `

// @<Initiate or terminate input...@>=
pub(crate) macro Initiate_or_terminate_input_from_a_file($globals:expr) {{
    // if cur_chr>0 then force_eof:=true
    if $globals.cur_chr.get() > 0 {
        $globals.force_eof = true;
    }
    // else if name_in_progress then insert_relax
    else if $globals.name_in_progress {
        insert_relax($globals);
    }
    // else start_input
    else {
        start_input($globals)?;
    }

    use crate::section_0379::insert_relax;
    use crate::section_0537::start_input;
}}
