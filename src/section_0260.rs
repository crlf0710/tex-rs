// @ @<Insert a new control...@>=
// begin if text(p)>0 then
//   begin repeat if hash_is_full then overflow("hash size",hash_size);
// @:TeX capacity exceeded hash size}{\quad hash size@>
//   decr(hash_used);
//   until text(hash_used)=0; {search for an empty location in |hash|}
//   next(p):=hash_used; p:=hash_used;
//   end;
// str_room(l); d:=cur_length;
// while pool_ptr>str_start[str_ptr] do
//   begin decr(pool_ptr); str_pool[pool_ptr+l]:=str_pool[pool_ptr];
//   end; {move current string up to make room for another}
// for k:=j to j+l-1 do append_char(buffer[k]);
// text(p):=make_string; pool_ptr:=pool_ptr+d;
// @!stat incr(cs_count);@+tats@;@/
// end
//