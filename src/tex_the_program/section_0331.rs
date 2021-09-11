//! @ To get \TeX's whole input mechanism going, we perform the following
//! actions.
//
// @<Initialize the input routines@>=
pub(crate) macro Initialize_the_input_routines($globals:expr, $lbl_final_end:lifetime) {
    crate::trace_span!("Initialize_the_input_routines");
    // begin input_ptr:=0; max_in_stack:=0;
    $globals.input_ptr = 0.into();
    $globals.max_in_stack = 0.into();
    // in_open:=0; open_parens:=0; max_buf_stack:=0;
    $globals.in_open = 0.into();
    $globals.open_parens = 0.into();
    $globals.max_buf_stack = 0.into();
    // param_ptr:=0; max_param_stack:=0;
    $globals.param_ptr = 0.into();
    $globals.max_param_stack = 0.into();
    // first:=buf_size; repeat buffer[first]:=0; decr(first); until first=0;
    $globals.first = buf_size.into();
    loop {
        $globals.buffer[$globals.first] = 0.into();
        decr!($globals.first);
        if $globals.first == 0 {
            break;
        }
    }
    // scanner_status:=normal; warning_index:=null; first:=1;
    $globals.scanner_status = scanner_status_kind::normal;
    $globals.warning_index = null;
    $globals.first = 1.into();
    // state:=new_line; start:=1; index:=0; line:=0; name:=0;
    state!($globals) = new_line;
    start!($globals) = 1;
    index!($globals) = 0;
    $globals.line = 0;
    name!($globals) = 0;
    // force_eof:=false;
    $globals.force_eof = false;
    // align_state:=1000000;@/
    $globals.align_state = 1000000;
    // if not init_terminal then goto final_end;
    if !init_terminal($globals) {
        crate::goto_forward_label!($lbl_final_end);
    }
    // limit:=last; first:=last+1; {|init_terminal| has set |loc| and |last|}
    /// `init_terminal` has set `loc` and `last`
    const _: () = ();
    limit!($globals) = $globals.last.get();
    $globals.first = $globals.last + 1;
    // end
    use crate::section_0011::buf_size;
    use crate::section_0016::decr;
    use crate::section_0037::init_terminal;
    use crate::section_0115::null;
    use crate::section_0302::index;
    use crate::section_0302::limit;
    use crate::section_0302::name;
    use crate::section_0302::start;
    use crate::section_0302::state;
    use crate::section_0303::new_line;
    use crate::section_0305::scanner_status_kind;
}
