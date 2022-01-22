//! @ The purpose of |make_scripts(q,delta)| is to attach the subscript and/or
//! superscript of noad |q| to the list that starts at |new_hlist(q)|,
//! given that the subscript and superscript aren't both empty. The superscript
//! will appear to the right of the subscript by a given distance |delta|.
//!
//! We set |shift_down| and |shift_up| to the minimum amounts to shift the
//! baseline of subscripts and superscripts based on the given nucleus.
//
// @<Declare math...@>=
// procedure make_scripts(@!q:pointer;@!delta:scaled);
#[allow(unused_variables, unused_assignments)]
pub(crate) fn make_scripts(globals: &mut TeXGlobals, q: pointer, delta: scaled) -> TeXResult<()> {
    // var p,@!x,@!y,@!z:pointer; {temporary registers for box construction}
    /// temporary registers for box construction
    let (mut p, x, z): (pointer, pointer, pointer);
    // @!shift_up,@!shift_down,@!clr:scaled; {dimensions in the calculation}
    /// dimensions in the calculation
    let (mut shift_up, mut shift_down, mut clr);
    // @!t:small_number; {subsidiary size code}
    /// subsidiary size code
    let t;
    // begin p:=new_hlist(q);
    p = new_hlist!(globals, q) as _;
    // if is_char_node(p) then
    if is_char_node!(globals, p) {
        // begin shift_up:=0; shift_down:=0;
        shift_up = scaled::zero();
        shift_down = scaled::zero();
        // end
    }
    // else  begin z:=hpack(p,natural);
    else {
        z = hpack(globals, p, natural0!(), natural1!())?;
        // if cur_style<script_style then t:=script_size@+else t:=script_script_size;
        if globals.cur_style.get() < style_node_subtype::script_style as _ {
            t = script_size;
        } else {
            t = script_script_size;
        }
        // shift_up:=height(z)-sup_drop(t);
        shift_up = height!(globals, z) - sup_drop!(globals, t);
        // shift_down:=depth(z)+sub_drop(t);
        shift_down = depth!(globals, z) + sub_drop!(globals, t);
        // free_node(z,box_node_size);
        free_node(globals, z, box_node_size as _);
        // end;
    }
    // if math_type(supscr(q))=empty then
    if math_type!(globals, supscr!(q)) == math_type_kind::empty as _ {
        // @<Construct a subscript box |x| when there is no superscript@>
        crate::section_0757::Construct_a_subscript_box_x_when_there_is_no_superscript!(
            globals, x, q, shift_down, clr
        );
    }
    // else  begin @<Construct a superscript box |x|@>;
    else {
        crate::section_0758::Construct_a_superscript_box_x!(globals, x, q, shift_up, clr);
        // if math_type(subscr(q))=empty then shift_amount(x):=-shift_up
        if math_type!(globals, subscr!(q)) == math_type_kind::empty as _ {
            shift_amount!(globals, x) = -shift_up;
        }
        // else @<Construct a sub/superscript combination box |x|, with the
        //   superscript offset by |delta|@>;
        else {
            todo!("construct 3");
        }
        // end;
    }
    // if new_hlist(q)=null then new_hlist(q):=x
    if new_hlist!(globals, q) == null as _ {
        new_hlist!(globals, q) = x as _;
    }
    // else  begin p:=new_hlist(q);
    else {
        p = new_hlist!(globals, q) as _;
        // while link(p)<>null do p:=link(p);
        while link!(globals, p) != null {
            p = link!(globals, p);
        }
        // link(p):=x;
        link!(globals, p) = x;
        // end;
    }
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::scaled;
use crate::section_0115::null;
use crate::section_0115::pointer;
use crate::section_0118::link;
use crate::section_0130::free_node;
use crate::section_0134::is_char_node;
use crate::section_0135::box_node_size;
use crate::section_0135::depth;
use crate::section_0135::height;
use crate::section_0135::shift_amount;
use crate::section_0644::natural0;
use crate::section_0644::natural1;
use crate::section_0649::hpack;
use crate::section_0681::math_type;
use crate::section_0681::math_type_kind;
use crate::section_0681::subscr;
use crate::section_0681::supscr;
use crate::section_0688::style_node_subtype;
use crate::section_0699::script_script_size;
use crate::section_0699::script_size;
use crate::section_0700::sub_drop;
use crate::section_0700::sup_drop;
use crate::section_0725::new_hlist;
