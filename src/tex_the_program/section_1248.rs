//! ` `
//!
//! Paragraph shapes are set up in the obvious way.
//
// @<Assignments@>=
pub(crate) macro Assignments_1248($globals:expr, $cur_cmd:expr, $a:expr) {{
    // set_shape: begin scan_optional_equals; scan_int; n:=cur_val;
    let processed = if $cur_cmd == set_shape {
        // if n<=0 then p:=null
        // else  begin p:=get_node(2*n+1); info(p):=n;
        //   for j:=1 to n do
        //     begin scan_normal_dimen;
        //     mem[p+2*j-1].sc:=cur_val; {indentation}
        //     scan_normal_dimen;
        //     mem[p+2*j].sc:=cur_val; {width}
        //     end;
        //   end;
        // define(par_shape_loc,shape_ref,p);
        // end;
        todo!("set shape");
    } else {
        false
    };
    use crate::section_0209::*;
    processed
}}
