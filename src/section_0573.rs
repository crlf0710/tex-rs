//! ` `
// @d check_existence(#)==@t@>@;@/
//   begin check_byte_range(#);
//   qw:=char_info(f)(#); {N.B.: not |qi(#)|}
//   if not char_exists(qw) then abort;
//   end
//
// @<Read ligature/kern program@>=
// bch_label:=@'77777; bchar:=256;
// if nl>0 then
//   begin for k:=lig_kern_base[f] to kern_base[f]+kern_base_offset-1 do
//     begin store_four_quarters(font_info[k].qqqq);
//     if a>128 then
//       begin if 256*c+d>=nl then abort;
//       if a=255 then if k=lig_kern_base[f] then bchar:=b;
//       end
//     else begin if b<>bchar then check_existence(b);
//       if c<128 then check_existence(d) {check ligature}
//       else if 256*(c-128)+d>=nk then abort; {check kern}
//       if a<128 then if k-lig_kern_base[f]+a+1>=nl then abort;
//       end;
//     end;
//   if a=255 then bch_label:=256*c+d;
//   end;
// for k:=kern_base[f]+kern_base_offset to exten_base[f]-1 do
//   store_scaled(font_info[k].sc);
//

