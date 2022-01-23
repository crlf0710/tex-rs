//! @ The |var_delimiter| function, which finds or constructs a sufficiently
//! large delimiter, is the most interesting of the auxiliary functions that
//! currently concern us. Given a pointer |d| to a delimiter field in some noad,
//! together with a size code |s| and a vertical distance |v|, this function
//! returns a pointer to a box that contains the smallest variant of |d| whose
//! height plus depth is |v| or more. (And if no variant is large enough, it
//! returns the largest available variant.) In particular, this routine will
//! construct arbitrarily large delimiters from extensible components, if
//! |d| leads to such characters.
//!
//! The value returned is a box whose |shift_amount| has been set so that
//! the box is vertically centered with respect to the axis in the given size.
//! If a built-up symbol is returned, the height of the box before shifting
//! will be the height of its topmost component.
//
// @p@t\4@>@<Declare subprocedures for |var_delimiter|@>
// function var_delimiter(@!d:pointer;@!s:small_number;@!v:scaled):pointer;
#[allow(unused_variables, unused_assignments)]
pub(crate) fn var_delimiter(
    globals: &mut TeXGlobals,
    d: pointer,
    s: small_number,
    v: scaled,
) -> TeXResult<pointer> {
    // label found,continue;
    // var b:pointer; {the box that will be constructed}
    /// the box that will be constructed
    let b;
    // @!f,@!g: internal_font_number; {best-so-far and tentative font codes}
    /// best-so-far and tentative font codes
    let mut f: internal_font_number;
    // @!c,@!x,@!y: quarterword; {best-so-far and tentative character codes}
    /// best-so-far and tentative character codes
    let mut c = ASCII_code::from(0);
    let mut x;
    // @!m,@!n: integer; {the number of extensible pieces}
    // @!u: scaled; {height-plus-depth of a tentative character}
    // @!w: scaled; {largest height-plus-depth so far}
    /// largest height-plus-depth so far
    let mut w;
    // @!q: four_quarters; {character info}
    /// character info
    let mut q = NULL_CHARACTER;
    // @!hd: eight_bits; {height-depth byte}
    // @!r: four_quarters; {extensible pieces}
    // @!z: small_number; {runs through font family members}
    /// runs through font family members
    let mut z: integer;
    // @!large_attempt: boolean; {are we trying the ``large'' variant?}
    /// are we trying the "large" variant?
    let mut large_attempt;
    // begin f:=null_font; w:=0; large_attempt:=false;
    f = null_font;
    w = scaled::zero();
    large_attempt = false;
    // z:=small_fam(d); x:=small_char(d);
    z = small_fam!(globals, d).get() as _;
    x = small_char!(globals, d);
    crate::region_forward_label! {
        |'found|
        {
            // loop@+  begin @<Look at the variants of |(z,x)|; set |f| and |c| whenever
            //     a better character is found; |goto found| as soon as a
            //     large enough variant is encountered@>;
            loop {
                crate::section_0707::Look_at_the_variants_of_z_x__set_f_and_c_whenever_a_better_character_is_found__goto_found_as_soon_as_a_large_enough_variant_is_encountered!(globals, z, x, s, f, c, w, v, q, 'found);
                // if large_attempt then goto found; {there were none large enough}
                if large_attempt {
                    /// there were none large enough
                    const _: () = ();
                    crate::goto_forward_label!('found);
                }
                // large_attempt:=true; z:=large_fam(d); x:=large_char(d);
                large_attempt = true;
                z = large_fam!(globals, d).get() as _;
                x = large_char!(globals, d);
                // end;
            }
        }
        // found: if f<>null_font then
        'found <-
    };
    if f != null_font {
        // @<Make variable |b| point to a box for |(f,c)|@>
        crate::section_0710::Make_variable_b_point_to_a_box_for_f_and_c!(globals, b, q, v, f, c);
    }
    // else  begin b:=new_null_box;
    else {
        b = new_null_box(globals)?;
        // width(b):=null_delimiter_space; {use this width if no delimiter was found}
        /// use this width if no delimiter was found
        const _: () = ();
        width!(globals, b) = null_delimiter_space!(globals);
        // end;
    }
    // shift_amount(b):=half(height(b)-depth(b)) - axis_height(s);
    shift_amount!(globals, b) = scaled::new_from_inner(
        half(height!(globals, b).inner() - depth!(globals, b).inner())
            - axis_height!(globals, s.get()).inner(),
    );
    // var_delimiter:=b;
    crate::ok_nojump!(b)
    // end;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0018::ASCII_code;
use crate::section_0081::TeXResult;
use crate::section_0100::half;
use crate::section_0101::scaled;
use crate::section_0101::small_number;
use crate::section_0115::pointer;
use crate::section_0135::depth;
use crate::section_0135::height;
use crate::section_0135::shift_amount;
use crate::section_0135::width;
use crate::section_0136::new_null_box;
use crate::section_0232::null_font;
use crate::section_0247::null_delimiter_space;
use crate::section_0548::internal_font_number;
use crate::section_0556::NULL_CHARACTER;
use crate::section_0683::large_char;
use crate::section_0683::large_fam;
use crate::section_0683::small_char;
use crate::section_0683::small_fam;
use crate::section_0700::axis_height;
