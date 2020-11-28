//! @ Here it is necessary to explain a little trick. We don't want to store a long
//! string that corresponds to a token list, because that string might take up
//! lots of memory; and we are printing during a time when an error message is
//! being given, so we dare not do anything that might overflow one of \TeX's
//! tables. So `pseudoprinting' is the answer: We enter a mode of printing
//! that stores characters into a buffer of length |error_line|, where character
//! $k+1$ is placed into \hbox{|trick_buf[k mod error_line]|} if
//! |k<trick_count|, otherwise character |k| is dropped. Initially we set
//! |tally:=0| and |trick_count:=1000000|; then when we reach the
//! point where transition from line 1 to line 2 should occur, we
//! set |first_count:=tally| and |trick_count:=@tmax@>(error_line,
//! tally+1+error_line-half_error_line)|. At the end of the
//! pseudoprinting, the values of |first_count|, |tally|, and
//! |trick_count| give us all the information we need to print the two lines,
//! and all of the necessary text is in |trick_buf|.
//!
//! Namely, let |l| be the length of the descriptive information that appears
//! on the first line. The length of the context information gathered for that
//! line is |k=first_count|, and the length of the context information
//! gathered for line~2 is $m=\min(|tally|, |trick_count|)-k$. If |l+k<=h|,
//! where |h=half_error_line|, we print |trick_buf[0..k-1]| after the
//! descriptive information on line~1, and set |n:=l+k|; here |n| is the
//! length of line~1. If $l+k>h$, some cropping is necessary, so we set |n:=h|
//! and print `\.{...}' followed by
//! $$\hbox{|trick_buf[(l+k-h+3)..k-1]|,}$$
//! where subscripts of |trick_buf| are circular modulo |error_line|. The
//! second line consists of |n|~spaces followed by |trick_buf[k..(k+m-1)]|,
//! unless |n+m>error_line|; in the latter case, further cropping is done.
//! This is easier to program than to explain.
//!
//! @<Local variables for formatting...@>=
//! @!i:0..buf_size; {index into |buffer|}
//! @!j:0..buf_size; {end of current line in |buffer|}
//! @!l:0..half_error_line; {length of descriptive information on line 1}
//! @!m:integer; {context information gathered for line 2}
//! @!n:0..error_line; {length of line 1}
//! @!p: integer; {starting or ending place in |trick_buf|}
//! @!q: integer; {temporary index}
//!
