// @ @<Read one string...@>=
pub(crate) macro Read_one_string_but_return_false_if_the_string_memory_space_is_getting_too_tight_for_comfort($globals:expr, $c:expr, $g:expr) {
    crate::trace_span_verbose!("Read one string...");
    // begin if eof(pool_file) then bad_pool('! TEX.POOL has no check sum.');
    // @.TEX.POOL has no check sum@>
    // read(pool_file,m,n); {read two digits of string length}
    let mut m: text_char = read_onearg(&mut $globals.pool_file);
    let n: text_char = read_onearg(&mut $globals.pool_file);
    // if m='*' then @<Check the pool check sum@>
    if xord(m) == ASCII_code_literal!(b'*') {
        crate::section_0053::Check_the_pool_check_sum!($globals, $c);
    }
    // else  begin if (xord[m]<"0")or(xord[m]>"9")or@|
    //       (xord[n]<"0")or(xord[n]>"9") then
    else {
        if xord(m) < ASCII_code_literal!(b'0')
            || xord(m) > ASCII_code_literal!(b'9')
            || xord(n) < ASCII_code_literal!(b'0')
            || xord(n) > ASCII_code_literal!(b'9')
        {
            //     bad_pool('! TEX.POOL line doesn''t begin with two digits.');
            // @.TEX.POOL line doesn't...@>
        }
        // l:=xord[m]*10+xord[n]-"0"*11; {compute the length}
        /// compute the length
        let l = (xord(m).numeric_value() as integer) * 10 + (xord(n).numeric_value() as integer)
            - (b'0' as integer) * 11;
        crate::trace_expr_verbose!("l = {}", l);
        crate::trace_expr_verbose!("pool_ptr = {}", $globals.pool_ptr.get());
        // if pool_ptr+l+string_vacancies>pool_size then
        //   bad_pool('! You have to increase POOLSIZE.');
        // @.You have to increase POOLSIZE@>
        // for k:=1 to l do
        for k in 1..=l {
            // begin if eoln(pool_file) then m:=' '@+else read(pool_file,m);
            if eoln(&mut $globals.pool_file) {
                m = text_char::new(b' ' as _);
            } else {
                m = read_onearg(&mut $globals.pool_file);
            }
            #[cfg(feature = "unicode_support")]
            assert!(xord(m).numeric_value() < 128);
            // append_char(xord[m]);
            append_char(make_globals_string_view!($globals), xord(m));
            // end;
        }
        // read_ln(pool_file); g:=make_string;
        read_ln(&mut $globals.pool_file);
        $g = make_string(make_globals_string_view!($globals));
        crate::trace_expr_verbose!("g = {}", $g.get());
        // end;
    }
    // end
    use crate::io_support::eoln;
    use crate::io_support::{read_ln, read_onearg};
    use crate::pascal::integer;
    use crate::section_0004::make_globals_string_view;
    use crate::section_0004::TeXGlobalsStringView;
    use crate::section_0018::ASCII_code_literal;
    use crate::section_0019::text_char;
    use crate::section_0020::xord;
    use crate::section_0042::append_char;
    use crate::section_0043::make_string;
}
