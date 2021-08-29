//! @ Here is a function that returns a pointer to a character node for a
//! given character in a given font. If that character doesn't exist,
//! |null| is returned instead.
//
// @p function new_character(@!f:internal_font_number;@!c:eight_bits):pointer;
// label exit;
// var p:pointer; {newly allocated node}
// begin if font_bc[f]<=c then if font_ec[f]>=c then
//   if char_exists(char_info(f)(qi(c))) then
//     begin p:=get_avail; font(p):=f; character(p):=qi(c);
//     new_character:=p; return;
//     end;
// char_warning(f,c);
// new_character:=null;
// exit:end;
//
