//! @ The present section goes directly to the log file instead of using
//! |print| commands, because there's no need for these strings to take
//! up |str_pool| memory when a non-{\bf stat} version of \TeX\ is being used.
//!
//! @<Output statistics...@>=
//! if log_opened then
//!   begin wlog_ln(' ');
//!   wlog_ln('Here is how much of TeX''s memory',' you used:');
//! @.Here is how much...@>
//!   wlog(' ',str_ptr-init_str_ptr:1,' string');
//!   if str_ptr<>init_str_ptr+1 then wlog('s');
//!   wlog_ln(' out of ', max_strings-init_str_ptr:1);@/
//!   wlog_ln(' ',pool_ptr-init_pool_ptr:1,' string characters out of ',
//!     pool_size-init_pool_ptr:1);@/
//!   wlog_ln(' ',lo_mem_max-mem_min+mem_end-hi_mem_min+2:1,@|
//!     ' words of memory out of ',mem_end+1-mem_min:1);@/
//!   wlog_ln(' ',cs_count:1,' multiletter control sequences out of ',
//!     hash_size:1);@/
//!   wlog(' ',fmem_ptr:1,' words of font info for ',
//!     font_ptr-font_base:1,' font');
//!   if font_ptr<>font_base+1 then wlog('s');
//!   wlog_ln(', out of ',font_mem_size:1,' for ',font_max-font_base:1);@/
//!   wlog(' ',hyph_count:1,' hyphenation exception');
//!   if hyph_count<>1 then wlog('s');
//!   wlog_ln(' out of ',hyph_size:1);@/
//!   wlog_ln(' ',max_in_stack:1,'i,',max_nest_stack:1,'n,',@|
//!     max_param_stack:1,'p,',@|
//!     max_buf_stack+1:1,'b,',@|
//!     max_save_stack+6:1,'s stack positions out of ',@|
//!     stack_size:1,'i,',
//!     nest_size:1,'n,',
//!     param_size:1,'p,',
//!     buf_size:1,'b,',
//!     save_size:1,'s');
//!   end
//!