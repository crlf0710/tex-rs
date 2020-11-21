//! ` `
// @<Declare act...@>=
// procedure app_space; {handle spaces when |space_factor<>1000|}
/// handle spaces when `space_factor<>1000`
#[allow(unused_variables)]
pub(crate) fn app_space(globals: &mut TeXGlobals) {
    todo!();
    // var@!q:pointer; {glue node}
    // begin if (space_factor>=2000)and(xspace_skip<>zero_glue) then
    //   q:=new_param_glue(xspace_skip_code)
    // else  begin if space_skip<>zero_glue then main_p:=space_skip
    //   else @<Find the glue specification...@>;
    //   main_p:=new_spec(main_p);
    //   @<Modify the glue specification in |main_p| according to the space factor@>;
    //   q:=new_glue(main_p); glue_ref_count(main_p):=null;
    //   end;
    // link(tail):=q; tail:=q;
    // end;
}

use crate::section_0004::TeXGlobals;
