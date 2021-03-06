//! ` `
// @<Modify the glue specification in |main_p| according to the space factor@>=
macro_rules! Modify_the_glue_specification_in_main_p_according_to_the_space_factor {
    ($globals:expr) => {{
        // if space_factor>=2000 then width(main_p):=width(main_p)+extra_space(cur_font);
        if space_factor!($globals) >= 2000 {
            width!($globals, $globals.main_p) =
                width!($globals, $globals.main_p) + extra_space!($globals, cur_font!($globals));
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
    }};
}
