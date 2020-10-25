//! ` `

// @d contrib_tail==nest[0].tail_field {tail of the contribution list}
/// tail of the contribution list
macro_rules! contrib_tail {
    ($globals:expr) => {
        $globals.nest[0].tail_field
    }
}

// @<Make the contribution list empty...@>=
// if nest_ptr=0 then tail:=contrib_head {vertical mode}
// else contrib_tail:=contrib_head {other modes}
//
