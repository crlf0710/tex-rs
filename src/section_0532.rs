//! @ Here's an example of how these conventions are used. Whenever it is time to
//! ship out a box of stuff, we shall use the macro |ensure_dvi_open|.
//!
//! @d ensure_dvi_open==if output_file_name=0 then
//!   begin if job_name=0 then open_log_file;
//!   pack_job_name(".dvi");
//!   while not b_open_out(dvi_file) do
//!     prompt_file_name("file name for output",".dvi");
//!   output_file_name:=b_make_name_string(dvi_file);
//!   end
//!
//! @<Glob...@>=
//! @!dvi_file: byte_file; {the device-independent output goes here}
//! @!output_file_name: str_number; {full name of the output file}
//! @!log_name:str_number; {full name of the log file}
//!
