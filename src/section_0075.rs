//! @ \TeX\ is careful not to call |error| when the print |selector| setting
//! might be unusual. The only possible values of |selector| at the time of
//! error messages are
//!
//! \yskip\hang|no_print| (when |interaction=batch_mode|
//!   and |log_file| not yet open);
//!
//! \hang|term_only| (when |interaction>batch_mode| and |log_file| not yet open);
//!
//! \hang|log_only| (when |interaction=batch_mode| and |log_file| is open);
//!
//! \hang|term_and_log| (when |interaction>batch_mode| and |log_file| is open).
//!
//! @<Initialize the print |selector| based on |interaction|@>=
//! if interaction=batch_mode then selector:=no_print@+else selector:=term_only
//!
