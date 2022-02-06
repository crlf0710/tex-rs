//! @ When both subscript and superscript are present, the subscript must be
//! separated from the superscript by at least four times |default_rule_thickness|.
//! If this condition would be violated, the subscript moves down, after which
//! both subscript and superscript move up so that the bottom of the superscript
//! is at least as high as the baseline plus four-fifths of the x-height.
//
// @<Construct a sub/superscript combination box |x|...@>=
pub(crate) macro Construct_a_sub_superscript_combination_box_x__with_the_superscript_offset_by_delta($globals:expr, $p:expr, $x:expr, $z:expr, $q:expr, $clr:expr, $delta:expr, $shift_up:expr, $shift_down:expr) {{
    /// temporary registers for box construction
    let y;
    // begin y:=clean_box(subscr(q),sub_style(cur_style));
    y = clean_box($globals, subscr!($q), sub_style!($globals.cur_style).into())?;
    // width(y):=width(y)+script_space;
    width!($globals, y) += script_space!($globals);
    // if shift_down<sub2(cur_size) then shift_down:=sub2(cur_size);
    if $shift_down < sub2!($globals, $globals.cur_size.get()) {
        $shift_down = sub2!($globals, $globals.cur_size.get())
    }
    // clr:=4*default_rule_thickness-
    //   ((shift_up-depth(x))-(height(y)-shift_down));
    $clr = scaled::new_from_inner(4 * default_rule_thickness!($globals).inner())
        - (($shift_up - depth!($globals, $x)) - (height!($globals, y) - $shift_down));
    // if clr>0 then
    if $clr > scaled::zero() {
        // begin shift_down:=shift_down+clr;
        $shift_down += $clr;
        // clr:=(abs(math_x_height(cur_size)*4) div 5)-(shift_up-depth(x));
        $clr = scaled::new_from_inner(
            math_x_height!($globals, $globals.cur_size.get())
                .inner()
                .abs()
                / 5,
        ) - ($shift_up - depth!($globals, $x));
        // if clr>0 then
        if $clr > scaled::zero() {
            // begin shift_up:=shift_up+clr;
            $shift_up += $clr;
            // shift_down:=shift_down-clr;
            $shift_down -= $clr;
            // end;
        }
        // end;
    }
    // shift_amount(x):=delta; {superscript is |delta| to the right of the subscript}
    /// superscript is `delta` to the right of the subscript
    const _: () = ();
    shift_amount!($globals, $x) = $delta;
    // p:=new_kern((shift_up-depth(x))-(height(y)-shift_down)); link(x):=p; link(p):=y;
    $p = new_kern(
        $globals,
        ($shift_up - depth!($globals, $x) - (height!($globals, y) - $shift_down)),
    )?;
    link!($globals, $x) = $p;
    link!($globals, $p) = y;
    // x:=vpack(x,natural); shift_amount(x):=shift_down;
    $x = vpack($globals, $x, natural0!(), natural1!())?;
    shift_amount!($globals, $x) = $shift_down;
    // end
    use crate::section_0101::scaled;
    use crate::section_0118::link;
    use crate::section_0135::depth;
    use crate::section_0135::height;
    use crate::section_0135::shift_amount;
    use crate::section_0135::width;
    use crate::section_0156::new_kern;
    use crate::section_0247::script_space;
    use crate::section_0644::natural0;
    use crate::section_0644::natural1;
    use crate::section_0668::vpack;
    use crate::section_0681::subscr;
    use crate::section_0700::math_x_height;
    use crate::section_0700::sub2;
    use crate::section_0701::default_rule_thickness;
    use crate::section_0702::sub_style;
    use crate::section_0720::clean_box;
}}
