//! @ The \TeX\ processor converts between ASCII code and
//! the user's external character set by means of arrays |xord| and |xchr|
//! that are analogous to \PASCAL's |ord| and |chr| functions.
//!
//! @<Glob...@>=
//! @!xord: array [text_char] of ASCII_code;
//!   {specifies conversion of input characters}
//! @!xchr: array [ASCII_code] of text_char;
//!   {specifies conversion of output characters}
//!
//! @ Since we are assuming that our \PASCAL\ system is able to read and
//! write the visible characters of standard ASCII (although not
//! necessarily using the ASCII codes to represent them), the following
//! assignment statements initialize the standard part of the |xchr| array
//! properly, without needing any system-dependent changes. On the other
//! hand, it is possible to implement \TeX\ with less complete character
//! sets, and in such cases it will be necessary to change something here.
//! @^system dependencies@>
//!
//! @<Set init...@>=
//! xchr[@'40]:=' ';
//! xchr[@'41]:='!';
//! xchr[@'42]:='"';
//! xchr[@'43]:='#';
//! xchr[@'44]:='$';
//! xchr[@'45]:='%';
//! xchr[@'46]:='&';
//! xchr[@'47]:='''';@/
//! xchr[@'50]:='(';
//! xchr[@'51]:=')';
//! xchr[@'52]:='*';
//! xchr[@'53]:='+';
//! xchr[@'54]:=',';
//! xchr[@'55]:='-';
//! xchr[@'56]:='.';
//! xchr[@'57]:='/';@/
//! xchr[@'60]:='0';
//! xchr[@'61]:='1';
//! xchr[@'62]:='2';
//! xchr[@'63]:='3';
//! xchr[@'64]:='4';
//! xchr[@'65]:='5';
//! xchr[@'66]:='6';
//! xchr[@'67]:='7';@/
//! xchr[@'70]:='8';
//! xchr[@'71]:='9';
//! xchr[@'72]:=':';
//! xchr[@'73]:=';';
//! xchr[@'74]:='<';
//! xchr[@'75]:='=';
//! xchr[@'76]:='>';
//! xchr[@'77]:='?';@/
//! xchr[@'100]:='@@';
//! xchr[@'101]:='A';
//! xchr[@'102]:='B';
//! xchr[@'103]:='C';
//! xchr[@'104]:='D';
//! xchr[@'105]:='E';
//! xchr[@'106]:='F';
//! xchr[@'107]:='G';@/
//! xchr[@'110]:='H';
//! xchr[@'111]:='I';
//! xchr[@'112]:='J';
//! xchr[@'113]:='K';
//! xchr[@'114]:='L';
//! xchr[@'115]:='M';
//! xchr[@'116]:='N';
//! xchr[@'117]:='O';@/
//! xchr[@'120]:='P';
//! xchr[@'121]:='Q';
//! xchr[@'122]:='R';
//! xchr[@'123]:='S';
//! xchr[@'124]:='T';
//! xchr[@'125]:='U';
//! xchr[@'126]:='V';
//! xchr[@'127]:='W';@/
//! xchr[@'130]:='X';
//! xchr[@'131]:='Y';
//! xchr[@'132]:='Z';
//! xchr[@'133]:='[';
//! xchr[@'134]:='\';
//! xchr[@'135]:=']';
//! xchr[@'136]:='^';
//! xchr[@'137]:='_';@/
//! xchr[@'140]:='`';
//! xchr[@'141]:='a';
//! xchr[@'142]:='b';
//! xchr[@'143]:='c';
//! xchr[@'144]:='d';
//! xchr[@'145]:='e';
//! xchr[@'146]:='f';
//! xchr[@'147]:='g';@/
//! xchr[@'150]:='h';
//! xchr[@'151]:='i';
//! xchr[@'152]:='j';
//! xchr[@'153]:='k';
//! xchr[@'154]:='l';
//! xchr[@'155]:='m';
//! xchr[@'156]:='n';
//! xchr[@'157]:='o';@/
//! xchr[@'160]:='p';
//! xchr[@'161]:='q';
//! xchr[@'162]:='r';
//! xchr[@'163]:='s';
//! xchr[@'164]:='t';
//! xchr[@'165]:='u';
//! xchr[@'166]:='v';
//! xchr[@'167]:='w';@/
//! xchr[@'170]:='x';
//! xchr[@'171]:='y';
//! xchr[@'172]:='z';
//! xchr[@'173]:='{';
//! xchr[@'174]:='|';
//! xchr[@'175]:='}';
//! xchr[@'176]:='~';@/
//!
//! @ Some of the ASCII codes without visible characters have been given symbolic
//! names in this program because they are used with a special meaning.
//!
//! @d null_code=@'0 {ASCII code that might disappear}
//! @d carriage_return=@'15 {ASCII code used at end of line}
//! @d invalid_code=@'177 {ASCII code that many systems prohibit in text files}
//!
//! @ The ASCII code is ``standard'' only to a certain extent, since many
//! computer installations have found it advantageous to have ready access
//! to more than 94 printing characters. Appendix~C of {\sl The \TeX book\/}
//! gives a complete specification of the intended correspondence between
//! characters and \TeX's internal representation.
//! @:TeXbook}{\sl The \TeX book@>
//!
//! If \TeX\ is being used
//! on a garden-variety \PASCAL\ for which only standard ASCII
//! codes will appear in the input and output files, it doesn't really matter
//! what codes are specified in |xchr[0..@'37]|, but the safest policy is to
//! blank everything out by using the code shown below.
//!
//! However, other settings of |xchr| will make \TeX\ more friendly on
//! computers that have an extended character set, so that users can type things
//! like `\.^^Z' instead of `\.{\\ne}'. People with extended character sets can
//! assign codes arbitrarily, giving an |xchr| equivalent to whatever
//! characters the users of \TeX\ are allowed to have in their input files.
//! It is best to make the codes correspond to the intended interpretations as
//! shown in Appendix~C whenever possible; but this is not necessary. For
//! example, in countries with an alphabet of more than 26 letters, it is
//! usually best to map the additional letters into codes less than~@'40.
//! To get the most ``permissive'' character set, change |' '| on the
//! right of these assignment statements to |chr(i)|.
//! @^character set dependencies@>
//! @^system dependencies@>
//!
//! @<Set init...@>=
//! for i:=0 to @'37 do xchr[i]:=' ';
//! for i:=@'177 to @'377 do xchr[i]:=' ';
//!
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
