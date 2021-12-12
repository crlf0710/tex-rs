//! @ A ligature found in a math formula does not create a |ligature_node|, because
//! there is no question of hyphenation afterwards; the ligature will simply be
//! stored in an ordinary |char_node|, after residing in an |ord_noad|.
//!
//! The |math_type| is converted to |math_text_char| here if we would not want to
//! apply an italic correction to the current character unless it belongs
//! to a math font (i.e., a font with |space=0|).
//!
//! No boundary characters enter into these ligatures.
//
// @<Declare math...@>=
// procedure make_ord(@!q:pointer);
pub(crate) fn make_ord(globals: &mut TeXGlobals, q: pointer) -> TeXResult<()> {
    // label restart,exit;
    // var a:integer; {address of lig/kern instruction}
    // @!p,@!r:pointer; {temporary registers for list manipulation}
    /// temporary registers for list manipulation
    let (p,);
    // begin restart:@t@>@;@/
    // if math_type(subscr(q))=empty then if math_type(supscr(q))=empty then
    //  if math_type(nucleus(q))=math_char then
    if math_type!(globals, subscr!(q)) == math_type_kind::empty as _
        && math_type!(globals, supscr!(q)) == math_type_kind::empty as _
        && math_type!(globals, nucleus!(q)) == math_type_kind::math_char as _
    {
        // begin p:=link(q);
        p = link!(globals, q);
        // if p<>null then if (type(p)>=ord_noad)and(type(p)<=punct_noad) then
        //   if math_type(nucleus(p))=math_char then
        //   if fam(nucleus(p))=fam(nucleus(q)) then
        if p != null
            && r#type!(globals, p) >= ord_noad
            && r#type!(globals, p) <= punct_noad
            && math_type!(globals, nucleus!(p)) == math_type_kind::math_char as _
            && fam!(globals, nucleus!(p)) == fam!(globals, nucleus!(q))
        {
            // begin math_type(nucleus(q)):=math_text_char;
            math_type!(globals, nucleus!(q)) = math_type_kind::math_text_char as _;
            // fetch(nucleus(q));
            // if char_tag(cur_i)=lig_tag then
            //   begin a:=lig_kern_start(cur_f)(cur_i);
            //   cur_c:=character(nucleus(p));
            //   cur_i:=font_info[a].qqqq;
            //   if skip_byte(cur_i)>stop_flag then
            //     begin a:=lig_kern_restart(cur_f)(cur_i);
            //     cur_i:=font_info[a].qqqq;
            //     end;
            //   loop@+ begin @<If instruction |cur_i| is a kern with |cur_c|, attach
            //       the kern after~|q|; or if it is a ligature with |cur_c|, combine
            //       noads |q| and~|p| appropriately; then |return| if the cursor has
            //       moved past a noad, or |goto restart|@>;
            //     if skip_byte(cur_i)>=stop_flag then return;
            //     a:=a+qo(skip_byte(cur_i))+1;
            //     cur_i:=font_info[a].qqqq;
            //     end;
            //   end;
            // end;
            todo!("mark math_text_char");
        }
        // end;
    }
    // exit:end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0115::null;
use crate::section_0115::pointer;
use crate::section_0118::link;
use crate::section_0133::r#type;
use crate::section_0681::fam;
use crate::section_0681::math_type;
use crate::section_0681::math_type_kind;
use crate::section_0681::nucleus;
use crate::section_0681::subscr;
use crate::section_0681::supscr;
use crate::section_0682::*;
