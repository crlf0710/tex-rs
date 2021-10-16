//! ` `
// @<Add the width of node |s| to |b...@>=
pub(crate) macro Add_the_width_of_node_s_to_break_width($globals:expr, $s:expr) {{
    /// used when calculating character widths
    let f: internal_font_number;

    // if is_char_node(s) then
    if is_char_node!($globals, $s) {
        // begin f:=font(s);
        f = font!($globals, $s);
        // break_width[1]:=@|break_width[1]+char_width(f)(char_info(f)(character(s)));
        let ch = character!($globals, $s).numeric_value();
        let ci = char_info!($globals, f, ch);
        let cw = char_width!($globals, f, ci);
        $globals.break_width[1] += cw;
        // end
    }
    // else  case type(s) of
    else {
        let type_p = r#type!($globals, $s);
        // ligature_node: begin f:=font(lig_char(s));
        if type_p == ligature_node {
            f = font!($globals, lig_char!($s));
            // break_width[1]:=break_width[1]+
            //   char_width(f)(char_info(f)(character(lig_char(s))));
            let ch = character!($globals, lig_char!($s)).numeric_value();
            let ci = char_info!($globals, f, ch);
            let cw = char_width!($globals, f, ci);
            $globals.break_width[1] += cw;
            // end;
        }
        // hlist_node,vlist_node,rule_node,kern_node:
        else if type_p == hlist_node
            || type_p == vlist_node
            || type_p == rule_node
            || type_p == kern_node
        {
            // break_width[1]:=break_width[1]+width(s);
            $globals.break_width[1] += width!($globals, $s);
        }
        // othercases confusion("disc2")
        else {
            confusion($globals, crate::strpool_str!("disc2"))?;
            // @:this can't happen disc2}{\quad disc2@>
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
