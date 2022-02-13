//! @ Here's a procedure that changes the |if_limit| code corresponding to
//! a given value of |cond_ptr|.
//
// @p procedure change_if_limit(@!l:small_number;@!p:pointer);
pub(crate) fn change_if_limit(globals: &mut TeXGlobals, l: small_number, p: pointer) {
    // label exit;
    // var q:pointer;
    // begin if p=cond_ptr then if_limit:=l {that's the easy case}
    if p == globals.cond_ptr {
        /// that's the easy case
        {
            globals.if_limit = l.get().into();
        }
    }
    // else  begin q:=cond_ptr;
    else {
        todo!("not yet implemented in {}", file!());
        //   loop@+  begin if q=null then confusion("if");
        // @:this can't happen if}{\quad if@>
        //     if link(q)=p then
        //       begin type(q):=l; return;
        //       end;
        //     q:=link(q);
        //     end;
        //   end;
    }
    // exit:end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0101::small_number;
use crate::section_0115::pointer;
