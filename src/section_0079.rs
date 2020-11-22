//! @ Individual lines of help are recorded in the array |help_line|, which
//! contains entries in positions |0..(help_ptr-1)|. They should be printed
//! in reverse order, i.e., with |help_line[0]| appearing last.
//
// @d hlp1(#)==help_line[0]:=#;@+end
// @d hlp2(#)==help_line[1]:=#; hlp1
// @d hlp3(#)==help_line[2]:=#; hlp2
// @d hlp4(#)==help_line[3]:=#; hlp3
// @d hlp5(#)==help_line[4]:=#; hlp4
// @d hlp6(#)==help_line[5]:=#; hlp5
// @d help0==help_ptr:=0 {sometimes there might be no help}
// @d help1==@+begin help_ptr:=1; hlp1 {use this with one help line}
// @d help2==@+begin help_ptr:=2; hlp2 {use this with two help lines}
// @d help3==@+begin help_ptr:=3; hlp3 {use this with three help lines}
// @d help4==@+begin help_ptr:=4; hlp4 {use this with four help lines}
// @d help5==@+begin help_ptr:=5; hlp5 {use this with five help lines}
// @d help6==@+begin help_ptr:=6; hlp6 {use this with six help lines}
//
// @<Glob...@>=
// @!help_line:array[0..5] of str_number; {helps for the next |error|}
// @!help_ptr:0..6; {the number of help lines present}
// @!use_err_help:boolean; {should the |err_help| list be shown?}
//
