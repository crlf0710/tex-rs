//! ` `

// @<Append the value |n| to list |p|@>=
macro_rules! Append_the_value_n_to_list_p {
    ($globals:expr, $n:expr, $p:expr) => {{
        /// used when creating a new node for list `p`
        let q: pointer;
        // begin if n<63 then
        if $n < 63 {
            // begin q:=get_avail; link(q):=p; info(q):=n; p:=q;
            q = get_avail($globals);
            link!($globals, q) = $p;
            info_inner!($globals, q) = $n.get().into();
            $p = q;
            // end;
        }
        // end
        use crate::section_0120::get_avail;
    }}
}
