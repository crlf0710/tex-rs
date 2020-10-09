//! @ Now this is really it: \TeX\ starts and ends here.
//!
//! The initial test involving |ready_already| should be deleted if the
//! \PASCAL\ runtime system is smart enough to detect such a ``mistake.''
//! @^system dependencies@>

/// Main entry to TeX
#[allow(unused_mut, unused_variables)]
pub fn entry() {
    // @p begin @!{|start_here|}

    /// start_here
    let mut globals = TeXGlobals::default();
    let globals = &mut globals;

    region_forward_label! {|'final_end|{
    region_forward_label! {|'end_of_TEX|{
    region_forward_label! {|'start_of_TEX|{

    // history:=fatal_error_stop; {in case we quit during initialization}
    /// in case we quit during initialization
    {
        globals.history = fatal_error_stop;
    }

    // t_open_out; {open the terminal for output}
    /// open the terminal for output
    t_open_out(globals);

    // if ready_already=314159 then goto start_of_TEX;
    if globals.ready_already == 314159 {
        goto_forward_label!('start_of_TEX);
    }
    // @<Check the ``constant'' values...@>@;
    // if bad>0 then
    //   begin wterm_ln('Ouch---my internal constants have been clobbered!',
    //     '---case ',bad:1);
    // @.Ouch...clobbered@>
    //   goto final_end;
    //   end;
    // initialize; {set global variables to their starting values}
    /// set global variables to their starting values
    initialize(globals);
    // @!init if not get_strings_started then goto final_end;
    // init_prim; {call |primitive| for each primitive}
    // init_str_ptr:=str_ptr; init_pool_ptr:=pool_ptr; fix_date_and_time;
    // tini@/
    /// ready_already:=314159;
    {
        globals.ready_already = 314159;
    }

    }
    // start_of_TEX: @<Initialize the output routines@>;
    'start_of_TEX <-
    };
    Initialize_the_output_routines!(globals);
    // @<Get the first line of input and prepare to start@>;
    Get_the_first_line_of_input_and_prepare_to_start!(globals);
    // history:=spotless; {ready to go!}
    /// ready to go!
    {
        globals.history = spotless;
    }

    // main_control; {come to life}
    /// come to life
    main_control(globals);
    // final_cleanup; {prepare for death}
    /// prepare for death
    final_cleanup(globals);
    }
    // end_of_TEX: close_files_and_terminate;
    'end_of_TEX <-
    };
    close_files_and_terminate(globals);
    }
    // final_end: ready_already:=0;
    'final_end <-
    };
    globals.ready_already = 0;
    // end.
}

use crate::section_0004::TeXGlobals;
use crate::section_0004::initialize;
use crate::section_0033::t_open_out;
use crate::section_0076::history_kind::{fatal_error_stop, spotless};
use crate::section_1030::main_control;
use crate::section_1333::close_files_and_terminate;
use crate::section_1335::final_cleanup;
