//! @ \TeX\ is not always in vertical mode at the time |build_page|
//! is called; the current mode reflects what \TeX\ should return to, after
//! the contribution list has been emptied. A call on |build_page| should
//! be immediately followed by `|goto big_switch|', which is \TeX's central
//! control point.
//
// @d contribute=80 {go here to link a node into the current page}
//
// @p @t\4@>@<Declare the procedure called |fire_up|@>@;@/
// procedure build_page; {append contributions to the current page}
/// append contributions to the current page
#[allow(unused_variables)]
pub(crate) fn build_page(globals: &mut TeXGlobals) {
// label exit,done,done1,continue,contribute,update_heights;
// var p:pointer; {the node being appended}
// @!q,@!r:pointer; {nodes being examined}
// @!b,@!c:integer; {badness and cost of current page}
// @!pi:integer; {penalty to be added to the badness}
// @!n:min_quarterword..255; {insertion box number}
// @!delta,@!h,@!w:scaled; {sizes used for insertion calculations}
// begin if (link(contrib_head)=null)or output_active then return;
// repeat continue: p:=link(contrib_head);@/
// @<Update the values of |last_glue|, |last_penalty|, and |last_kern|@>;
// @<Move node |p| to the current page; if it is time for a page break,
//   put the nodes following the break back onto the contribution list,
//   and |return| to the user's output routine if there is one@>;
// until link(contrib_head)=null;
// @<Make the contribution list empty by setting its tail to |contrib_head|@>;
// exit:end;
}

use crate::section_0004::TeXGlobals;
