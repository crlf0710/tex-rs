//! @ To put a byte in the buffer without paying the cost of invoking a procedure
//! each time, we use the macro |dvi_out|.
//
// @d dvi_out(#)==@+begin dvi_buf[dvi_ptr]:=#; incr(dvi_ptr);
pub(crate) macro dvi_out($globals:expr, $v:expr) {{
    $globals.dvi_buf[$globals.dvi_ptr] = $v as _;
    incr!($globals.dvi_ptr);
    // if dvi_ptr=dvi_limit then dvi_swap;
    if $globals.dvi_ptr == $globals.dvi_limit {
        dvi_swap($globals);
    }
    // end
    use crate::section_0016::incr;
    use crate::section_0598::dvi_swap;
}}

// @p procedure dvi_swap; {outputs half of the buffer}
/// outputs half of the buffer
pub(crate) fn dvi_swap(globals: &mut TeXGlobals) {
    // begin if dvi_limit=dvi_buf_size then
    if globals.dvi_limit == dvi_buf_size {
        // begin write_dvi(0,half_buf-1); dvi_limit:=half_buf;
        write_dvi(globals, 0.into(), globals.half_buf - 1);
        globals.dvi_limit = globals.half_buf;
        // dvi_offset:=dvi_offset+dvi_buf_size; dvi_ptr:=0;
        globals.dvi_offset += dvi_buf_size as integer;
        globals.dvi_ptr = 0.into();
        // end
    }
    // else  begin write_dvi(half_buf,dvi_buf_size-1); dvi_limit:=dvi_buf_size;
    else {
        write_dvi(globals, globals.half_buf, dvi_index::new(dvi_buf_size - 1));
        globals.dvi_limit = dvi_buf_size.into();
        // end;
    }
    // dvi_gone:=dvi_gone+half_buf;
    globals.dvi_gone += globals.half_buf.get() as integer;
    // end;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0011::dvi_buf_size;
use crate::section_0594::dvi_index;
use crate::section_0597::write_dvi;
