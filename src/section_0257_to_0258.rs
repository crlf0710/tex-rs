//! @ @<Set init...@>=
//! no_new_control_sequence:=true; {new identifiers are usually forbidden}
//! next(hash_base):=0; text(hash_base):=0;
//! for k:=hash_base+1 to undefined_control_sequence-1 do hash[k]:=hash[hash_base];
//!
//! @ @<Initialize table entries...@>=
//! hash_used:=frozen_control_sequence; {nothing is used}
//! cs_count:=0;
//! eq_type(frozen_dont_expand):=dont_expand;
//! text(frozen_dont_expand):="notexpanded:";
//! @.notexpanded:@>
//!
