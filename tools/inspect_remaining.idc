#include <idc.idc>
static dump(f, ea) {
    auto p, end, n;
    p = get_func_attr(ea, FUNCATTR_START);
    end = get_func_attr(ea, FUNCATTR_END);
    if (p == BADADDR) { p = ea; end = ea + 400; }
    fprintf(f, "FUNCTION %X\n", p);
    for(n=0; n<110 && p<end; n++) {
        fprintf(f, "%X %s\n", p, generate_disasm_line(p, 0));
        p = next_head(p, end);
    }
}
static main() {
    auto f, ea, s, x;
    auto_wait();
    f = fopen(ARGV[1], "w");
    for(ea=get_first_seg(); ea!=BADADDR; ea=next_head(ea, BADADDR)) {
        if (is_strlit(get_full_flags(ea))) {
            s = get_strlit_contents(ea, -1, STRTYPE_C);
            if (strstr(s,"Source2EngineToClient001")>=0 || strstr(s,"vote_cast")>=0 || strstr(s,"setinfo")>=0) {
                fprintf(f,"STRING %X %s\n",ea,s);
                for(x=get_first_dref_to(ea);x!=BADADDR;x=get_next_dref_to(ea,x)) dump(f,x);
            }
        }
    }
    fclose(f);
    qexit(0);
}
