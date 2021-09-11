//! @ When the following code becomes active, we have matched tokens from |s| to
//! the predecessor of |r|, and we have found that |cur_tok<>info(r)|. An
//! interesting situation now presents itself: If the parameter is to be
//! delimited by a string such as `\.{ab}', and if we have scanned `\.{aa}',
//! we want to contribute one `\.a' to the current parameter and resume
//! looking for a `\.b'. The program must account for such partial matches and
//! for others that can be quite complex.  But most of the time we have |s=r|
//! and nothing needs to be done.
//!
//! Incidentally, it is possible for \.{\\par} tokens to sneak in to certain
//! parameters of non-\.{\\long} macros. For example, consider a case like
//! `\.{\\def\\a\#1\\par!\{...\}}' where the first \.{\\par} is not followed
//! by an exclamation point. In such situations it does not seem appropriate
//! to prohibit the \.{\\par}, so \TeX\ keeps quiet about this bending of
//! the rules.

// @<Contribute the recently matched tokens to the current parameter...@>=
pub(crate) macro Contribute_the_recently_matched_tokens_to_the_current_parameter__and_goto_continue_if_a_partial_match_is_still_in_effect__but_abort_if_s_null {
    ($globals:expr, $r:expr, $info_r:expr, $m:expr, $s:expr, $p:expr, $q:expr) => {
        // if s<>r then
        if $s != $r {
            // if s=null then @<Report an improper use of the macro and abort@>
            if $s == null {
                crate::section_0398::Report_an_improper_use_of_the_macro_and_abort!($globals);
            }
            // else  begin t:=s;
            else {
                let t = $s;
                let info_t = info_tok!($globals, t);
                // repeat store_new_token(info(t)); incr(m); u:=link(t); v:=s;
                loop {
                    store_new_token!($globals, info_t.get(), $p, $q);
                    incr!($m);
                    let u = link!($globals, t);
                    let v = $s;
                    crate::region_forward_label!(
                        |'done|
                        {
                            todo!("inner");
                            // loop@+  begin if u=r then
                            //     if cur_tok<>info(v) then goto done
                            //     else  begin r:=link(v); goto continue;
                            //       end;
                            //   if info(u)<>info(v) then goto done;
                            //   u:=link(u); v:=link(v);
                            //   end;
                        }
                        // done: t:=link(t);
                        'done <-
                    );
                    t = link!($globals, t);
                    info_t = info_tok!($globals, t);
                    // until t=r;
                    if t == $r {
                        break;
                    }
                }
                // r:=s; {at this point, no tokens are recently matched}
                /// at this point, no tokens are recently matched
                {
                    $r = $s;
                    $info_r = info_tok!($globals, $r);
                }
                // end
            }
        }
        use crate::section_0016::incr;
        use crate::section_0115::null;
        use crate::section_0118::info_tok;
        use crate::section_0118::link;
        use crate::section_0371::store_new_token;
    }
}
