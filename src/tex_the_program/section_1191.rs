//! ` `
// @<Declare act...@>=
// procedure math_left_right;
pub(crate) fn math_left_right(globals: &mut TeXGlobals) -> TeXResult<()> {
    // var t:small_number; {|left_noad| or |right_noad|}
    /// `left_noad` or `right_noad`
    let t;
    // @!p:pointer; {new noad}
    /// new noad
    let mut p;
    // begin t:=cur_chr;
    t = globals.cur_chr.get();
    // if (t=right_noad)and(cur_group<>math_left_group) then
    if t == right_noad as _ && globals.cur_group != math_left_group {
        //   @<Try to recover from mismatched \.{\\right}@>
        todo!("Try to recover");
    }
    // else  begin p:=new_noad; type(p):=t;
    else {
        p = new_noad(globals)?;
        r#type!(globals, p) = t as _;
        // scan_delimiter(delimiter(p),false);
        scan_delimiter(globals, delimiter!(p), false)?;
        // if t=left_noad then
        if t == left_noad as _ {
            // begin push_math(math_left_group); link(head):=p; tail:=p;
            push_math(globals, math_left_group.into());
            link!(globals, head!(globals)) = p;
            tail!(globals) = p;
            // end
        }
        // else  begin p:=fin_mlist(p); unsave; {end of |math_left_group|}
        else {
            p = fin_mlist(globals, p);
            unsave(globals)?;
            /// end of `math_left_group`
            const _: () = ();
            // tail_append(new_noad); type(tail):=inner_noad;
            tail_append!(globals, new_noad(globals)?);
            r#type!(globals, tail!(globals)) = inner_noad as _;
            // math_type(nucleus(tail)):=sub_mlist;
            math_type!(globals, nucleus!(tail!(globals))) = math_type_kind::sub_mlist as _;
            // info(nucleus(tail)):=p;
            info_inner!(globals, nucleus!(tail!(globals))) = p;
            // end;
        }
        // end;
    }
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0118::info_inner;
use crate::section_0118::link;
use crate::section_0133::r#type;
use crate::section_0213::head;
use crate::section_0213::tail;
use crate::section_0214::tail_append;
use crate::section_0269::math_left_group;
use crate::section_0281::unsave;
use crate::section_0681::math_type;
use crate::section_0681::math_type_kind;
use crate::section_0681::nucleus;
use crate::section_0682::inner_noad;
use crate::section_0686::new_noad;
use crate::section_0687::delimiter;
use crate::section_0687::left_noad;
use crate::section_0687::right_noad;
use crate::section_1136::push_math;
use crate::section_1160::scan_delimiter;
use crate::section_1184::fin_mlist;
