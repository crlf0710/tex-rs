//! ` `
// @<Declare act...@>=
// procedure app_space; {handle spaces when |space_factor<>1000|}
/// handle spaces when `space_factor<>1000`
#[allow(unused_variables)]
pub(crate) fn app_space(globals: &mut TeXGlobals) -> TeXResult<()> {
    // var@!q:pointer; {glue node}
    /// glue node
    let q: pointer;
    // begin if (space_factor>=2000)and(xspace_skip<>zero_glue) then
    if space_factor!(globals) >= 2000 && xspace_skip!(globals) != zero_glue {
        // q:=new_param_glue(xspace_skip_code)
        q = new_param_glue(globals, xspace_skip_code.into())?;
    }
    // else  begin if space_skip<>zero_glue then main_p:=space_skip
    else {
        if space_skip!(globals) != zero_glue {
            globals.main_p = space_skip!(globals);
        }
        // else @<Find the glue specification...@>;
        else {
            crate::section_1042::Find_the_glue_specification_main_p_for_text_spaces_in_the_current_font!(
                globals
            );
        }
        // main_p:=new_spec(main_p);
        globals.main_p = new_spec(globals, globals.main_p)?;
        // @<Modify the glue specification in |main_p| according to the space factor@>;
        crate::section_1044::Modify_the_glue_specification_in_main_p_according_to_the_space_factor!(
            globals
        );
        // q:=new_glue(main_p); glue_ref_count(main_p):=null;
        q = new_glue(globals, globals.main_p)?;
        glue_ref_count!(globals, globals.main_p) = null;
        // end;
    }
    // link(tail):=q; tail:=q;
    link!(globals, tail!(globals)) = q;
    tail!(globals) = q;
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0115::null;
use crate::section_0115::pointer;
use crate::section_0118::link;
use crate::section_0150::glue_ref_count;
use crate::section_0151::new_spec;
use crate::section_0152::new_param_glue;
use crate::section_0153::new_glue;
use crate::section_0162::zero_glue;
use crate::section_0213::space_factor;
use crate::section_0213::tail;
use crate::section_0224::space_skip;
use crate::section_0224::xspace_skip;
use crate::section_0224::xspace_skip_code;
