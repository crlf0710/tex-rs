//! ` `

// @<Look up the characters of list |r| in the hash table...@>=
macro_rules! Look_up_the_characters_of_list_r_in_the_hash_table__and_set_cur_cs {
    ($globals:expr, $p:expr, $r:expr) => {{
        /// index into `buffer`
        let mut j: u16_from_0_to_n<buf_size_TYPENUM>;

        // j:=first; p:=link(r);
        j = $globals.first;
        $p = link!($globals, $r);
        // while p<>null do
        while $p != null {
            // begin if j>=max_buf_stack then
            if j >= $globals.max_buf_stack {
                // begin max_buf_stack:=j+1;
                $globals.max_buf_stack = j + 1;
                // if max_buf_stack=buf_size then
                if $globals.max_buf_stack == buf_size {
                    // overflow("buffer size",buf_size);
                    todo!("overflow");
                    // @:TeX capacity exceeded buffer size}{\quad buffer size@>
                }
                // end;
            }
            // buffer[j]:=info(p) mod @'400; incr(j); p:=link(p);
            $globals.buffer[j] = info_tok!($globals, $p).get_cmd_and_chr().unwrap().1.into();
            incr!(j);
            $p = link!($globals, $p);
            // end;
        }

        // if j>first+1 then
        if j > $globals.first + 1 {
            // begin no_new_control_sequence:=false; cur_cs:=id_lookup(first,j-first);
            $globals.no_new_control_sequence = false;
            $globals.cur_cs = id_lookup(
                $globals,
                $globals.first.get() as integer,
                j.get() as integer - $globals.first.get() as integer,
            );
            // no_new_control_sequence:=true;
            $globals.no_new_control_sequence = true;
        // end
        }
        // else if j=first then cur_cs:=null_cs {the list is empty}
        else if j == $globals.first {
            /// the list is empty
            const _: () = ();
            $globals.cur_cs = null_cs;
        }
        // else cur_cs:=single_base+buffer[first] {the list has length one}
        else {
            /// the list has length one
            const _: () = ();
            if $globals.buffer[$globals.first].numeric_value() > 255 {
                todo!();
            }
            $globals.cur_cs =
                (single_base + $globals.buffer[$globals.first].numeric_value()) as pointer;
        }

        use crate::pascal::integer;
        use crate::pascal::u16_from_0_to_n;
        use crate::section_0011::buf_size;
        use crate::section_0011::buf_size_TYPENUM;
        use crate::section_0115::null;
        use crate::section_0222::null_cs;
        use crate::section_0222::single_base;
        use crate::section_0259::id_lookup;
    }};
}
