//! ` `

// @<Pseudoprint the token list@>=
macro_rules! Pseudoprint_the_token_list {
    ($globals:expr, $l:expr) => {{
        // begin_pseudoprint;
        begin_pseudoprint!($globals, $l);
        // if token_type<macro then show_token_list(start,loc,100000)
        if token_type!($globals) < r#macro {
            show_token_list($globals, start!($globals) as _, loc!($globals) as _, 100000);
        }
        // else show_token_list(link(start),loc,100000) {avoid reference count}
        else {
            /// avoid reference count
            const _: () = ();
            show_token_list(
                $globals,
                link!($globals, start!($globals)) as _,
                loc!($globals) as _,
                100000,
            );
        }

        use crate::section_0292::show_token_list;
        use crate::section_0307::r#macro;
    }};
}
