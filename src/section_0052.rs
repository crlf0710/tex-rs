// @ @<Read one string...@>=
// begin if eof(pool_file) then bad_pool('! TEX.POOL has no check sum.');
// @.TEX.POOL has no check sum@>
// read(pool_file,m,n); {read two digits of string length}
// if m='*' then @<Check the pool check sum@>
// else  begin if (xord[m]<"0")or(xord[m]>"9")or@|
//       (xord[n]<"0")or(xord[n]>"9") then
//     bad_pool('! TEX.POOL line doesn''t begin with two digits.');
// @.TEX.POOL line doesn't...@>
//   l:=xord[m]*10+xord[n]-"0"*11; {compute the length}
//   if pool_ptr+l+string_vacancies>pool_size then
//     bad_pool('! You have to increase POOLSIZE.');
// @.You have to increase POOLSIZE@>
//   for k:=1 to l do
//     begin if eoln(pool_file) then m:=' '@+else read(pool_file,m);
//     append_char(xord[m]);
//     end;
//   read_ln(pool_file); g:=make_string;
//   end;
// end
