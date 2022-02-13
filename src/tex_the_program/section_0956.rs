//! ` `

// @<Pack the family into |trie| relative to |h|@>=
pub(crate) macro Pack_the_family_into_trie_relative_to_h($globals:expr, $h:expr, $p:expr, $z:expr) {{
    /// runs through the family starting at `p`
    let mut q: trie_pointer;
    // trie_taken[h]:=true; trie_ref[p]:=h; q:=p;
    $globals.trie_taken[$h.get()] = true;
    trie_ref!($globals, $p) = $h;
    q = $p;
    // repeat z:=h+so(trie_c[q]); l:=trie_back(z); r:=trie_link(z);
    loop {
        /// left and right neighbors
        let (mut l, r): (trie_pointer, trie_pointer);

        let trie_c_q = $globals.trie_c[q];
        let trie_c_q_u8 = if trie_c_q.numeric_value() > 255 {
            todo!("not yet implemented in {}", file!());
        } else {
            trie_c_q.numeric_value() as u8
        };
        $z = trie_pointer::new(($h.get() as integer + trie_c_q_u8 as integer) as _);
        l = trie_back!($globals, $z.get()).into();
        r = trie_link!($globals, $z.get()).into();
        // trie_back(r):=l; trie_link(l):=r; trie_link(z):=0;
        trie_back!($globals, r) = l.get();
        trie_link!($globals, l) = r.get();
        trie_link!($globals, $z.get()) = 0;
        // if l<256 then
        if l < 256 {
            let ll: u16;
            // begin if z<256 then ll:=z @+else ll:=256;
            if $z < 256 {
                ll = $z.get();
            } else {
                ll = 256;
            }
            // repeat trie_min[l]:=r; incr(l);
            loop {
                $globals.trie_min[l.get() as usize] = r;
                incr!(l);
                // until l=ll;
                if l == ll {
                    break;
                }
            }
            // end;
        }
        // q:=trie_r[q];
        q = $globals.trie_r[q];
        // until q=0
        if q == 0 {
            break;
        }
    }
    use crate::pascal::integer;
    use crate::section_0016::incr;
    use crate::section_0920::trie_pointer;
    use crate::section_0921::trie_link;
    use crate::section_0950::trie_back;
    use crate::section_0950::trie_ref;
}}
