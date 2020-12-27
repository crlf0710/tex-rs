//! @ Now this is really it: \TeX\ starts and ends here.
//!
//! The initial test involving |ready_already| should be deleted if the
//! \PASCAL\ runtime system is smart enough to detect such a ``mistake.''
//! @^system dependencies@>

#[distributed_slice]
pub(crate) static CHECK_THE_CONSTANT_VALUES_FOR_CONSISTENCY: [fn(&mut TeXGlobals)] = [..];

macro_rules! Check_the_constant_values_for_consistency {
    ($globals:expr) => {
        for f in CHECK_THE_CONSTANT_VALUES_FOR_CONSISTENCY {
            f($globals);
        }
    };
}

macro_rules! Initialize_the_output_routines {
    ($globals:expr) => {
        Initialize_the_output_routines_0055!($globals);
        Initialize_the_output_routines_0061!($globals);
        //Initialize_the_output_routines_0528!($globals);
        //Initialize_the_output_routines_053?!($globals);
    };
}


/// Main entry to TeX
#[allow(unused_mut, unused_variables)]
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace", skip(globals)))]
pub fn entry(globals: &mut TeXGlobals) {
    workarounds!();
    

    // @p begin @!{|start_here|}

    /// start_here
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
    Check_the_constant_values_for_consistency!(globals);
    // if bad>0 then
    if globals.bad > 0 {
        // begin wterm_ln('Ouch---my internal constants have been clobbered!',
        //   '---case ',bad:1);
        // @.Ouch...clobbered@>
        wterm_ln(make_globals_io_view!(globals), format!("{}{}{:1}","Ouch---my internal constants have been clobbered!",
            "---case ", globals.bad));
        //   goto final_end;
        goto_forward_label!('final_end);
        // end;
    }
    // initialize; {set global variables to their starting values}
    /// set global variables to their starting values
    initialize(globals);
    // @!init if not get_strings_started then goto final_end;
    region_initex! {
        if !get_strings_started(globals) {
            goto_forward_label!('final_end);
        }
        // init_prim; {call |primitive| for each primitive}
        /// call `primitive` for each primitive
        init_prim(globals);
        // init_str_ptr:=str_ptr; init_pool_ptr:=pool_ptr; fix_date_and_time;
        globals.init_str_ptr = globals.str_ptr;
        globals.init_pool_ptr = globals.pool_ptr;
        fix_date_and_time(globals);
        // tini@/
    };
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
    Get_the_first_line_of_input_and_prepare_to_start!(globals, 'end_of_TEX, 'final_end);
    // history:=spotless; {ready to go!}
    /// ready to go!
    {
        globals.history = spotless;
    }

    // main_control; {come to life}
    /// come to life
    try_or_jump!(main_control(globals), 'end_of_TEX);
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

use crate::section_0004::initialize;
use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoView;
use crate::section_0033::t_open_out;
use crate::section_0047::get_strings_started;
use crate::section_0076::history_kind::{fatal_error_stop, spotless};
use crate::section_0241::fix_date_and_time;
use crate::section_1030::main_control;
use crate::section_1333::close_files_and_terminate;
use crate::section_1335::final_cleanup;
use crate::section_1336::init_prim;
use linkme::distributed_slice;
