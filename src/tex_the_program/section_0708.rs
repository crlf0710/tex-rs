//! ` `

// @<Look at the list of characters starting with |x|...@>=
pub(crate) macro Look_at_the_list_of_characters_starting_with_x_in_font_g__set_f_and_c_whenever_a_better_character_is_found__goto_found_as_soon_as_a_large_enough_variant_is_encountered($globals:expr, $x:expr, $g:expr, $f:expr, $c:expr, $w:expr, $v:expr, $q:expr, $lbl_found:lifetime) {{
    /// best-so-far and tentative character codes
    let y;
    // begin y:=x;
    y = $x;
    // if (qo(y)>=font_bc[g])and(qo(y)<=font_ec[g]) then
    if y >= $globals.font_bc[$g] && y <= $globals.font_ec[$g] {
        // begin continue: q:=char_info(g)(y);
        crate::region_backward_label! {
            'continue_ <-
            {
                $q = char_info!($globals, $g, y.numeric_value());
                // if char_exists(q) then
                if $q.char_exists() {
                    /// height-plus-depth of a tentative character
                    let u;
                    /// height-depth byte
                    let hd;
                    // begin if char_tag(q)=ext_tag then
                    if $q.char_tag() == char_tag::ext_tag {
                        // begin f:=g; c:=y; goto found;
                        // end;
                        todo!("ext_tag");
                    }
                    // hd:=height_depth(q);
                    hd = $q.height_depth();
                    // u:=char_height(g)(hd)+char_depth(g)(hd);
                    u = char_height!($globals, $g, hd) + char_depth!($globals, $g, hd);
                    // if u>w then
                    if u > $w {
                        // begin f:=g; c:=y; w:=u;
                        $f = $g;
                        $c = y;
                        $w = u;
                        // if u>=v then goto found;
                        if u > $v {
                            crate::goto_forward_label!($lbl_found);
                        }
                        // end;
                    }
                    // if char_tag(q)=list_tag then
                    if $q.char_tag() == char_tag::list_tag {
                        // begin y:=rem_byte(q); goto continue;
                        // end;
                        todo!("list_tag");
                    }
                    // end;
                }
            }
            |'continue_|
        };
        // end;
    }
    // end
    use crate::section_0544::char_tag;
    use crate::section_0554::char_depth;
    use crate::section_0554::char_height;
    use crate::section_0554::char_info;
}}
