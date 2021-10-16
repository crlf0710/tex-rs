//! ` `
//! Replacement texts and discretionary texts are supposed to contain
//! only character nodes, kern nodes, ligature nodes, and box or rule nodes.
//
// @<Subtract the width of node |v|...@>=
pub(crate) macro Subtract_the_width_of_node_v_from_break_width($globals:expr, $v:expr) {{
    /// used when calculating character widths
    let f: internal_font_number;

    // if is_char_node(v) then
    if is_char_node!($globals, $v) {
        // begin f:=font(v);
        f = font!($globals, $v);
        // break_width[1]:=break_width[1]-char_width(f)(char_info(f)(character(v)));
        let ch = character!($globals, $v).numeric_value();
        let ci = char_info!($globals, f, ch);
        let cw = char_width!($globals, f, ci);
        $globals.break_width[1] -= cw;
        // end
    }
    // else  case type(v) of
    else {
        let type_p = r#type!($globals, $v);
        // ligature_node: begin f:=font(lig_char(v));@/
        if type_p == ligature_node {
            f = font!($globals, lig_char!($v));
            // break_width[1]:=@|break_width[1]-
            //   char_width(f)(char_info(f)(character(lig_char(v))));
            let ch = character!($globals, lig_char!($v)).numeric_value();
            let ci = char_info!($globals, f, ch);
            let cw = char_width!($globals, f, ci);
            $globals.break_width[1] -= cw;
            // end;
        }
        // hlist_node,vlist_node,rule_node,kern_node:
        else if type_p == hlist_node
            || type_p == vlist_node
            || type_p == rule_node
            || type_p == kern_node
        {
            // break_width[1]:=break_width[1]-width(v);
            $globals.break_width[1] -= width!($globals, $v);
        }
        // othercases confusion("disc1")
        else {
            confusion($globals, crate::strpool_str!("disc1"))?;
            // @:this can't happen disc1}{\quad disc1@>
        }
        // endcases
    }
    use crate::section_0095::confusion;
    use crate::section_0133::r#type;
    use crate::section_0134::character;
    use crate::section_0134::font;
    use crate::section_0134::is_char_node;
    use crate::section_0135::hlist_node;
    use crate::section_0135::width;
    use crate::section_0137::vlist_node;
    use crate::section_0138::rule_node;
    use crate::section_0143::lig_char;
    use crate::section_0143::ligature_node;
    use crate::section_0155::kern_node;
    use crate::section_0548::internal_font_number;
    use crate::section_0554::char_info;
    use crate::section_0554::char_width;
}}
