//! @ There's also a |fast_get_avail| routine, which saves the procedure-call
//! overhead at the expense of extra programming. This routine is used in
//! the places that would otherwise account for the most calls of |get_avail|.
//! @^inner loop@>
//
// @d fast_get_avail(#)==@t@>@;@/
macro_rules! fast_get_avail {
    ($globals:expr, $val:expr) => {{
        // begin #:=avail; {avoid |get_avail| if possible, to save time}
        /// avoid `get_avail` if possible, to save time
        const _ : () = ();
        $val = $globals.avail;
        // if #=null then #:=get_avail
        if $val == null {
            $val = get_avail($globals);
        }
        // else  begin avail:=link(#); link(#):=null;
        else {
            $globals.avail = link!($globals, $val);
            link!($globals, $val) = null;
            // @!stat incr(dyn_used);@+tats@/
            region_stat! {
                incr!($globals.dyn_used);
            }
            // end;
        }
        // end
        use crate::section_0115::null;
        use crate::section_0120::get_avail;
    }}
}