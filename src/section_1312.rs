//! @ @<Undump the dynamic memory@>=
//! undump(lo_mem_stat_max+1000)(hi_mem_stat_min-1)(lo_mem_max);
//! undump(lo_mem_stat_max+1)(lo_mem_max)(rover);
//! p:=mem_bot; q:=rover;
//! repeat for k:=p to q+1 do undump_wd(mem[k]);
//! p:=q+node_size(q);
//! if (p>lo_mem_max)or((q>=rlink(q))and(rlink(q)<>rover)) then goto bad_fmt;
//! q:=rlink(q);
//! until q=rover;
//! for k:=p to lo_mem_max do undump_wd(mem[k]);
//! if mem_min<mem_bot-2 then {make more low memory available}
//!   begin p:=llink(rover); q:=mem_min+1;
//!   link(mem_min):=null; info(mem_min):=null; {we don't use the bottom word}
//!   rlink(p):=q; llink(rover):=q;@/
//!   rlink(q):=rover; llink(q):=p; link(q):=empty_flag;
//!   node_size(q):=mem_bot-q;
//!   end;
//! undump(lo_mem_max+1)(hi_mem_stat_min)(hi_mem_min);
//! undump(null)(mem_top)(avail); mem_end:=mem_top;
//! for k:=hi_mem_min to mem_end do undump_wd(mem[k]);
//! undump_int(var_used); undump_int(dyn_used)
//!
