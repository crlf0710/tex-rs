//! @ However, all this discussion about input state really applies only to the
//! case that we are inputting from a file. There is another important case,
//! namely when we are currently getting input from a token list. In this case
//! |state=token_list|, and the conventions about the other state variables
//! are different:
//!
//! \yskip\hang|loc| is a pointer to the current node in the token list, i.e.,
//! the node that will be read next. If |loc=null|, the token list has been
//! fully read.
//!
//! \yskip\hang|start| points to the first node of the token list; this node
//! may or may not contain a reference count, depending on the type of token
//! list involved.
//!
//! \yskip\hang|token_type|, which takes the place of |index| in the
//! discussion above, is a code number that explains what kind of token list
//! is being scanned.
//!
//! \yskip\hang|name| points to the |eqtb| address of the control sequence
//! being expanded, if the current token list is a macro.
//!
//! \yskip\hang|param_start|, which takes the place of |limit|, tells where
//! the parameters of the current macro begin in the |param_stack|, if the
//! current token list is a macro.
//!
//! \yskip\noindent The |token_type| can take several values, depending on
//! where the current token list came from:
//!
//! \yskip\hang|parameter|, if a parameter is being scanned;
//!
//! \hang|u_template|, if the \<u_j> part of an alignment
//! template is being scanned;
//!
//! \hang|v_template|, if the \<v_j> part of an alignment
//! template is being scanned;
//!
//! \hang|backed_up|, if the token list being scanned has been inserted as
//! `to be read again'.
//!
//! \hang|inserted|, if the token list being scanned has been inserted as
//! the text expansion of a \.{\\count} or similar variable;
//!
//! \hang|macro|, if a user-defined control sequence is being scanned;
//!
//! \hang|output_text|, if an \.{\\output} routine is being scanned;
//!
//! \hang|every_par_text|, if the text of \.{\\everypar} is being scanned;
//!
//! \hang|every_math_text|, if the text of \.{\\everymath} is being scanned;
//!
//! \hang|every_display_text|, if the text of \.{\\everydisplay} is being scanned;
//!
//! \hang|every_hbox_text|, if the text of \.{\\everyhbox} is being scanned;
//!
//! \hang|every_vbox_text|, if the text of \.{\\everyvbox} is being scanned;
//!
//! \hang|every_job_text|, if the text of \.{\\everyjob} is being scanned;
//!
//! \hang|every_cr_text|, if the text of \.{\\everycr} is being scanned;
//!
//! \hang|mark_text|, if the text of a \.{\\mark} is being scanned;
//!
//! \hang|write_text|, if the text of a \.{\\write} is being scanned.
//!
//! \yskip\noindent
//! The codes for |output_text|, |every_par_text|, etc., are equal to a constant
//! plus the corresponding codes for token list parameters |output_routine_loc|,
//! |every_par_loc|, etc.  The token list begins with a reference count if and
//! only if |token_type>=macro|.
//! @^reference counts@>

// @d token_list=0 {|state| code when scanning a token list}
/// `state` code when scanning a token list
pub(crate) const token_list: quarterword = 0;
// @d token_type==index {type of current token list}
/// type of current token list
macro_rules! token_type {
    ($globals:expr) => {index!($globals)}
}
// @d param_start==limit {base of macro parameters in |param_stack|}
/// base of macro parameters in `param_stack`
#[allow(unused_macros)]
macro_rules! param_start {
    ($globals:expr) => {limit!($globals)}
}
// @d parameter=0 {|token_type| code for parameter}
/// `token_type` code for parameter
pub(crate) const parameter: quarterword = 0;
// @d u_template=1 {|token_type| code for \<u_j> template}
/// `token_type` code for `<u_j>` template
pub(crate) const u_template: quarterword = 1;
// @d v_template=2 {|token_type| code for \<v_j> template}
/// `token_type` code for `<v_j>` template
pub(crate) const v_template: quarterword = 2;
// @d backed_up=3 {|token_type| code for text to be reread}
/// `token_type` code for text to be reread
pub(crate) const backed_up: quarterword = 3;
// @d inserted=4 {|token_type| code for inserted texts}
/// `token_type` code for inserted texts
pub(crate) const inserted: quarterword = 4;
// @d macro=5 {|token_type| code for defined control sequences}
/// `token_type` code for defined control sequences
pub(crate) const r#macro: quarterword = 5;
// @d output_text=6 {|token_type| code for output routines}
/// `token_type` code for output routines
pub(crate) const output_text: quarterword = 6;
// @d every_par_text=7 {|token_type| code for \.{\\everypar}}
// @d every_math_text=8 {|token_type| code for \.{\\everymath}}
// @d every_display_text=9 {|token_type| code for \.{\\everydisplay}}
// @d every_hbox_text=10 {|token_type| code for \.{\\everyhbox}}
/// `token_type` code for `\everyhbox`
pub(crate) const every_hbox_text: quarterword = 10;
// @d every_vbox_text=11 {|token_type| code for \.{\\everyvbox}}
// @d every_job_text=12 {|token_type| code for \.{\\everyjob}}
/// `token_type` code for `\everyjob`
pub(crate) const every_job_text: quarterword = 12;
// @d every_cr_text=13 {|token_type| code for \.{\\everycr}}
// @d mark_text=14 {|token_type| code for \.{\\topmark}, etc.}
/// `token_type` code for `\topmark`, etc.
pub(crate) const mark_text: quarterword = 14;
// @d write_text=15 {|token_type| code for \.{\\write}}
/// `token_type` code for `\write`
pub(crate) const write_text: quarterword = 15;

use crate::section_0113::quarterword;
