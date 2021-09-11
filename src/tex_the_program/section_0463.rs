//! @ Here's a similar procedure that returns a pointer to a rule node. This
//! routine is called just after \TeX\ has seen \.{\\hrule} or \.{\\vrule};
//! therefore |cur_cmd| will be either |hrule| or |vrule|. The idea is to store
//! the default rule dimensions in the node, then to override them if
//! `\.{height}' or `\.{width}' or `\.{depth}' specifications are
//! found (in any order).
//
// @d default_rule=26214 {0.4\thinspace pt}
pub(crate) const default_rule: scaled = scaled::new_from_inner(26214);
//
// @p function scan_rule_spec:pointer;
pub(crate) fn scan_rule_spec(globals: &mut TeXGlobals) -> TeXResult<pointer> {
    // label reswitch;
    // var q:pointer; {the rule node being created}
    /// the rule node being created
    let q: pointer;
    // begin q:=new_rule; {|width|, |depth|, and |height| all equal |null_flag| now}
    /// `width`, `depth`, and `height` all equal `null_flag` now
    const _: () = ();
    q = new_rule(globals)?;
    // if cur_cmd=vrule then width(q):=default_rule
    if globals.cur_cmd == vrule {
        width!(globals, q) = default_rule;
    }
    // else  begin height(q):=default_rule; depth(q):=0;
    else {
        height!(globals, q) = default_rule;
        depth!(globals, q) = scaled::zero();
        // end;
    }
    // reswitch: if scan_keyword("width") then
    crate::region_backward_label!(
        'reswitch <-
        {
            if scan_keyword(globals, crate::strpool_str!("width"))? {
                // @.width@>
                // begin scan_normal_dimen; width(q):=cur_val; goto reswitch;
                scan_normal_dimen!(globals)?;
                width!(globals, q) = scaled::new_from_inner(globals.cur_val);
                crate::goto_backward_label!('reswitch);
                // end;
            }
            // if scan_keyword("height") then
            if scan_keyword(globals, crate::strpool_str!("height"))? {
                // @.height@>
                // begin scan_normal_dimen; height(q):=cur_val; goto reswitch;
                scan_normal_dimen!(globals)?;
                height!(globals, q) = scaled::new_from_inner(globals.cur_val);
                crate::goto_backward_label!('reswitch);
                // end;
            }
            // if scan_keyword("depth") then
            if scan_keyword(globals, crate::strpool_str!("depth"))? {
                // @.depth@>
                // begin scan_normal_dimen; depth(q):=cur_val; goto reswitch;
                scan_normal_dimen!(globals)?;
                depth!(globals, q) = scaled::new_from_inner(globals.cur_val);
                crate::goto_backward_label!('reswitch);
                // end;
            }
        }
        |'reswitch|
    );
    // scan_rule_spec:=q;
    crate::return_nojump!(q);
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::scaled;
use crate::section_0115::pointer;
use crate::section_0135::depth;
use crate::section_0135::height;
use crate::section_0135::width;
use crate::section_0139::new_rule;
use crate::section_0208::vrule;
use crate::section_0407::scan_keyword;
use crate::section_0448::scan_normal_dimen;
