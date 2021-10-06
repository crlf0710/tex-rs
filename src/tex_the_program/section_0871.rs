//! ` `

// @<Add the width of node |s| to |act_width|@>=
pub(crate) macro Add_the_width_of_node_s_to_act_width($globals:expr, $s:expr) {{
    // if is_char_node(s) then
    if is_char_node!($globals, $s) {
        /// used when calculating character widths
        let f: internal_font_number;

        // begin f:=font(s);
        f = font!($globals, $s);
        // act_width:=act_width+char_width(f)(char_info(f)(character(s)));
        let char = character!($globals, $s);
        let char_info = char_info!($globals, f, char.numeric_value());
        let char_width = char_width!($globals, f, char_info);
        act_width!($globals) += char_width
        // end
    }
    // else  case type(s) of
    else {
        let type_s = r#type!($globals, $s);
        // ligature_node: begin f:=font(lig_char(s));
        if type_s == ligature_node {
            /// used when calculating character widths
            let f: internal_font_number;

            f = font!($globals, lig_char!($s));
            // act_width:=act_width+
            //   char_width(f)(char_info(f)(character(lig_char(s))));
            let char = character!($globals, lig_char!($s));
            let char_info = char_info!($globals, f, char.numeric_value());
            let char_width = char_width!($globals, f, char_info);
            act_width!($globals) += char_width;
            // end;
        }
        // hlist_node,vlist_node,rule_node,kern_node:
        else if type_s == hlist_node
            || type_s == vlist_node
            || type_s == rule_node
            || type_s == kern_node
        {
            // act_width:=act_width+width(s);
            act_width!($globals) += width!($globals, $s);
        }
        // othercases confusion("disc4")
        else {
            confusion($globals, crate::strpool_str!("disc4"))?;
        }
        // @:this can't happen disc4}{\quad disc4@>
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
    use crate::section_0866::act_width;
}}
