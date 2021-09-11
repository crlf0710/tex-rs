//! @ The actual output of |dvi_buf[a..b]| to |dvi_file| is performed by calling
//! |write_dvi(a,b)|. For best results, this procedure should be optimized to
//! run as fast as possible on each particular system, since it is part of
//! \TeX's inner loop. It is safe to assume that |a| and |b+1| will both be
//! multiples of 4 when |write_dvi(a,b)| is called; therefore it is possible on
//! many machines to use efficient methods to pack four bytes per word and to
//! output an array of words with one system call.
//! @^system dependencies@>
//! @^inner loop@>
//! @^defecation@>
//
// @p procedure write_dvi(@!a,@!b:dvi_index);
pub(crate) fn write_dvi(globals: &mut TeXGlobals, a: dvi_index, b: dvi_index) {
    // var k:dvi_index;
    // begin for k:=a to b do write(dvi_file,dvi_buf[k]);
    for k in a.get()..=b.get() {
        write_binary(&mut globals.dvi_file, globals.dvi_buf[k]);
    }
    // end;
}

use crate::pascal::write_binary;
use crate::section_0004::TeXGlobals;
use crate::section_0594::dvi_index;
