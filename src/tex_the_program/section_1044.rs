//! ` `
// @<Modify the glue specification in |main_p| according to the space factor@>=
pub(crate) macro Modify_the_glue_specification_in_main_p_according_to_the_space_factor($globals:expr) {{
    // if space_factor>=2000 then width(main_p):=width(main_p)+extra_space(cur_font);
    if space_factor!($globals) >= 2000 {
        width!($globals, $globals.main_p) = width!($globals, $globals.main_p)
            + extra_space!($globals, internal_font_number::new(cur_font!($globals)));
    }
    // stretch(main_p):=xn_over_d(stretch(main_p),space_factor,1000);
    stretch!($globals, $globals.main_p) = xn_over_d(
        $globals,
        stretch!($globals, $globals.main_p),
        space_factor!($globals).into(),
        1000,
    );
    // shrink(main_p):=xn_over_d(shrink(main_p),1000,space_factor)
    shrink!($globals, $globals.main_p) = xn_over_d(
        $globals,
        shrink!($globals, $globals.main_p),
        space_factor!($globals).into(),
        1000,
    );

    use crate::section_0107::xn_over_d;
    use crate::section_0135::width;
    use crate::section_0150::shrink;
    use crate::section_0150::stretch;
    use crate::section_0213::space_factor;
    use crate::section_0230::cur_font;
    use crate::section_0548::internal_font_number;
    use crate::section_0558::extra_space;
}}
