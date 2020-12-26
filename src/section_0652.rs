//! @ @<Make node |p| look like a |char_node| and |goto reswitch|@>=
//! begin mem[lig_trick]:=mem[lig_char(p)]; link(lig_trick):=link(p);
//! p:=lig_trick; goto reswitch;
//! end
//!
