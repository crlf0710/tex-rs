//! @ In anomalous cases, the print selector might be in an unknown state;
//! the following subroutine is called to fix things just enough to keep
//! running a bit longer.
//!
//! @p procedure normalize_selector;
//! begin if log_opened then selector:=term_and_log
//! else selector:=term_only;
//! if job_name=0 then open_log_file;
//! if interaction=batch_mode then decr(selector);
//! end;
//!
