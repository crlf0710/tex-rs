//! @ The following code is part of \TeX's inner loop; i.e., adding another
//! character of text to the user's input will cause each of these instructions
//! to be exercised one more time.
//! @^inner loop@>
//
// @<Incorporate character dimensions into the dimensions of the hbox...@>=

macro_rules! Incorporate_character_dimensions_into_the_dimensions_of_the_hbox_that_will_contain_it__then_move_to_the_next_node {
    ($globals:expr, $p:expr, $h:expr, $d:expr, $x:expr) => {{
        /// shift amount
        let mut s: scaled;
        /// the font in a `char_node`
        let f: internal_font_number;
        /// font information about a `char_node`
        let i: char_info;
        /// height and depth indices for a character
        let hd: eight_bits;
        // begin f:=font(p); i:=char_info(f)(character(p)); hd:=height_depth(i);
        f = font!($globals, $p);
        let character = character!($globals, $p);
        i = char_info!($globals, f, character.numeric_value());
        hd = i.height_depth();
        // x:=x+char_width(f)(i);@/
        $x = $x + char_width!($globals, f, i);
        // s:=char_height(f)(hd);@+if s>h then h:=s;
        s = char_height!($globals, f, hd);
        if s > $h {
            $h = s;
        }
        // s:=char_depth(f)(hd);@+if s>d then d:=s;
        s = char_depth!($globals, f, hd);
        if s > $d {
            $d = s;
        }
        // p:=link(p);
        $p = link!($globals, $p);
        // end

        use crate::section_0025::eight_bits;
        use crate::section_0113::four_quarters;
        use crate::section_0548::internal_font_number;
        use crate::section_0554::char_info;
    }}
}