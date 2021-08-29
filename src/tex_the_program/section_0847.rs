//! @ The length of lines depends on whether the user has specified
//! \.{\\parshape} or \.{\\hangindent}. If |par_shape_ptr| is not null, it
//! points to a $(2n+1)$-word record in |mem|, where the |info| in the first
//! word contains the value of |n|, and the other $2n$ words contain the left
//! margins and line lengths for the first |n| lines of the paragraph; the
//! specifications for line |n| apply to all subsequent lines. If
//! |par_shape_ptr=null|, the shape of the paragraph depends on the value of
//! |n=hang_after|; if |n>=0|, hanging indentation takes place on lines |n+1|,
//! |n+2|, \dots, otherwise it takes place on lines 1, \dots, $\vert
//! n\vert$. When hanging indentation is active, the left margin is
//! |hang_indent|, if |hang_indent>=0|, else it is 0; the line length is
//! $|hsize|-\vert|hang_indent|\vert$. The normal setting is
//! |par_shape_ptr=null|, |hang_after=1|, and |hang_indent=0|.
//! Note that if |hang_indent=0|, the value of |hang_after| is irrelevant.
//! @^length of lines@> @^hanging indentation@>
//
// @<Glob...@>=
// @!easy_line:halfword; {line numbers |>easy_line| are equivalent in break nodes}
/// line numbers `>easy_line` are equivalent in break nodes
#[globals_struct_field(TeXGlobals)]
pub(crate) static easy_line: halfword = 0;
// @!last_special_line:halfword; {line numbers |>last_special_line| all have
//   the same width}
/// line numbers `>last_special_line` all have the same width
#[globals_struct_field(TeXGlobals)]
pub(crate) static last_special_line: halfword = 0;
// @!first_width:scaled; {the width of all lines |<=last_special_line|, if
//   no \.{\\parshape} has been specified}
/// the width of all lines `<=last_special_line`, if no `\parshape` has been specified
#[globals_struct_field(TeXGlobals)]
pub(crate) static first_width: scaled = scaled::zero();
// @!second_width:scaled; {the width of all lines |>last_special_line|}
/// the width of all lines `>last_special_line`
#[globals_struct_field(TeXGlobals)]
pub(crate) static second_width: scaled = scaled::zero();
// @!first_indent:scaled; {left margin to go with |first_width|}
/// left margin to go with `first_width`
#[globals_struct_field(TeXGlobals)]
pub(crate) static first_indent: scaled = scaled::zero();
// @!second_indent:scaled; {left margin to go with |second_width|}
/// left margin to go with `second_width`
#[globals_struct_field(TeXGlobals)]
pub(crate) static second_indent: scaled = scaled::zero();

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
