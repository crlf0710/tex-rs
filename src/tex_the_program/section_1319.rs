//! ` `
// @<Undump the hash table@>=
macro_rules! Undump_the_hash_table {
    ($globals:expr, $lbl_bad_fmt:lifetime) => {{
        /// all-purpose pointers
        let mut p: pointer;
        // undump(hash_base)(frozen_control_sequence)(hash_used); p:=hash_base-1;
        undump!(
            $globals,
            hash_base,
            frozen_control_sequence,
            $globals.hash_used,
            core::convert::identity,
            $lbl_bad_fmt
        );
        p = (hash_base - 1) as _;
        // repeat undump(p+1)(hash_used)(p); undump_hh(hash[p]);
        loop {
            undump!(
                $globals,
                p + 1,
                $globals.hash_used,
                p,
                core::convert::identity,
                $lbl_bad_fmt
            );
            undump_hh!($globals, $globals.hash[p]);
            // until p=hash_used;
            if p == $globals.hash_used {
                break;
            }
        }
        // for p:=hash_used+1 to undefined_control_sequence-1 do undump_hh(hash[p]);
        for p in $globals.hash_used + 1..=undefined_control_sequence - 1 {
            undump_hh!($globals, $globals.hash[p]);
        }
        // undump_int(cs_count)
        undump_int!($globals, $globals.cs_count);
        use crate::section_0115::pointer;
        use crate::section_0222::undefined_control_sequence;
    }};
}
