#include <unistd.h>
#include <sys/syscall.h>
#include <stdio.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <signal.h>
#define mycall 450

void handler(){
    printf("1 za sve kernel procese\n");
    printf("ili konkretan PID procesa\n");
    fflush(stdout);
}
int main(){
    printf("Pokrece se iz user-spacea sistemski poziv\n");

    pid_t child = fork();
    if(!child){
        printf("Pravi se 8 procesa\n");
        execl("./kernel_modules/tree_demo.o", "tree_demo.o", NULL);
    }
    else{
        signal(SIGUSR1, handler);
        int pid = 0;
        
        scanf("%d", &pid);
        long res = syscall(mycall, pid);
        printf("Rezultat sistemskog poziva: %ld\n", res);
    }
    
    return 0;
}