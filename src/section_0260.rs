// @ @<Insert a new control...@>=
macro_rules! Insert_a_new_control_sequence_after_p_then_make_p_point_to_it {
    ($globals:expr, $p:expr, $j:expr, $l_raw:expr) => {
        // begin if text(p)>0 then
        trace_expr!("p = {}", $p);
        if text!($globals, $p) > 0 {
            todo!();
            //   begin repeat if hash_is_full then overflow("hash size",hash_size);
            // @:TeX capacity exceeded hash size}{\quad hash size@>
            //   decr(hash_used);
            //   until text(hash_used)=0; {search for an empty location in |hash|}
            //   next(p):=hash_used; p:=hash_used;
            //   end;
        }
        #[cfg(not(feature = "unicode_support"))]
        let l = $l_raw;
        #[cfg(feature = "unicode_support")]
        let l = buffer_range_bytes($globals, $j, $l_raw);
        str_room($globals, l);
        let d = cur_length!($globals);
        // str_room(l); d:=cur_length;
        // while pool_ptr>str_start[str_ptr] do
        /// move current string up to make room for another
        while $globals.pool_ptr > $globals.str_start[$globals.str_ptr] {
            // begin decr(pool_ptr); str_pool[pool_ptr+l]:=str_pool[pool_ptr];
            decr!($globals.pool_ptr);
            $globals.str_pool[$globals.pool_ptr + l] = $globals.str_pool[$globals.pool_ptr];
            // end; {move current string up to make room for another}
        }
        // for k:=j to j+l-1 do append_char(buffer[k]);
        for k in $j..=$j + l - 1 {
            append_char($globals, $globals.buffer[k as u16]);
        }
        // text(p):=make_string; pool_ptr:=pool_ptr+d;
        text!($globals, $p) = make_string($globals).get() as _;
        trace_expr!("text(p) = {}", text!($globals, $p));
        $globals.pool_ptr = $globals.pool_ptr + d;
        // @!stat incr(cs_count);@+tats@;@/
        // end
        use crate::section_0042::str_room;
        use crate::section_0042::append_char;
        use crate::section_0043::make_string;
        #[cfg(feature = "unicode_support")]
        use crate::section_0260::buffer_range_bytes;
    }
}

#[cfg(feature = "unicode_support")]
pub(crate) fn buffer_range_bytes(globals: &TeXGlobals, j: integer, l: integer) -> integer {
    let mut result = 0;
    for k in 0..l {
        result += globals.buffer[u16_from_0_to_n::new((j + k) as _)].fss_utf_len() as integer;
    }
    result
}

use crate::pascal::integer;
use crate::pascal::u16_from_0_to_n;
use crate::section_0004::TeXGlobals;
