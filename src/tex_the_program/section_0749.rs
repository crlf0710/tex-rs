//! @ If the nucleus of an |op_noad| is a single character, it is to be
//! centered vertically with respect to the axis, after first being enlarged
//! (via a character list in the font) if we are in display style.  The normal
//! convention for placing displayed limits is to put them above and below the
//! operator in display style.
//!
//! The italic correction is removed from the character if there is a subscript
//! and the limits are not being displayed. The |make_op|
//! routine returns the value that should be used as an offset between
//! subscript and superscript.
//!
//! After |make_op| has acted, |subtype(q)| will be |limits| if and only if
//! the limits have been set above and below the operator. In that case,
//! |new_hlist(q)| will already contain the desired final box.
//
// @<Declare math...@>=
// function make_op(@!q:pointer):scaled;
pub(crate) fn make_op(globals: &mut TeXGlobals, q: pointer) -> TeXResult<scaled> {
    // var delta:scaled; {offset between subscript and superscript}
    /// offset between subscript and superscript
    let delta;
    // @!p,@!v,@!x,@!y,@!z:pointer; {temporary registers for box construction}
    /// temporary registers for box construction
    let (x,);
    // @!c:quarterword;@+@!i:four_quarters; {registers for character examination}
    /// registers for character examination
    let (c, i);
    // @!shift_up,@!shift_down:scaled; {dimensions for box calculation}
    // begin if (subtype(q)=normal)and(cur_style<text_style) then
    if subtype!(globals, q) == op_noad_subtype::normal as _
        && globals.cur_style.get() < style_node_subtype::text_style.get()
    {
        // subtype(q):=limits;
        subtype!(globals, q) = op_noad_subtype::limits as _;
    }
    // if math_type(nucleus(q))=math_char then
    if math_type!(globals, nucleus!(q)) == math_type_kind::math_char as _ {
        // begin fetch(nucleus(q));
        let mut fetched = fetch(globals, nucleus!(q));
        // if (cur_style<text_style)and(char_tag(cur_i)=list_tag) then {make it larger}
        if globals.cur_style.get() < style_node_subtype::text_style.get()
            && fetched.cur_i.char_tag() == char_tag::list_tag
        {
            /// make it larger
            const _: () = ();
            // begin c:=rem_byte(cur_i); i:=char_info(cur_f)(c);
            c = ASCII_code::from(fetched.cur_i.rem_byte() as integer);
            i = char_info!(globals, fetched.cur_f, c.numeric_value());
            // if char_exists(i) then
            if i.char_exists() {
                // begin cur_c:=c; cur_i:=i; character(nucleus(q)):=c;
                fetched.cur_c = c;
                fetched.cur_i = i;
                let fam = fam!(globals, nucleus!(q));
                assign_fam_and_character!(globals, nucleus!(q), fam, c);
                // end;
            }
            // end;
        }
        // delta:=char_italic(cur_f)(cur_i); x:=clean_box(nucleus(q),cur_style);
        delta = char_italic!(globals, fetched.cur_f, fetched.cur_i);
        x = clean_box(globals, nucleus!(q), globals.cur_style)?;
        // if (math_type(subscr(q))<>empty)and(subtype(q)<>limits) then
        if math_type!(globals, subscr!(q)) != math_type_kind::empty as _
            && subtype!(globals, q) != op_noad_subtype::limits as _
        {
            // width(x):=width(x)-delta; {remove italic correction}
            /// remove italic correction
            const _: () = ();
            width!(globals, x) -= delta;
        }
        // shift_amount(x):=half(height(x)-depth(x)) - axis_height(cur_size);
        //   {center vertically}
        /// center vertically
        const _: () = ();
        shift_amount!(globals, x) = (height!(globals, x) - depth!(globals, x)).half()
            - axis_height!(globals, globals.cur_size.get());
        // math_type(nucleus(q)):=sub_box; info(nucleus(q)):=x;
        math_type!(globals, nucleus!(q)) = math_type_kind::sub_box as _;
        info_inner!(globals, nucleus!(q)) = x;
        // end
    }
    // else delta:=0;
    else {
        delta = scaled::zero();
    }
    // if subtype(q)=limits then
    if subtype!(globals, q) == op_noad_subtype::limits as _ {
        // @<Construct a box with limits above and below it, skewed by |delta|@>;
        crate::section_0750::Construct_a_box_with_limits_above_and_below_it__skewed_by_delta!(
            globals, q, delta
        );
    }
    // make_op:=delta;
    crate::ok_nojump!(delta)
    // end;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0018::ASCII_code;
use crate::section_0081::TeXResult;
use crate::section_0100::Half;
use crate::section_0101::scaled;
use crate::section_0115::pointer;
use crate::section_0118::info_inner;
use crate::section_0133::subtype;
use crate::section_0135::depth;
use crate::section_0135::height;
use crate::section_0135::shift_amount;
use crate::section_0135::width;
use crate::section_0544::char_tag;
use crate::section_0554::char_info;
use crate::section_0554::char_italic;
use crate::section_0681::assign_fam_and_character;
use crate::section_0681::fam;
use crate::section_0681::math_type;
use crate::section_0681::math_type_kind;
use crate::section_0681::nucleus;
use crate::section_0681::subscr;
use crate::section_0682::op_noad_subtype;
use crate::section_0688::style_node_subtype;
use crate::section_0700::axis_height;
use crate::section_0720::clean_box;
use crate::section_0722::fetch;
