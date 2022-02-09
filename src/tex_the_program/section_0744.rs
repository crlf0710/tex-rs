//! ` `

// @<Create equal-width boxes |x| and |z| for the numerator and denom...@>=
pub(crate) macro Create_equal_width_boxes_x_and_z_for_the_numerator_and_denominator__and_compute_the_default_amounts_shift_up_and_shift_down_by_which_they_are_displaced_from_the_baseline($globals:expr, $q:expr, $x:expr, $z:expr, $shift_up:expr, $shift_down:expr) {{
    // x:=clean_box(numerator(q),num_style(cur_style));
    $x = clean_box(
        $globals,
        numerator!($q),
        num_style!($globals.cur_style).into(),
    )?;
    // z:=clean_box(denominator(q),denom_style(cur_style));
    $z = clean_box(
        $globals,
        denominator!($q),
        denom_style!($globals.cur_style).into(),
    )?;
    // if width(x)<width(z) then x:=rebox(x,width(z))
    if width!($globals, $x) < width!($globals, $z) {
        $x = rebox($globals, $x, width!($globals, $z))?;
    }
    // else z:=rebox(z,width(x));
    else {
        $z = rebox($globals, $z, width!($globals, $x))?;
    }
    // if cur_style<text_style then {display style}
    if $globals.cur_style.get() < style_node_subtype::text_style.get() {
        /// display style
        const _: () = ();
        // begin shift_up:=num1(cur_size); shift_down:=denom1(cur_size);
        $shift_up = num1!($globals, $globals.cur_size.get());
        $shift_down = denom1!($globals, $globals.cur_size.get());
        // end
    }
    // else  begin shift_down:=denom2(cur_size);
    else {
        $shift_down = denom2!($globals, $globals.cur_size.get());
        // if thickness(q)<>0 then shift_up:=num2(cur_size)
        if thickness!($globals, $q) != scaled::zero() {
            $shift_up = num2!($globals, $globals.cur_size.get());
        }
        // else shift_up:=num3(cur_size);
        else {
            $shift_up = num3!($globals, $globals.cur_size.get());
            // end
        }
    }

    use crate::section_0101::scaled;
    use crate::section_0135::width;
    use crate::section_0683::denominator;
    use crate::section_0683::numerator;
    use crate::section_0683::thickness;
    use crate::section_0688::style_node_subtype;
    use crate::section_0700::denom1;
    use crate::section_0700::denom2;
    use crate::section_0700::num1;
    use crate::section_0700::num2;
    use crate::section_0700::num3;
    use crate::section_0702::denom_style;
    use crate::section_0702::num_style;
    use crate::section_0715::rebox;
    use crate::section_0720::clean_box;
}}
