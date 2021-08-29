//! ` `
// @<Declare act...@>=
// procedure after_math;
pub(crate) fn after_math(_globals: &mut TeXGlobals) {
    // var l:boolean; {`\.{\\leqno}' instead of `\.{\\eqno}'}
    // @!danger:boolean; {not enough symbol fonts are present}
    // @!m:integer; {|mmode| or |-mmode|}
    // @!p:pointer; {the formula}
    // @!a:pointer; {box containing equation number}
    // @<Local variables for finishing a displayed formula@>@;
    // begin danger:=false;
    // @<Check that the necessary fonts for math symbols are present;
    //   if not, flush the current math lists and set |danger:=true|@>;
    // m:=mode; l:=false; p:=fin_mlist(null); {this pops the nest}
    // if mode=-m then {end of equation number}
    //   begin @<Check that another \.\$ follows@>;
    //   cur_mlist:=p; cur_style:=text_style; mlist_penalties:=false;
    //   mlist_to_hlist; a:=hpack(link(temp_head),natural);
    //   unsave; decr(save_ptr); {now |cur_group=math_shift_group|}
    //   if saved(0)=1 then l:=true;
    //   danger:=false;
    //   @<Check that the necessary fonts for math symbols are present;
    //     if not, flush the current math lists and set |danger:=true|@>;
    //   m:=mode; p:=fin_mlist(null);
    //   end
    // else a:=null;
    // if m<0 then @<Finish math in text@>
    // else  begin if a=null then @<Check that another \.\$ follows@>;
    //   @<Finish displayed math@>;
    //   end;
    // end;
    panic!("after_math");
}

use crate::section_0004::TeXGlobals;
