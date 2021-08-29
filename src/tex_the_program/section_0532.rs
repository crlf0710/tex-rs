//! @ Here's an example of how these conventions are used. Whenever it is time to
//! ship out a box of stuff, we shall use the macro |ensure_dvi_open|.
//
// @d ensure_dvi_open==if output_file_name=0 then
macro_rules! ensure_dvi_open {
    ($globals:expr) => {{
        // begin if job_name=0 then open_log_file;
        if $globals.job_name == 0 {
            open_log_file($globals);
        }
        // pack_job_name(".dvi");
        pack_job_name($globals, strpool_str!(".dvi"));
        // while not b_open_out(dvi_file) do
        while !b_open_out(
            make_globals_filename_view!($globals),
            &mut $globals.dvi_file,
        ) {
            // prompt_file_name("file name for output",".dvi");
            prompt_file_name(
                $globals,
                strpool_str!("file name for output"),
                strpool_str!(".dvi"),
            )?;
        }
        // output_file_name:=b_make_name_string(dvi_file);
        $globals.output_file_name = b_make_name_string(
            make_globals_io_string_view!($globals),
            &mut $globals.dvi_file,
        );
        // end
        use crate::section_0004::TeXGlobalsFilenameView;
        use crate::section_0004::TeXGlobalsIoStringView;
        use crate::section_0027::b_open_out;
        use crate::section_0525::b_make_name_string;
        use crate::section_0529::pack_job_name;
        use crate::section_0530::prompt_file_name;
        use crate::section_0534::open_log_file;
    }};
}

// @<Glob...@>=
// @!dvi_file: byte_file; {the device-independent output goes here}
/// the device-independent output goes here
#[globals_struct_field(TeXGlobals)]
pub(crate) static dvi_file: byte_file = byte_file::default();
// @!output_file_name: str_number; {full name of the output file}
/// full name of the output file
#[globals_struct_field(TeXGlobals)]
pub(crate) static output_file_name: str_number = str_number::default();
// @!log_name:str_number; {full name of the log file}
/// full name of the log file
#[globals_struct_field(TeXGlobals)]
pub(crate) static log_name: str_number = str_number::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0038::str_number;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
