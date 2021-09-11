//! @ The code here implicitly uses the fact that running dimensions are
//! indicated by |null_flag|, which will be ignored in the calculations
//! because it is a highly negative number.
//
// @<Incorporate box dimensions into the dimensions of the hbox...@>=
pub(crate) macro Incorporate_box_dimensions_into_the_dimensions_of_the_hbox_that_will_contain_it($globals:expr, $p:expr, $h:expr, $d:expr, $x:expr) {{
    /// shift amount
    let s: scaled;
    // begin x:=x+width(p);
    $x = $x + width!($globals, $p);
    // if type(p)>=rule_node then s:=0 @+else s:=shift_amount(p);
    if r#type!($globals, $p) >= rule_node {
        s = scaled::zero();
    } else {
        s = shift_amount!($globals, $p);
    }
    // if height(p)-s>h then h:=height(p)-s;
    if height!($globals, $p) - s > $h {
        $h = height!($globals, $p) - s;
    }
    // if depth(p)+s>d then d:=depth(p)+s;
    if depth!($globals, $p) + s > $d {
        $d = depth!($globals, $p) + s;
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
