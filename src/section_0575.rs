//! @ We check to see that the \.{TFM} file doesn't end prematurely; but
//! no error message is given for files having more than |lf| words.
//
// @<Read font parameters@>=
// begin for k:=1 to np do
//   if k=1 then {the |slant| parameter is a pure number}
//     begin fget; sw:=fbyte; if sw>127 then sw:=sw-256;
//     fget; sw:=sw*@'400+fbyte; fget; sw:=sw*@'400+fbyte;
//     fget; font_info[param_base[f]].sc:=
//       (sw*@'20)+(fbyte div@'20);
//     end
//   else store_scaled(font_info[param_base[f]+k-1].sc);
// if eof(tfm_file) then abort;
// for k:=np+1 to 7 do font_info[param_base[f]+k-1].sc:=0;
// end
//
