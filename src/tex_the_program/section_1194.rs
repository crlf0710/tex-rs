//! ` `
// @<Declare act...@>=
// procedure after_math;
#[allow(unused_variables, unused_assignments)]
pub(crate) fn after_math(globals: &mut TeXGlobals) -> TeXResult<()> {
    // var l:boolean; {`\.{\\leqno}' instead of `\.{\\eqno}'}
    /// `\leqno' instead of `\eqno`
    let l;
    // @!danger:boolean; {not enough symbol fonts are present}
    /// not enough symbol fonts are present
    let danger;
    // @!m:integer; {|mmode| or |-mmode|}
    /// `mmode` or `-mmode`
    let m;
    // @!p:pointer; {the formula}
    /// the formula
    let p;
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
        // cur_mlist:=p; cur_style:=text_style; mlist_penalties:=false;
        // mlist_to_hlist; a:=hpack(link(temp_head),natural);
        // unsave; decr(save_ptr); {now |cur_group=math_shift_group|}
        // if saved(0)=1 then l:=true;
        // danger:=false;
        // @<Check that the necessary fonts for math symbols are present;
        //   if not, flush the current math lists and set |danger:=true|@>;
        // m:=mode; p:=fin_mlist(null);
        // end
        todo!("end of equation number");
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
            todo!("Check that another `$` follows");
        }
        // @<Finish displayed math@>;
        todo!("Finish displayed math");
        // end;
    }
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0115::null;
use crate::section_0213::mode;
use crate::section_1184::fin_mlist;
