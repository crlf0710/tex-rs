//! @ The code that passes over the characters of words in a paragraph is
//! part of \TeX's inner loop, so it has been streamlined for speed. We use
//! the fact that `\.{\\parfillskip}' glue appears at the end of each paragraph;
//! it is therefore unnecessary to check if |link(cur_p)=null| when |cur_p| is a
//! character node.
//! @^inner loop@>
//
// @<Advance \(c)|cur_p| to the node following the present string...@>=
pub(crate) macro Advance_cur_p_to_the_node_following_the_present_string_of_characters($globals:expr, $prev_p:expr) {{
    // begin prev_p:=cur_p;
    $prev_p = $globals.cur_p;
    // repeat f:=font(cur_p);
    loop {
        /// used when calculating character widths
        let f: internal_font_number;
        f = font!($globals, $globals.cur_p);
        // act_width:=act_width+char_width(f)(char_info(f)(character(cur_p)));
        let char = character!($globals, $globals.cur_p);
        let char_info = char_info!($globals, f, char.numeric_value());
        let char_width = char_width!($globals, f, char_info);
        act_width!($globals) += char_width;
        // cur_p:=link(cur_p);
        $globals.cur_p = link!($globals, $globals.cur_p);
        // until not is_char_node(cur_p);
        if !is_char_node!($globals, $globals.cur_p) {
            break;
        }
    }
    // end
    use crate::section_0118::link;
    use crate::section_0134::character;
    use crate::section_0134::font;
    use crate::section_0134::is_char_node;
    use crate::section_0548::internal_font_number;
    use crate::section_0554::char_info;
    use crate::section_0554::char_width;
    use crate::section_0866::act_width;
}}
