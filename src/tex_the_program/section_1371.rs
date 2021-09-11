//! @ The final line of this routine is slightly subtle; at least, the author
//! didn't think about it until getting burnt! There is a used-up token list
//! @^Knuth, Donald Ervin@>
//! on the stack, namely the one that contained |end_write_token|. (We
//! insert this artificial `\.{\\endwrite}' to prevent runaways, as explained
//! above.) If it were not removed, and if there were numerous writes on a
//! single page, the stack would overflow.

// @d end_write_token==cs_token_flag+end_write
pub(crate) macro end_write_token() {
    crate::section_0297::cur_tok_type::from_cs(crate::section_0222::end_write as _)
}

// @<Expand macros in the token list and...@>=
pub(crate) macro Expand_macros_in_the_token_list_and_make_link_def_ref_point_to_the_result($globals:expr, $p:expr) {{
    let globals = &mut *$globals;
    let p = $p;
    /// temporary variables for list manipulation
    let (mut q, r): (pointer, pointer);
    // q:=get_avail; info(q):=right_brace_token+"}";@/
    q = get_avail(globals);
    info_tok_assign!(
        globals,
        q,
        cur_tok_type::new(right_brace_token + b'}' as cur_tok_repr)
    );
    // r:=get_avail; link(q):=r; info(r):=end_write_token; ins_list(q);@/
    r = get_avail(globals);
    link!(globals, q) = r;
    info_tok_assign!(globals, r, end_write_token!());
    ins_list!(globals, q);
    // begin_token_list(write_tokens(p),write_text);@/
    begin_token_list(globals, write_tokens!(globals, p), write_text);
    // q:=get_avail; info(q):=left_brace_token+"{"; ins_list(q);
    q = get_avail(globals);
    info_tok_assign!(
        globals,
        q,
        cur_tok_type::new(left_brace_token + b'{' as cur_tok_repr)
    );
    ins_list!(globals, q);
    // {now we're ready to scan
    //   `\.\{$\langle\,$token list$\,\rangle$\.{\} \\endwrite}'}
    /// now we're ready to scan `{< token list >} \endwrite`
    const _: () = ();
    /// saved `mode`
    let old_mode;

    // old_mode:=mode; mode:=0;
    //   {disable \.{\\prevdepth}, \.{\\spacefactor}, \.{\\lastskip}, \.{\\prevgraf}}
    /// disable `\prevdepth`, `\spacefactor`, `\lastskip`, `\prevgraf`
    const _: () = ();

    old_mode = mode!(globals);
    mode!(globals) = 0.into();
    // cur_cs:=write_loc; q:=scan_toks(false,true); {expand macros, etc.}
    /// expand macros, etc.
    const _: () = ();
    globals.cur_cs = globals.write_loc;
    q = scan_toks(globals, false, true)?;

    let _ = q;
    // get_token;@+if cur_tok<>end_write_token then
    //   @<Recover from an unbalanced write command@>;
    get_token(globals)?;
    if globals.cur_tok != end_write_token!() {
        todo!("recover");
    }
    // mode:=old_mode;
    mode!(globals) = old_mode;
    // end_token_list {conserve stack space}
    /// conserve stack space
    end_token_list(globals);
    use crate::section_0115::pointer;
    use crate::section_0118::info_tok_assign;
    use crate::section_0118::link;
    use crate::section_0120::get_avail;
    use crate::section_0213::mode;
    use crate::section_0289::left_brace_token;
    use crate::section_0289::right_brace_token;
    use crate::section_0297::cur_tok_repr;
    use crate::section_0297::cur_tok_type;
    use crate::section_0307::write_text;
    use crate::section_0323::begin_token_list;
    use crate::section_0323::ins_list;
    use crate::section_0324::end_token_list;
    use crate::section_0365::get_token;
    use crate::section_0473::scan_toks;
    use crate::section_1341::write_tokens;
}}
