//! @ And the following code uses the information after it has been gathered.

// @<Print two lines using the tricky pseudoprinted information@>=
macro_rules! Print_two_lines_using_the_tricky_pseudoprinted_information {
    ($globals:expr, $l:expr) => {{
        /// context information gathered for line 2
        let m: integer;
        /// length of line 1
        let n: u8_from_0_to_n<error_line_TYPENUM>;
        /// starting or ending place in `trick_buf`
        let mut p: integer;
        // if trick_count=1000000 then set_trick_count;
        //   {|set_trick_count| must be performed}
        if $globals.trick_count == 1000000 {
            /// `set_trick_count` must be performed
            set_trick_count!($globals);
        }
        // if tally<trick_count then m:=tally-first_count
        if $globals.tally < $globals.trick_count {
            m = $globals.tally - $globals.first_count;
        }
        // else m:=trick_count-first_count; {context on line 2}
        else {
            m = $globals.trick_count - $globals.first_count;
            /// context on line 2
            const _ : () = ();
        }
        // if l+first_count<=half_error_line then
        if $l.get() as integer + $globals.first_count <= half_error_line as _ {
            // begin p:=0; n:=l+first_count;
            p = 0;
            n = ($l.get() + $globals.first_count as u8).into();
            // end
        }
        // else  begin print("..."); p:=l+first_count-half_error_line+3;
        else {
            print($globals, strpool_str!("...").get() as _);
            p = $l.get() as integer + $globals.first_count - half_error_line as integer + 3;
            // n:=half_error_line;
            n = half_error_line.into();
            // end;
        }
        // for q:=p to first_count-1 do print_char(trick_buf[q mod error_line]);
        for q in p ..= ($globals.first_count - 1) {
            let ch = $globals.trick_buf[(q % error_line as integer) as u8];
            print_char(make_globals_io_string_log_view!($globals), ch);
        }
        // print_ln;
        print_ln(make_globals_io_view!($globals));
        // for q:=1 to n do print_char(" "); {print |n| spaces to begin line~2}
        /// print `n` spaces to begin line 2
        for _ in $globals.first_count ..= (p - 1) {
            let ch = ASCII_code_literal!(b' ');
            print_char(make_globals_io_string_log_view!($globals), ch);
        }
        // if m+n<=error_line then p:=first_count+m else p:=first_count+(error_line-n-3);
        if m + n.get() as integer <= error_line as _ {
            p = $globals.first_count + m;
        } else {
            p = $globals.first_count + (error_line - n.get() - 3) as integer;
        }
        // for q:=first_count to p-1 do print_char(trick_buf[q mod error_line]);
        for q in $globals.first_count ..= (p - 1) {
            let ch = $globals.trick_buf[(q % error_line as integer) as u8];
            print_char(make_globals_io_string_log_view!($globals), ch);
        }
        // if m+n>error_line then print("...")
        if m + n.get() as integer > error_line as _ {
            print($globals, strpool_str!("...").get() as _);
        }
        use crate::pascal::u8_from_0_to_n;
        use crate::section_0004::TeXGlobalsIoView;
        use crate::section_0004::TeXGlobalsIoStringLogView;
        use crate::section_0011::half_error_line;
        use crate::section_0011::error_line_TYPENUM;
        use crate::section_0011::error_line;
        use crate::section_0057::print_ln;
        use crate::section_0058::print_char;
        use crate::section_0059::print;
    }}
}
