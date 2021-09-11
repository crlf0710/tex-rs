//! ` `

// @<Incorporate box dimensions into the dimensions of the vbox...@>=
pub(crate) macro Incorporate_box_dimensions_into_the_dimensions_of_the_vbox_that_will_contain_it($globals:expr, $p:expr, $w:expr, $d:expr, $x:expr) {{
    /// shift amount
    let s: scaled;

    // begin x:=x+d+height(p); d:=depth(p);
    $x = $x + $d + height!($globals, $p);
    $d = depth!($globals, $p);
    // if type(p)>=rule_node then s:=0 @+else s:=shift_amount(p);
    if r#type!($globals, $p) >= rule_node {
        s = scaled::zero();
    } else {
        s = shift_amount!($globals, $p);
    }
    // if width(p)+s>w then w:=width(p)+s;
    if width!($globals, $p) + s > $w {
        $w = width!($globals, $p) + s;
    }
    // end
    use crate::section_0101::scaled;
    use crate::section_0133::r#type;
    use crate::section_0135::depth;
    use crate::section_0135::height;
    use crate::section_0135::shift_amount;
    use crate::section_0135::width;
    use crate::section_0138::rule_node;
}}
