//! @ When we get to the present part of the program, |x| is the natural width
//! of the box being packaged.
//
// @<Determine the value of |width(r)| and the appropriate glue setting...@>=

pub(crate) macro Determine_the_value_of_width_r_and_the_appropriate_glue_setting__then_return_or_goto_common_ending($globals:expr, $m:expr, $r:expr, $w:expr, $x:expr, $q:expr, $lbl_common_ending:lifetime) {{
    // if m=additional then w:=x+w;
    if $m == additional {
        $w = $x + $w;
    }
    // width(r):=w; x:=w-x; {now |x| is the excess to be made up}
    width!($globals, $r) = $w;
    $x = $w - $x;
    /// now `x` is the excess to be made up
    const _: () = ();
    // if x=0 then
    if $x == scaled::zero() {
        // begin glue_sign(r):=normal; glue_order(r):=normal;
        glue_sign!($globals, $r) = glue_sign::normal as _;
        glue_order!($globals, $r) = glue_ord::normal as _;
        // set_glue_ratio_zero(glue_set(r));
        set_glue_ratio_zero!(glue_set!($globals, $r));
        // return;
        crate::return_nojump!($r);
        // end
    }
    // else if x>0 then @<Determine horizontal glue stretch setting, then |return|
    //     or \hbox{|goto common_ending|}@>
    else if $x > scaled::zero() {
        crate::section_0658::Determine_horizontal_glue_stretch_setting__then_return_or_goto_common_ending!(
            $globals,
            $r,
            $x,
            $lbl_common_ending
        );
    }
    // else @<Determine horizontal glue shrink setting, then |return|
    //     or \hbox{|goto common_ending|}@>
    else {
        crate::section_0664::Determine_horizontal_glue_shrink_setting__then_return_or_goto_common_ending!(
            $globals,
            $r,
            $x,
            $q,
            $lbl_common_ending
        );
    }

    use crate::section_0101::scaled;
    use crate::section_0109::set_glue_ratio_zero;
    use crate::section_0135::glue_order;
    use crate::section_0135::glue_set;
    use crate::section_0135::glue_sign;
    use crate::section_0135::width;
    use crate::section_0150::glue_ord;
    use crate::section_0644::additional;
}}
