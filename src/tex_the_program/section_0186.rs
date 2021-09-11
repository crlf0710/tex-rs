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
pub(crate) macro Display_the_value_of_glue_set_p($globals:expr, $p:expr) {{
    /// a glue ratio, as a floating point number
    let g: real;
    // g:=float(glue_set(p));
    g = float!(glue_set!($globals, $p));
    // if (g<>float_constant(0))and(glue_sign(p)<>normal) then
    if g != float_constant!(0)
        && glue_sign!($globals, $p) as integer != glue_sign::normal as integer
    {
        // begin print(", glue set ");
        print($globals, crate::strpool_str!(", glue set ").get() as _);
        // if glue_sign(p)=shrinking then print("- ");
        if glue_sign!($globals, $p) as integer == glue_sign::shrinking as integer {
            print($globals, crate::strpool_str!("- ").get() as _);
        }
        // if abs(mem[p+glue_offset].int)<@'4000000 then print("?.?")
        if $globals.mem[$p + glue_offset as pointer][MEMORY_WORD_INT].abs() < 0o4000000 {
            print($globals, crate::strpool_str!("?.?").get() as _);
        }
        // else if abs(g)>float_constant(20000) then
        else if g.abs() > float_constant!(20000) {
            // begin if g>float_constant(0) then print_char(">")
            if g > float_constant!(0) {
                print_char(
                    make_globals_io_string_log_view!($globals),
                    ASCII_code_literal!(b'>'),
                );
            }
            // else print("< -");
            else {
                print($globals, crate::strpool_str!("< -").get() as _);
            }
            // print_glue(20000*unity,glue_order(p),0);
            print_glue(
                $globals,
                scaled::new_from_inner(20000 * unity.inner()),
                glue_order!($globals, $p) as _,
                str_number::zero(),
            );
            // end
        }
        // else print_glue(round(unity*g),glue_order(p),0);
        else {
            print_glue(
                $globals,
                scaled::new_from_inner(((unity.inner() as real) * g).round() as _),
                glue_order!($globals, $p) as _,
                str_number::zero(),
            );
        }
        // @^real multiplication@>
        // end
    }
    use crate::pascal::integer;
    use crate::pascal::real;
    use crate::section_0004::make_globals_io_string_log_view;
    use crate::section_0018::ASCII_code_literal;
    use crate::section_0038::str_number;
    use crate::section_0058::print_char;
    use crate::section_0059::print;
    use crate::section_0101::scaled;
    use crate::section_0101::unity;
    use crate::section_0109::float;
    use crate::section_0109::float_constant;
    use crate::section_0113::MEMORY_WORD_INT;
    use crate::section_0115::pointer;
    use crate::section_0135::glue_offset;
    use crate::section_0135::glue_order;
    use crate::section_0135::glue_set;
    use crate::section_0135::glue_sign;
    use crate::section_0177::print_glue;
}}
