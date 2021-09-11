//! @ Another system-dependent routine is needed to convert three internal
//! \TeX\ strings
//! into the |name_of_file| value that is used to open files. The present code
//! allows both lowercase and uppercase letters in the file name.
//! @^system dependencies@>
//
// @d append_to_name(#)==begin c:=#; incr(k);
pub(crate) macro append_to_name($globals:expr, $val:expr, $k:expr) {{
    let c = $val;
    incr!($k);
    // if k<=file_name_size then name_of_file[k]:=xchr[c];
    if ($k as integer) < file_name_size as integer {
        $globals.name_of_file[$k as u16] = xchr(c);
    }
    // end
    use crate::pascal::integer;
    use crate::section_0016::incr;
    use crate::section_0020::xchr;
}}

// @p procedure pack_file_name(@!n,@!a,@!e:str_number);
#[allow(unused_variables)]
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
pub(crate) fn pack_file_name(
    globals: &mut TeXGlobals,
    n: str_number,
    a: str_number,
    e: str_number,
) {
    // var k:integer; {number of positions filled in |name_of_file|}
    // @!c: ASCII_code; {character being packed}
    // @!j:pool_pointer; {index into |str_pool|}
    // begin k:=0;
    let mut k = 0;
    #[cfg(not(feature = "unicode_support"))]
    {
        // for j:=str_start[a] to str_start[a+1]-1 do append_to_name(so(str_pool[j]));
        // for j:=str_start[n] to str_start[n+1]-1 do append_to_name(so(str_pool[j]));
        // for j:=str_start[e] to str_start[e+1]-1 do append_to_name(so(str_pool[j]));
        todo!();
    }
    #[cfg(feature = "unicode_support")]
    {
        for ch in globals.str_pool.str_ascii_codes(&globals.str_start, a) {
            append_to_name!(globals, xord(ch), k);
        }
        for ch in globals.str_pool.str_ascii_codes(&globals.str_start, n) {
            append_to_name!(globals, xord(ch), k);
        }
        for ch in globals.str_pool.str_ascii_codes(&globals.str_start, e) {
            append_to_name!(globals, xord(ch), k);
        }
    }
    // if k<=file_name_size then name_length:=k@+else name_length:=file_name_size;
    if k < file_name_size {
        globals.name_length = k.into();
    } else {
        globals.name_length = file_name_size.into();
    }
    // for k:=name_length+1 to file_name_size do name_of_file[k]:=' ';
    for k in globals.name_length.get() + 1..=file_name_size {
        globals.name_of_file[k] = xchr(ASCII_code_literal!(b' '));
    }
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0011::file_name_size;
use crate::section_0018::ASCII_code_literal;
use crate::section_0020::xchr;
use crate::section_0020::xord;
use crate::section_0038::str_number;
