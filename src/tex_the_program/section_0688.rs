//! ` `
//!
//! Math formulas can also contain instructions like \.{\\textstyle} that
//! override \TeX's normal style rules. A |style_node| is inserted into the
//! data structure to record such instructions; it is three words long, so it
//! is considered a node instead of a noad. The |subtype| is either |display_style|
//! or |text_style| or |script_style| or |script_script_style|. The
//! second and third words of a |style_node| are not used, but they are
//! present because a |choice_node| is converted to a |style_node|.
//!
//! \TeX\ uses even numbers 0, 2, 4, 6 to encode the basic styles
//! |display_style|, \dots, |script_script_style|, and adds~1 to get the
//! ``cramped'' versions of these styles. This gives a numerical order that
//! is backwards from the convention of Appendix~G in {\sl The \TeX book\/};
//! i.e., a smaller style has a larger numerical value.
//! @:TeXbook}{\sl The \TeX book@>
//
// @d style_node=unset_node+1 {|type| of a style node}
/// `type` of a style node
pub(crate) const style_node: quarterword = unset_node + 1;

// @d style_node_size=3 {number of words in a style node}
// @d display_style=0 {|subtype| for \.{\\displaystyle}}
// @d text_style=2 {|subtype| for \.{\\textstyle}}
// @d script_style=4 {|subtype| for \.{\\scriptstyle}}
// @d script_script_style=6 {|subtype| for \.{\\scriptscriptstyle}}
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub(crate) enum style_node_subtype {
    /// `subtype` for `\displaystyle`
    display_style = 0,
    /// `subtype` for `\textstyle`
    text_style = 2,
    /// `subtype` for `\scriptstyle`
    script_style = 4,
    /// `subtype` for `\scriptscriptstyle`
    script_script_style = 6,
    ///
    display_style_cramped = 0 + 1,
    ///
    text_style_cramped = 2 + 1,
    ///
    script_style_cramped = 4 + 1,
    ///
    script_script_style_cramped = 6 + 1,
}
// @d cramped=1 {add this to an uncramped style if you want to cramp it}

impl style_node_subtype {
    pub(crate) fn get(self) -> quarterword {
        self as _
    }
}

// @p function new_style(@!s:small_number):pointer; {create a style node}
// var p:pointer; {the new node}
// begin p:=get_node(style_node_size); type(p):=style_node;
// subtype(p):=s; width(p):=0; depth(p):=0; {the |width| and |depth| are not used}
// new_style:=p;
// end;
//

use crate::section_0113::quarterword;
use crate::section_0159::unset_node;
