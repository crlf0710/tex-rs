//! @ As an introduction to these routines, let's consider one of the simplest
//! cases: What happens when `\.{\\hrule}' occurs in vertical mode, or
//! `\.{\\vrule}' in horizontal mode or math mode? The code in |main_control|
//! is short, since the |scan_rule_spec| routine already does most of what is
//! required; thus, there is no need for a special action procedure.
//!
//! Note that baselineskip calculations are disabled after a rule in vertical
//! mode, by setting |prev_depth:=ignore_depth|.
//!
//! @<Cases of |main_control| that build...@>=
//! vmode+hrule,hmode+vrule,mmode+vrule: begin tail_append(scan_rule_spec);
//!   if abs(mode)=vmode then prev_depth:=ignore_depth
//!   else if abs(mode)=hmode then space_factor:=1000;
//!   end;
//!
