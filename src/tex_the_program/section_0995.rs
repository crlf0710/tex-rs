//! ` `

// @d contrib_tail==nest[0].tail_field {tail of the contribution list}
/// tail of the contribution list
#[allow(unused_macros)]
macro_rules! contrib_tail {
    ($globals:expr) => {
        $globals.nest[0].tail_field
    }
}

// @<Make the contribution list empty...@>=
macro_rules! Make_the_contribution_list_empty_by_setting_its_tail_to_contrib_head {
    ($globals:expr) => {{
        // if nest_ptr=0 then tail:=contrib_head {vertical mode}
        if $globals.nest_ptr == 0 {
            /// vertical mode
            const _: () = ();
            tail!($globals) = contrib_head;
        }
        // else contrib_tail:=contrib_head {other modes}
        else {
            /// other modes
            const _ : () = ();
            contrib_tail!($globals) = contrib_head;
        }
    }}
}
