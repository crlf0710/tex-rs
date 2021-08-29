//! @ Initially |job_name=0|; it becomes nonzero as soon as the true name is known.
//! We have |job_name=0| if and only if the `\.{log}' file has not been opened,
//! except of course for a short time just after |job_name| has become nonzero.
//
// @<Initialize the output...@>=
// job_name:=0; name_in_progress:=false; log_opened:=false;
//
