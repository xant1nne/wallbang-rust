#include <idc.idc>
static main() {
    auto f,i,p,e,t;
    auto_wait(); f=fopen("C:\\wallbang-rust\\target\\client_remaining.txt","w");
    for(i=0;i<18;i++) {
        t=get_qword(0x181C31D48+i*8);
        fprintf(f,"SLOT %d %X\n",i,t);
        p=t;e=get_func_attr(t,FUNCATTR_END);
        for(;p!=BADADDR && p<e && p<t+160;p=next_head(p,e)) fprintf(f,"%X %s\n",p,generate_disasm_line(p,0));
    }
    p=0x1809E22B0;
    for(i=0;i<40;i++) {fprintf(f,"%X %s\n",p,generate_disasm_line(p,0));p=next_head(p,BADADDR);}
    fclose(f);qexit(0);
}
