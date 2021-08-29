//! ` `

// @<Implement \.{\\closeout}@>=
macro_rules! Implement_closeout {
    ($globals:expr) => {{
        // begin new_write_whatsit(write_node_size); write_tokens(tail):=null;
        new_write_whatsit($globals, write_node_size.into())?;
        write_tokens!($globals, tail!($globals)) = null;
        // end
        use crate::section_0115::null;
        use crate::section_1341::write_node_size;
        use crate::section_1350::new_write_whatsit;
    }}
}
