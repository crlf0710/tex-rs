//! @ The bottom of a superscript should never descend below the baseline plus
//! one-fourth of the x-height.
//
// @<Construct a superscript box |x|@>=
pub(crate) macro Construct_a_superscript_box_x($globals:expr, $x:expr, $q:expr, $shift_up:expr, $clr:expr) {{
    // begin x:=clean_box(supscr(q),sup_style(cur_style));
    $x = clean_box($globals, supscr!($q), sup_style!($globals.cur_style).into())?;
    // width(x):=width(x)+script_space;
    width!($globals, $x) += script_space!($globals);
    // if odd(cur_style) then clr:=sup3(cur_size)
    if $globals.cur_style.is_odd() {
        $clr = sup3!($globals, $globals.cur_size.get());
    }
    // else if cur_style<text_style then clr:=sup1(cur_size)
    else if $globals.cur_size.get() < style_node_subtype::text_style as _ {
        $clr = sup1!($globals, $globals.cur_size.get());
    }
    // else clr:=sup2(cur_size);
    else {
        $clr = sup2!($globals, $globals.cur_size.get());
    }
    // if shift_up<clr then shift_up:=clr;
    if $shift_up < $clr {
        $shift_up = $clr;
    }
    // clr:=depth(x)+(abs(math_x_height(cur_size)) div 4);
    $clr = depth!($globals, $x)
        + scaled::new_from_inner(
            math_x_height!($globals, $globals.cur_size.get())
                .abs()
                .inner()
                / 4,
        );
    // if shift_up<clr then shift_up:=clr;
    if $shift_up < $clr {
        $shift_up = $clr;
    }
    // end
    use crate::pascal::IsOddOrEven;
    use crate::section_0101::scaled;
    use crate::section_0135::depth;
    use crate::section_0135::width;
    use crate::section_0247::script_space;
    use crate::section_0681::supscr;
    use crate::section_0688::style_node_subtype;
    use crate::section_0700::math_x_height;
    use crate::section_0700::sup1;
    use crate::section_0700::sup2;
    use crate::section_0700::sup3;
    use crate::section_0702::sup_style;
    use crate::section_0720::clean_box;
}}
