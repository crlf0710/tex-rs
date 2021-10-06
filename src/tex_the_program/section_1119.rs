//! ` `

// @<Declare act...@>=
// procedure build_discretionary;
#[allow(unused_variables)]
pub(crate) fn build_discretionary(globals: &mut TeXGlobals) -> TeXResult<()> {
    // label done,exit;
    // var p,@!q:pointer; {for link manipulation}
    /// for link manipulation
    let p;
    // @!n:integer; {length of discretionary list}
    // begin unsave;
    unsave(globals)?;
    // @<Prune the current list, if necessary, until it contains only
    //   |char_node|, |kern_node|, |hlist_node|, |vlist_node|, |rule_node|,
    //   and |ligature_node| items; set |n| to the length of the list,
    //   and set |q| to the list's tail@>;
    todo!("prune list");
    // p:=link(head); pop_nest;
    p = link!(globals, head!(globals));
    pop_nest(globals);
    // case saved(-1) of
    match saved!(globals, @neg 1) {
        // 0:pre_break(tail):=p;
        0 => {
            pre_break!(globals, tail!(globals)) = p;
        }
        // 1:post_break(tail):=p;
        1 => {
            post_break!(globals, tail!(globals)) = p;
        }
        // 2:@<Attach list |p| to the current list, and record its length;
        //   then finish up and |return|@>;
        2 => {
            todo!("attach list");
        }
        // end; {there are no other cases}
        _ => {
            unreachable!();
        }
    }
    // incr(saved(-1)); new_save_level(disc_group); scan_left_brace;
    incr!(saved!(globals, @neg 1));
    new_save_level(globals, disc_group.into());
    scan_left_brace(globals)?;
    // push_nest; mode:=-hmode; space_factor:=1000;
    push_nest(globals);
    mode!(globals) = (-hmode).into();
    space_factor!(globals) = 1000;
    // exit:end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0016::incr;
use crate::section_0081::TeXResult;
use crate::section_0118::link;
use crate::section_0145::post_break;
use crate::section_0145::pre_break;
use crate::section_0211::hmode;
use crate::section_0213::head;
use crate::section_0213::mode;
use crate::section_0213::space_factor;
use crate::section_0213::tail;
use crate::section_0216::push_nest;
use crate::section_0217::pop_nest;
use crate::section_0269::disc_group;
use crate::section_0274::new_save_level;
use crate::section_0274::saved;
use crate::section_0281::unsave;
use crate::section_0403::scan_left_brace;
