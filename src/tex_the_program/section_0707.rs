//! @ The search process is complicated slightly by the facts that some of the
//! characters might not be present in some of the fonts, and they might not
//! be probed in increasing order of height.
//
// @<Look at the variants of |(z,x)|; set |f| and |c|...@>=
pub(crate) macro Look_at_the_variants_of_z_x__set_f_and_c_whenever_a_better_character_is_found__goto_found_as_soon_as_a_large_enough_variant_is_encountered($globals:expr, $z:expr, $x:expr, $s:expr, $f:expr, $c:expr, $w:expr, $v:expr, $q:expr, $lbl_found:lifetime) {{
    // if (z<>0)or(x<>min_quarterword) then
    if $z != 0 || $x.numeric_value() != min_quarterword as _ {
        // begin z:=z+s+16;
        $z = $z + $s.get() as integer + 16;
        // repeat z:=z-16; g:=fam_fnt(z);
        loop {
            /// best-so-far and tentative font codes
            let g: internal_font_number;
            $z = $z - 16;
            g = fam_fnt!($globals, $z).into();
            // if g<>null_font then
            if g != null_font {
                // @<Look at the list of characters starting with |x| in
                //   font |g|; set |f| and |c| whenever
                //   a better character is found; |goto found| as soon as a
                //   large enough variant is encountered@>;
                crate::section_0708::Look_at_the_list_of_characters_starting_with_x_in_font_g__set_f_and_c_whenever_a_better_character_is_found__goto_found_as_soon_as_a_large_enough_variant_is_encountered!($globals, $x, g, $f, $c, $w, $v, $q, $lbl_found);
            }
            // until z<16;
            if $z < 16 {
                break;
            }
        }
        // end
    }
    use crate::pascal::integer;
    use crate::section_0110::min_quarterword;
    use crate::section_0230::fam_fnt;
    use crate::section_0232::null_font;
    use crate::section_0548::internal_font_number;
}}
