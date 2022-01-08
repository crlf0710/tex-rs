//! @ Let's consider now the previously unwritten part of |show_node_list|
//! that displays the things that can only be present in mlists; this
//! program illustrates how to access the data structures just defined.
//!
//! In the context of the following program, |p| points to a node or noad that
//! should be displayed, and the current string contains the ``recursion history''
//! that leads to this point. The recursion history consists of a dot for each
//! outer level in which |p| is subsidiary to some node, or in which |p| is
//! subsidiary to the |nucleus| field of some noad; the dot is replaced by
//! `\.\_' or `\.\^' or `\./' or `\.\\' if |p| is descended from the |subscr|
//! or |supscr| or |denominator| or |numerator| fields of noads. For example,
//! the current string would be `\.{.\^.\_/}' if |p| points to the |ord_noad| for
//! |x| in the (ridiculous) formula
//! `\.{\$\\sqrt\{a\^\{\\mathinner\{b\_\{c\\over x+y\}\}\}\}\$}'.
//
// @<Cases of |show_node_list| that arise...@>=
pub(crate) macro Cases_of_show_node_list_that_arise_in_mlists_only($globals:expr, $p:expr, $type_p:expr) {{
    // style_node:print_style(subtype(p));
    let processed = if $type_p == style_node {
        todo!("style_node");
        true
    }
    // choice_node:@<Display choice node |p|@>;
    else if $type_p == choice_node {
        todo!("choice_node");
        true
    }
    // ord_noad,op_noad,bin_noad,rel_noad,open_noad,close_noad,punct_noad,inner_noad,
    //   radical_noad,over_noad,under_noad,vcenter_noad,accent_noad,
    //   left_noad,right_noad:@<Display normal noad |p|@>;
    else if $type_p == ord_noad
        || $type_p == op_noad
        || $type_p == bin_noad
        || $type_p == rel_noad
        || $type_p == open_noad
        || $type_p == close_noad
        || $type_p == punct_noad
        || $type_p == inner_noad
        || $type_p == radical_noad
        || $type_p == over_noad
        || $type_p == under_noad
        || $type_p == vcenter_noad
        || $type_p == accent_noad
        || $type_p == left_noad
        || $type_p == right_noad
    {
        todo!("normal noad");
        true
    }
    // fraction_noad:@<Display fraction noad |p|@>;
    else if $type_p == fraction_noad {
        todo!("fraction noad");
        true
    } else {
        false
    };

    use crate::section_0682::*;
    use crate::section_0683::*;
    use crate::section_0687::*;
    use crate::section_0688::*;
    use crate::section_0689::*;
    processed
}}
