//! @ The counterpart of |eq_define| for the remaining (fullword) positions in
//! |eqtb| is called |eq_word_define|. Since |xeq_level[p]>=level_one| for all
//! |p|, a `|restore_zero|' will never be used in this case.

// @p procedure eq_word_define(@!p:pointer;@!w:integer);
#[allow(unused_variables)]
pub(crate) fn eq_word_define(globals: &mut TeXGlobals, p:pointer, w:integer) {
// begin if xeq_level[p]<>cur_level then
//   begin eq_save(p,xeq_level[p]); xeq_level[p]:=cur_level;
//   end;
// eqtb[p].int:=w;
// end;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0115::pointer;