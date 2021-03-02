//! @ @<Undump the hash table@>=
//! undump(hash_base)(frozen_control_sequence)(hash_used); p:=hash_base-1;
//! repeat undump(p+1)(hash_used)(p); undump_hh(hash[p]);
//! until p=hash_used;
//! for p:=hash_used+1 to undefined_control_sequence-1 do undump_hh(hash[p]);
//! undump_int(cs_count)
//!
