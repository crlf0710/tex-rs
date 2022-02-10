//! ` `
// @<Declare act...@>=
// procedure after_math;
#[allow(unused_variables, unused_assignments)]
pub(crate) fn after_math(globals: &mut TeXGlobals) -> TeXResult<()> {
    // var l:boolean; {`\.{\\leqno}' instead of `\.{\\eqno}'}
    /// `\leqno' instead of `\eqno`
    let mut l;
    // @!danger:boolean; {not enough symbol fonts are present}
    /// not enough symbol fonts are present
    let mut danger;
    // @!m:integer; {|mmode| or |-mmode|}
    /// `mmode` or `-mmode`
    let mut m;
    // @!p:pointer; {the formula}
    /// the formula
    let mut p;
    // @!a:pointer; {box containing equation number}
    /// box containing equation number
    let a;
    // @<Local variables for finishing a displayed formula@>@;
    // begin danger:=false;
    danger = false;
    // @<Check that the necessary fonts for math symbols are present;
    //   if not, flush the current math lists and set |danger:=true|@>;
    crate::section_1195::Check_that_the_necessary_fonts_for_math_symbols_are_present__if_not__flush_the_current_math_lists_and_set_danger_to_true!(
        globals, danger
    );
    // m:=mode; l:=false; p:=fin_mlist(null); {this pops the nest}
    m = mode!(globals);
    l = false;
    /// this pops the nest
    const _: () = ();
    p = fin_mlist(globals, null);
    // if mode=-m then {end of equation number}
    if mode!(globals) == -m {
        /// end of equation number
        const _: () = ();
        // begin @<Check that another \.\$ follows@>;
        crate::section_1197::Check_that_another_dollar_follows!(globals);
        // cur_mlist:=p; cur_style:=text_style; mlist_penalties:=false;
        globals.cur_mlist = p;
        globals.cur_style = style_node_subtype::text_style.get().into();
        globals.mlist_penalties = false;
        // mlist_to_hlist; a:=hpack(link(temp_head),natural);
        mlist_to_hlist(globals)?;
        a = hpack(globals, link!(globals, temp_head), natural0!(), natural1!())?;
        // unsave; decr(save_ptr); {now |cur_group=math_shift_group|}
        unsave(globals)?;
        decr!(globals.save_ptr);
        /// now `cur_group=math_shift_group`
        const _: () = ();
        // if saved(0)=1 then l:=true;
        if saved!(globals, 0) == 1 {
            l = true;
        }
        // danger:=false;
        danger = false;
        // @<Check that the necessary fonts for math symbols are present;
        //   if not, flush the current math lists and set |danger:=true|@>;
        crate::section_1195::Check_that_the_necessary_fonts_for_math_symbols_are_present__if_not__flush_the_current_math_lists_and_set_danger_to_true!(
            globals, danger
        );
        // m:=mode; p:=fin_mlist(null);
        m = mode!(globals);
        p = fin_mlist(globals, null);
        // end
    }
    // else a:=null;
    else {
        a = null;
    }
    // if m<0 then @<Finish math in text@>
    if m < 0 {
        crate::section_1196::Finish_math_in_text!(globals, p);
    }
    // else  begin if a=null then @<Check that another \.\$ follows@>;
    else {
        if a == null {
            crate::section_1197::Check_that_another_dollar_follows!(globals);
        }
        // @<Finish displayed math@>;
        crate::section_1199::Finish_displayed_math!(globals, p, a, l, danger);
        // end;
    }
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0016::decr;
use crate::section_0081::TeXResult;
use crate::section_0115::null;
use crate::section_0118::link;
use crate::section_0162::temp_head;
use crate::section_0213::mode;
use crate::section_0274::saved;
use crate::section_0281::unsave;
use crate::section_0644::natural0;
use crate::section_0644::natural1;
use crate::section_0649::hpack;
use crate::section_0688::style_node_subtype;
use crate::section_0726::mlist_to_hlist;
use crate::section_1184::fin_mlist;
