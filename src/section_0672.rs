//! @ When we get to the present part of the program, |x| is the natural height
//! of the box being packaged.
//
// @<Determine the value of |height(r)| and the appropriate glue setting...@>=
macro_rules! Determine_the_value_of_height_r_and_the_appropriate_glue_setting__then_return_or_goto_common_ending {
    ($globals:expr, $r:expr, $m:expr, $h:expr, $x:expr) => {{
        // if m=additional then h:=x+h;
        if $m == additional {
            $h = $x + $h;
        }
        // height(r):=h; x:=h-x; {now |x| is the excess to be made up}
        height!($globals, $r) = $h;
        $x = $h - $x;
        /// now `x` is the excess to be made up
        const _ : () = ();
        // if x=0 then
        if $x == scaled::zero() {
            // begin glue_sign(r):=normal; glue_order(r):=normal;
            glue_sign!($globals, $r) = glue_sign::normal as _;
            glue_order!($globals, $r) = glue_ord::normal as _;
            // set_glue_ratio_zero(glue_set(r));
            set_glue_ratio_zero!(glue_set!($globals, $r));
            // return;
            return_nojump!($r);
            // end
        }
        // else if x>0 then @<Determine vertical glue stretch setting, then |return|
        //     or \hbox{|goto common_ending|}@>
        else if $x > scaled::zero() {
            Determine_vertical_glue_stretch_setting__then_return_or_goto_common_ending!
                ($globals, $r, $x);
        }
        // else @<Determine vertical glue shrink setting, then |return|
        //     or \hbox{|goto common_ending|}@>
        else {
            todo!("x<0");
        }
        use crate::section_0135::glue_sign;
        use crate::section_0150::glue_ord;
        use crate::section_0644::additional;
    }}
}
