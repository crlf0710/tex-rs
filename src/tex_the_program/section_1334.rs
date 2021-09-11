//! @ The present section goes directly to the log file instead of using
//! |print| commands, because there's no need for these strings to take
//! up |str_pool| memory when a non-{\bf stat} version of \TeX\ is being used.
//
// @<Output statistics...@>=
#[cfg(feature = "statistics")]
pub(crate) macro Output_statistics_about_this_job($globals:expr) {{
    // if log_opened then
    if $globals.log_opened {
        // begin wlog_ln(' ');
        wlog_ln(make_globals_log_view!($globals), " ");
        // wlog_ln('Here is how much of TeX''s memory',' you used:');
        wlog_ln(
            make_globals_log_view!($globals),
            format!("{}{}", "Here is how much of TeX's memory", " you used:"),
        );
        // @.Here is how much...@>
        // wlog(' ',str_ptr-init_str_ptr:1,' string');
        wlog(
            make_globals_log_view!($globals),
            format!(
                " {} string",
                $globals.str_ptr.get() - $globals.init_str_ptr.get()
            ),
        );
        // if str_ptr<>init_str_ptr+1 then wlog('s');
        if $globals.str_ptr != $globals.init_str_ptr + 1 {
            wlog(make_globals_log_view!($globals), "s");
        }
        // wlog_ln(' out of ', max_strings-init_str_ptr:1);@/
        wlog_ln(
            make_globals_log_view!($globals),
            format!(
                " out of {}",
                max_strings as integer - $globals.init_str_ptr.get() as integer
            ),
        );
        // wlog_ln(' ',pool_ptr-init_pool_ptr:1,' string characters out of ',
        //   pool_size-init_pool_ptr:1);@/
        wlog_ln(
            make_globals_log_view!($globals),
            format!(
                " {} string characters out of {}",
                $globals.pool_ptr - $globals.init_pool_ptr,
                pool_size as integer - $globals.init_pool_ptr.get() as integer
            ),
        );
        // wlog_ln(' ',lo_mem_max-mem_min+mem_end-hi_mem_min+2:1,@|
        //   ' words of memory out of ',mem_end+1-mem_min:1);@/
        wlog_ln(
            make_globals_log_view!($globals),
            format!(
                " {} words of memory out of {}",
                $globals.lo_mem_max as integer - mem_min as integer + $globals.mem_end as integer
                    - $globals.hi_mem_min as integer
                    + 2,
                $globals.mem_end as integer + 1 - mem_min as integer
            ),
        );
        // wlog_ln(' ',cs_count:1,' multiletter control sequences out of ',
        //   hash_size:1);@/
        wlog_ln(
            make_globals_log_view!($globals),
            format!(
                " {} multiletter control sequences out of {}",
                $globals.cs_count, hash_size
            ),
        );
        // wlog(' ',fmem_ptr:1,' words of font info for ',
        //   font_ptr-font_base:1,' font');
        wlog(
            make_globals_log_view!($globals),
            format!(
                " {} words of font info for {} font",
                $globals.fmem_ptr.get(),
                $globals.font_ptr.get() as integer - font_base as integer
            ),
        );
        // if font_ptr<>font_base+1 then wlog('s');
        if $globals.font_ptr.get() as integer != font_base as integer + 1 {
            wlog(make_globals_log_view!($globals), "s");
        }
        // wlog_ln(', out of ',font_mem_size:1,' for ',font_max-font_base:1);@/
        wlog_ln(
            make_globals_log_view!($globals),
            format!(", out of {} for {}", font_mem_size, font_max - font_base),
        );
        // wlog(' ',hyph_count:1,' hyphenation exception');
        wlog(
            make_globals_log_view!($globals),
            format!(" {} hyphenation exception", $globals.hyph_count.get()),
        );
        // if hyph_count<>1 then wlog('s');
        if $globals.hyph_count != 1 {
            wlog(make_globals_log_view!($globals), "s");
        }
        // wlog_ln(' out of ',hyph_size:1);@/
        wlog_ln(
            make_globals_log_view!($globals),
            format!(" out of {}", hyph_size),
        );
        // wlog_ln(' ',max_in_stack:1,'i,',max_nest_stack:1,'n,',@|
        //   max_param_stack:1,'p,',@|
        //   max_buf_stack+1:1,'b,',@|
        //   max_save_stack+6:1,'s stack positions out of ',@|
        //   stack_size:1,'i,',
        //   nest_size:1,'n,',
        //   param_size:1,'p,',
        //   buf_size:1,'b,',
        //   save_size:1,'s');
        wlog_ln(
            make_globals_log_view!($globals),
            format!(
                " {}i,{}n,{}p,{}b,{}s stack positions out of {}i,{}n,{}p,{}b,{}s",
                $globals.max_in_stack.get(),
                $globals.max_nest_stack.get(),
                $globals.max_param_stack,
                $globals.max_buf_stack.get() + 1,
                $globals.max_save_stack.get(),
                stack_size,
                nest_size,
                param_size,
                buf_size,
                save_size
            ),
        );
        // end
    }
    use crate::pascal::integer;
    use crate::section_0011::buf_size;
    use crate::section_0011::font_max;
    use crate::section_0011::font_mem_size;
    use crate::section_0011::max_strings;
    use crate::section_0011::mem_min;
    use crate::section_0011::nest_size;
    use crate::section_0011::param_size;
    use crate::section_0011::pool_size;
    use crate::section_0011::save_size;
    use crate::section_0011::stack_size;
    use crate::section_0012::font_base;
    use crate::section_0012::hash_size;
    use crate::section_0012::hyph_size;
    use crate::section_0056::wlog;
    use crate::section_0056::wlog_ln;
}}
