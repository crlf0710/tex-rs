//! @ That which can be displayed can also be destroyed.
//
// @<Cases of |flush_node_list| that arise...@>=
pub(crate) macro Cases_of_flush_node_list_that_arise_in_mlists_only($globals:expr, $p:expr, $type_p:expr, $lbl_done:lifetime) {{
    // style_node: begin free_node(p,style_node_size); goto done;
    let processed = if $type_p == style_node {
        todo!("style_node");
        // end;
        true
    }
    // choice_node:begin flush_node_list(display_mlist(p));
    else if $type_p == choice_node {
        // flush_node_list(text_mlist(p));
        // flush_node_list(script_mlist(p));
        // flush_node_list(script_script_mlist(p));
        // free_node(p,style_node_size); goto done;
        // end;
        todo!("choice_node");
        true
    }
    // ord_noad,op_noad,bin_noad,rel_noad,open_noad,close_noad,punct_noad,inner_noad,
    //   radical_noad,over_noad,under_noad,vcenter_noad,accent_noad:@t@>@;@/
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
    {
        // begin if math_type(nucleus(p))>=sub_box then
        if math_type!($globals, nucleus!($p)) >= math_type_kind::sub_box as _ {
            // flush_node_list(info(nucleus(p)));
            flush_node_list($globals, info_inner!($globals, nucleus!($p)))?;
        }
        // if math_type(supscr(p))>=sub_box then
        if math_type!($globals, supscr!($p)) >= math_type_kind::sub_box as _ {
            // flush_node_list(info(supscr(p)));
            flush_node_list($globals, info_inner!($globals, supscr!($p)))?;
        }
        // if math_type(subscr(p))>=sub_box then
        if math_type!($globals, subscr!($p)) >= math_type_kind::sub_box as _ {
            // flush_node_list(info(subscr(p)));
            flush_node_list($globals, info_inner!($globals, subscr!($p)))?;
        }
        // if type(p)=radical_noad then free_node(p,radical_noad_size)
        if r#type!($globals, $p) == radical_noad {
            free_node($globals, $p, radical_noad_size as _);
        }
        // else if type(p)=accent_noad then free_node(p,accent_noad_size)
        else if r#type!($globals, $p) == accent_noad {
            free_node($globals, $p, accent_noad_size as _);
        }
        // else free_node(p,noad_size);
        else {
            free_node($globals, $p, noad_size as _);
        }
        // goto done;
        crate::goto_forward_label!($lbl_done);
        // end;
        true
    }
    // left_noad,right_noad: begin free_node(p,noad_size); goto done;
    else if $type_p == left_noad || $type_p == right_noad {
        todo!("l|r noad");
        //   end;
        true
    }
    // fraction_noad: begin flush_node_list(info(numerator(p)));
    else if $type_p == fraction_noad {
        // flush_node_list(info(denominator(p)));
        // free_node(p,fraction_noad_size); goto done;
        // end;
        todo!("fraction_noad");
        true
    } else {
        false
    };
    use crate::section_0118::info_inner;
    use crate::section_0130::free_node;
    use crate::section_0133::r#type;
    use crate::section_0202::flush_node_list;
    use crate::section_0681::math_type;
    use crate::section_0681::math_type_kind;
    use crate::section_0681::noad_size;
    use crate::section_0681::nucleus;
    use crate::section_0681::subscr;
    use crate::section_0681::supscr;
    use crate::section_0682::bin_noad;
    use crate::section_0682::close_noad;
    use crate::section_0682::inner_noad;
    use crate::section_0682::op_noad;
    use crate::section_0682::open_noad;
    use crate::section_0682::ord_noad;
    use crate::section_0682::punct_noad;
    use crate::section_0682::rel_noad;
    use crate::section_0683::fraction_noad;
    use crate::section_0683::radical_noad;
    use crate::section_0683::radical_noad_size;
    use crate::section_0687::accent_noad;
    use crate::section_0687::accent_noad_size;
    use crate::section_0687::left_noad;
    use crate::section_0687::over_noad;
    use crate::section_0687::right_noad;
    use crate::section_0687::under_noad;
    use crate::section_0687::vcenter_noad;
    use crate::section_0688::style_node;
    use crate::section_0689::choice_node;
    processed
}}
