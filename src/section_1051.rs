//! @ Some operations are allowed only in privileged modes, i.e., in cases
//! that |mode>0|. The |privileged| function is used to detect violations
//! of this rule; it issues an error message and returns |false| if the
//! current |mode| is negative.
//!
//! @<Declare act...@>=
//! function privileged:boolean;
//! begin if mode>0 then privileged:=true
//! else  begin report_illegal_case; privileged:=false;
//!   end;
//! end;
//!
