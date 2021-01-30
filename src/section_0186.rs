//! @ The code will have to change in this place if |glue_ratio| is
//! a structured type instead of an ordinary |real|. Note that this routine
//! should avoid arithmetic errors even if the |glue_set| field holds an
//! arbitrary random value. The following code assumes that a properly
//! formed nonzero |real| number has absolute value $2^{20}$ or more when
//! it is regarded as an integer; this precaution was adequate to prevent
//! floating point underflow on the author's computer.
//! @^system dependencies@>
//! @^dirty \PASCAL@>
//
// @<Display the value of |glue_set(p)|@>=
macro_rules! Display_the_value_of_glue_set_p {
    ($globals:expr, $p:expr) => {{
        /// a glue ratio, as a floating point number
        let g: real;
        // g:=float(glue_set(p));
        g = float!(glue_set!($globals, $p));
        // if (g<>float_constant(0))and(glue_sign(p)<>normal) then
        if g != float_constant!(0)
            && glue_sign!($globals, $p) as integer != glue_sign::normal as integer
        {
            // begin print(", glue set ");
            print($globals, strpool_str!(", glue set ").get() as _);
            todo!("display glue_set(p)");
            // if glue_sign(p)=shrinking then print("- ");
            // if abs(mem[p+glue_offset].int)<@'4000000 then print("?.?")
            // else if abs(g)>float_constant(20000) then
            //   begin if g>float_constant(0) then print_char(">")
            //   else print("< -");
            //   print_glue(20000*unity,glue_order(p),0);
            //   end
            // else print_glue(round(unity*g),glue_order(p),0);
            // @^real multiplication@>
            // end
        }
        use crate::pascal::real;
        use crate::section_0135::glue_sign;
    }};
}
