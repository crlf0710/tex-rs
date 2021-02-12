//! @ Still another subroutine is needed: This one is sort of a combination
//! of |new_param_glue| and |new_glue|. It creates a glue node for one of
//! the current glue parameters, but it makes a fresh copy of the glue
//! specification, since that specification will probably be subject to change,
//! while the parameter will stay put. The global variable |temp_ptr| is
//! set to the address of the new spec.
//
// @p function new_skip_param(@!n:small_number):pointer;
pub(crate) fn new_skip_param(globals: &mut TeXGlobals, n: small_number) -> TeXResult<pointer> {
    // var p:pointer; {the new node}
    /// the new node
    let p: pointer;
    // begin temp_ptr:=new_spec(@<Current |mem| equivalent of glue parameter...@>);
    globals.temp_ptr = new_spec(
        globals,
        Current_mem_equivalent_of_glue_parameter_number_n!(globals, n.get()),
    )?;
    // p:=new_glue(temp_ptr); glue_ref_count(temp_ptr):=null; subtype(p):=n+1;
    p = new_glue(globals, globals.temp_ptr)?;
    glue_ref_count!(globals, globals.temp_ptr) = null;
    subtype!(globals, p) = n.get() + 1;
    // new_skip_param:=p;
    ok_nojump!(p)
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::small_number;
use crate::section_0115::null;
use crate::section_0115::pointer;
use crate::section_0151::new_spec;
use crate::section_0153::new_glue;
