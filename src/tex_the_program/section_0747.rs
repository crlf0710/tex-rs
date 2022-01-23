//! ` `
// @<Construct a vlist box for the fraction...@>=
pub(crate) macro Construct_a_vlist_box_for_the_fraction__according_to_shift_up_and_shift_down($globals:expr, $q:expr, $x:expr, $z:expr, $v:expr, $shift_up:expr, $shift_down:expr, $delta:expr) {{
    /// temporary registers for box construction
    let (mut p, y);
    // v:=new_null_box; type(v):=vlist_node;
    $v = new_null_box($globals)?;
    r#type!($globals, $v) = vlist_node;
    // height(v):=shift_up+height(x); depth(v):=depth(z)+shift_down;
    height!($globals, $v) = $shift_up + height!($globals, $x);
    depth!($globals, $v) = depth!($globals, $z) + $shift_down;
    // width(v):=width(x); {this also equals |width(z)|}
    width!($globals, $v) = width!($globals, $x);
    /// this also equals `width(z)`
    const _: () = ();
    // if thickness(q)=0 then
    if thickness!($globals, $q) == scaled::zero() {
        // begin p:=new_kern((shift_up-depth(x))-(height(z)-shift_down));
        // link(p):=z;
        // end
        todo!("thickness(q)=0");
    }
    // else  begin y:=fraction_rule(thickness(q));@/
    else {
        y = fraction_rule($globals, thickness!($globals, $q))?;
        // p:=new_kern((axis_height(cur_size)-delta)-@|(height(z)-shift_down));@/
        p = new_kern(
            $globals,
            (axis_height!($globals, $globals.cur_size.get()) - $delta)
                - (height!($globals, $z) - $shift_down),
        )?;
        // link(y):=p; link(p):=z;@/
        link!($globals, y) = p;
        link!($globals, p) = $z;
        // p:=new_kern((shift_up-depth(x))-(axis_height(cur_size)+delta));
        p = new_kern(
            $globals,
            ($shift_up - depth!($globals, $x))
                - (axis_height!($globals, $globals.cur_size.get()) + $delta),
        )?;
        // link(p):=y;
        link!($globals, p) = y;
        // end;
    }
    // link(x):=p; list_ptr(v):=x
    link!($globals, $x) = p;
    list_ptr!($globals, $v) = $x;
    use crate::section_0101::scaled;
    use crate::section_0118::link;
    use crate::section_0133::r#type;
    use crate::section_0135::depth;
    use crate::section_0135::height;
    use crate::section_0135::list_ptr;
    use crate::section_0135::width;
    use crate::section_0136::new_null_box;
    use crate::section_0137::vlist_node;
    use crate::section_0156::new_kern;
    use crate::section_0683::thickness;
    use crate::section_0700::axis_height;
    use crate::section_0704::fraction_rule;
}}
