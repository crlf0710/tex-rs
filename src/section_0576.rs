//! @ Now to wrap it up, we have checked all the necessary things about the \.{TFM}
//! file, and all we need to do is put the finishing touches on the data for
//! the new font.
//
// @d adjust(#)==#[f]:=qo(#[f])
//   {correct for the excess |min_quarterword| that was added}
//
// @<Make final adjustments...@>=
// if np>=7 then font_params[f]:=np@+else font_params[f]:=7;
// hyphen_char[f]:=default_hyphen_char; skew_char[f]:=default_skew_char;
// if bch_label<nl then bchar_label[f]:=bch_label+lig_kern_base[f]
// else bchar_label[f]:=non_address;
// font_bchar[f]:=qi(bchar);
// font_false_bchar[f]:=qi(bchar);
// if bchar<=ec then if bchar>=bc then
//   begin qw:=char_info(f)(bchar); {N.B.: not |qi(bchar)|}
//   if char_exists(qw) then font_false_bchar[f]:=non_char;
//   end;
// font_name[f]:=nom;
// font_area[f]:=aire;
// font_bc[f]:=bc; font_ec[f]:=ec; font_glue[f]:=null;
// adjust(char_base); adjust(width_base); adjust(lig_kern_base);
// adjust(kern_base); adjust(exten_base);
// decr(param_base[f]);
// fmem_ptr:=fmem_ptr+lf; font_ptr:=f; g:=f; goto done
//
