//! ` `

// @<Add the width of node |s| to |disc_width|@>=
pub(crate) macro Add_the_width_of_node_s_to_disc_width($globals:expr, $s:expr) {{
    /// used when calculating character widths
    let f: internal_font_number;

    // if is_char_node(s) then
    if is_char_node!($globals, $s) {
        // begin f:=font(s);
        f = font!($globals, $s);
        // disc_width:=disc_width+char_width(f)(char_info(f)(character(s)));
        let ch = character!($globals, $s).numeric_value();
        let ci = char_info!($globals, f, ch);
        let cw = char_width!($globals, f, ci);
        $globals.disc_width += cw;
        // end
    }
    // else  case type(s) of
    else {
        let type_s = r#type!($globals, $s);
        // ligature_node: begin f:=font(lig_char(s));
        if type_s == ligature_node {
            f = font!($globals, lig_char!($s));
            // disc_width:=disc_width+
            //   char_width(f)(char_info(f)(character(lig_char(s))));
            let ch = character!($globals, lig_char!($s)).numeric_value();
            let ci = char_info!($globals, f, ch);
            let cw = char_width!($globals, f, ci);
            $globals.disc_width += cw;
            // end;
        }
        // hlist_node,vlist_node,rule_node,kern_node:
        else if type_s == hlist_node
            || type_s == vlist_node
            || type_s == rule_node
            || type_s == kern_node
        {
            // disc_width:=disc_width+width(s);
            $globals.disc_width += width!($globals, $s);
        }
        // othercases confusion("disc3")
        else {
            confusion($globals, crate::strpool_str!("disc3"))?;
            // @:this can't happen disc3}{\quad disc3@>
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
