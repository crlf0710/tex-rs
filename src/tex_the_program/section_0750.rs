//! @ The following program builds a vlist box |v| for displayed limits. The
//! width of the box is not affected by the fact that the limits may be skewed.
//
// @<Construct a box with limits above and below it...@>=
pub(crate) macro Construct_a_box_with_limits_above_and_below_it__skewed_by_delta($globals:expr, $q:expr, $delta:expr) {{
    /// temporary registers for box construction
    let (v, mut x, mut y, mut z);
    // begin x:=clean_box(supscr(q),sup_style(cur_style));
    x = clean_box($globals, supscr!($q), sup_style!($globals.cur_style).into())?;
    // y:=clean_box(nucleus(q),cur_style);
    y = clean_box($globals, nucleus!($q), $globals.cur_style)?;
    // z:=clean_box(subscr(q),sub_style(cur_style));
    z = clean_box($globals, subscr!($q), sub_style!($globals.cur_style).into())?;
    // v:=new_null_box; type(v):=vlist_node; width(v):=width(y);
    v = new_null_box($globals)?;
    r#type!($globals, v) = vlist_node;
    width!($globals, v) = width!($globals, y);
    // if width(x)>width(v) then width(v):=width(x);
    if width!($globals, x) > width!($globals, v) {
        width!($globals, v) = width!($globals, x);
    }
    // if width(z)>width(v) then width(v):=width(z);
    if width!($globals, z) > width!($globals, v) {
        width!($globals, v) = width!($globals, z);
    }
    // x:=rebox(x,width(v)); y:=rebox(y,width(v)); z:=rebox(z,width(v));@/
    x = rebox($globals, x, width!($globals, v))?;
    y = rebox($globals, y, width!($globals, v))?;
    z = rebox($globals, z, width!($globals, v))?;
    // shift_amount(x):=half(delta); shift_amount(z):=-shift_amount(x);
    shift_amount!($globals, x) = $delta.half();
    shift_amount!($globals, z) = -shift_amount!($globals, x);
    // height(v):=height(y); depth(v):=depth(y);
    height!($globals, v) = height!($globals, y);
    depth!($globals, v) = depth!($globals, y);
    // @<Attach the limits to |y| and adjust |height(v)|, |depth(v)| to
    //   account for their presence@>;
    crate::section_0751::Attach_the_limits_to_y_and_adjust_height_v__depth_v_to_account_for_their_presence!($globals, $q, x, y, z, v);
    // new_hlist(q):=v;
    new_hlist!($globals, $q) = v as _;
    // end
    use crate::section_0100::Half;
    use crate::section_0133::r#type;
    use crate::section_0135::depth;
    use crate::section_0135::height;
    use crate::section_0135::shift_amount;
    use crate::section_0135::width;
    use crate::section_0136::new_null_box;
    use crate::section_0137::vlist_node;
    use crate::section_0681::nucleus;
    use crate::section_0681::subscr;
    use crate::section_0681::supscr;
    use crate::section_0702::sub_style;
    use crate::section_0702::sup_style;
    use crate::section_0715::rebox;
    use crate::section_0720::clean_box;
    use crate::section_0725::new_hlist;
}}
