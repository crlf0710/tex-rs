//! @ When we get to the present part of the program, |x| is the natural height
//! of the box being packaged.
//
// @<Determine the value of |height(r)| and the appropriate glue setting...@>=
macro_rules! Determine_the_value_of_height_r_and_the_appropriate_glue_setting__then_return_or_goto_common_ending {
    ($globals:expr) => {{
        // if m=additional then h:=x+h;
        // height(r):=h; x:=h-x; {now |x| is the excess to be made up}
        // if x=0 then
        //   begin glue_sign(r):=normal; glue_order(r):=normal;
        //   set_glue_ratio_zero(glue_set(r));
        //   return;
        //   end
        // else if x>0 then @<Determine vertical glue stretch setting, then |return|
        //     or \hbox{|goto common_ending|}@>
        // else @<Determine vertical glue shrink setting, then |return|
        //     or \hbox{|goto common_ending|}@>
        todo!("determine");
    }}
}
