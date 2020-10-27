//! @ Another system-dependent routine is needed to convert three internal
//! \TeX\ strings
//! into the |name_of_file| value that is used to open files. The present code
//! allows both lowercase and uppercase letters in the file name.
//! @^system dependencies@>
//
// @d append_to_name(#)==begin c:=#; incr(k);
//   if k<=file_name_size then name_of_file[k]:=xchr[c];
//   end
//
// @p procedure pack_file_name(@!n,@!a,@!e:str_number);
#[allow(unused_variables)]
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
pub(crate) fn pack_file_name(globals: &mut TeXGlobals, n: str_number, a: str_number, e:str_number) {
    // var k:integer; {number of positions filled in |name_of_file|}
    // @!c: ASCII_code; {character being packed}
    // @!j:pool_pointer; {index into |str_pool|}
    // begin k:=0;
    let k = 0;
    // for j:=str_start[a] to str_start[a+1]-1 do append_to_name(so(str_pool[j]));
    // for j:=str_start[n] to str_start[n+1]-1 do append_to_name(so(str_pool[j]));
    // for j:=str_start[e] to str_start[e+1]-1 do append_to_name(so(str_pool[j]));
    // if k<=file_name_size then name_length:=k@+else name_length:=file_name_size;
    if k < file_name_size {
        globals.name_length = k.into();
    } else {
        globals.name_length = file_name_size.into();
    }
    // for k:=name_length+1 to file_name_size do name_of_file[k]:=' ';
    for k in globals.name_length.get() + 1 ..= file_name_size {
        globals.name_of_file[k] = xchr(ASCII_code_literal!(b' '));
    }
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0011::file_name_size;
use crate::section_0020::xchr;
use crate::section_0038::str_number;
