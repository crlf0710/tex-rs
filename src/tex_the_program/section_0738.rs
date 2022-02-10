//! @ Slants are not considered when placing accents in math mode. The accenter is
//! centered over the accentee, and the accent width is treated as zero with
//! respect to the size of the final box.
//
// @<Declare math...@>=
// procedure make_math_accent(@!q:pointer);
#[allow(unused_variables, unused_assignments)]
pub(crate) fn make_math_accent(globals: &mut TeXGlobals, q: pointer) -> TeXResult<()> {
    // label done,done1;
    // var p,@!x,@!y:pointer; {temporary registers for box construction}
    /// temporary registers for box construction
    let (mut p, mut x, mut y);
    // @!a:integer; {address of lig/kern instruction}
    // @!c:quarterword; {accent character}
    /// accent character
    let mut c;
    // @!f:internal_font_number; {its font}
    /// its font
    let f;
    // @!i:four_quarters; {its |char_info|}
    /// its `char_info`
    let mut i;
    // @!s:scaled; {amount to skew the accent to the right}
    /// amount to skew the accent to the right
    let mut s: scaled;
    // @!h:scaled; {height of character being accented}
    /// height of character being accented
    let mut h;
    // @!delta:scaled; {space to remove between accent and accentee}
    /// space to remove between accent and accentee
    let mut delta;
    // @!w:scaled; {width of the accentee, not including sub/superscripts}
    /// width of the accentee, not including sub/superscripts
    let w;
    // begin fetch(accent_chr(q));
    let fetched = fetch(globals, accent_chr!(q));
    // if char_exists(cur_i) then
    if fetched.cur_i.char_exists() {
        // begin i:=cur_i; c:=cur_c; f:=cur_f;@/
        i = fetched.cur_i;
        c = fetched.cur_c;
        f = fetched.cur_f;
        // @<Compute the amount of skew@>;
        crate::section_0741::Compute_the_amount_of_skew!(globals, q, s);
        // x:=clean_box(nucleus(q),cramped_style(cur_style)); w:=width(x); h:=height(x);
        x = clean_box(
            globals,
            nucleus!(q),
            cramped_style!(globals.cur_style).into(),
        )?;
        w = width!(globals, x);
        h = height!(globals, x);
        // @<Switch to a larger accent if available and appropriate@>;
        crate::section_0740::Switch_to_a_larger_accent_if_available_and_appropriate!(
            globals, i, c, f, w
        );
        // if h<x_height(f) then delta:=h@+else delta:=x_height(f);
        if h < x_height!(globals, f) {
            delta = h;
        } else {
            delta = x_height!(globals, f);
        }
        // if (math_type(supscr(q))<>empty)or(math_type(subscr(q))<>empty) then
        if math_type!(globals, supscr!(q)) != math_type_kind::empty as _
            || math_type!(globals, subscr!(q)) != math_type_kind::empty as _
        {
            // if math_type(nucleus(q))=math_char then
            if math_type!(globals, nucleus!(q)) == math_type_kind::math_char as _ {
                // @<Swap the subscript and superscript into box |x|@>;
                crate::section_0742::Swap_the_subscript_and_superscript_into_box_x!(
                    globals, q, x, h, delta
                );
            }
        }
        // y:=char_box(f,c);
        y = char_box(globals, f, c)?;
        // shift_amount(y):=s+half(w-width(y));
        shift_amount!(globals, y) = s + (w - width!(globals, y)).half();
        // width(y):=0; p:=new_kern(-delta); link(p):=x; link(y):=p;
        width!(globals, y) = scaled::zero();
        p = new_kern(globals, -delta)?;
        link!(globals, p) = x;
        link!(globals, y) = p;
        // y:=vpack(y,natural); width(y):=width(x);
        y = vpack(globals, y, natural0!(), natural1!())?;
        width!(globals, y) = width!(globals, x);
        // if height(y)<h then @<Make the height of box |y| equal to |h|@>;
        if height!(globals, y) < h {
            crate::section_0739::Make_the_height_of_box_y_equal_to_h!(globals, y, h, p);
        }
        // info(nucleus(q)):=y;
        info_inner!(globals, nucleus!(q)) = y;
        // math_type(nucleus(q)):=sub_box;
        math_type!(globals, nucleus!(q)) = math_type_kind::sub_box as _;
        // end;
    }
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0100::Half;
use crate::section_0101::scaled;
use crate::section_0115::pointer;
use crate::section_0118::info_inner;
use crate::section_0118::link;
use crate::section_0135::height;
use crate::section_0135::shift_amount;
use crate::section_0135::width;
use crate::section_0156::new_kern;
use crate::section_0558::x_height;
use crate::section_0644::natural0;
use crate::section_0644::natural1;
use crate::section_0668::vpack;
use crate::section_0681::math_type;
use crate::section_0681::math_type_kind;
use crate::section_0681::nucleus;
use crate::section_0681::subscr;
use crate::section_0681::supscr;
use crate::section_0687::accent_chr;
use crate::section_0702::cramped_style;
use crate::section_0709::char_box;
use crate::section_0720::clean_box;
use crate::section_0722::fetch;
