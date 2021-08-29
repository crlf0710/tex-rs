//! @ The following system-independent code makes the |xord| array contain a
//! suitable inverse to the information in |xchr|. Note that if |xchr[i]=xchr[j]|
//! where |i<j<@'177|, the value of |xord[xchr[i]]| will turn out to be
//! |j| or more; hence, standard ASCII code numbers will be used instead of
//! codes below @'40 in case there is a coincidence.
//!
//! @<Set init...@>=
//! for i:=first_text_char to last_text_char do xord[chr(i)]:=invalid_code;
//! for i:=@'200 to @'377 do xord[xchr[i]]:=i;
//! for i:=0 to @'176 do xord[xchr[i]]:=i;
//!
