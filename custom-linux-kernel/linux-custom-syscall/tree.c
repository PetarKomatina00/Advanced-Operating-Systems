#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/sched.h>
#include <linux/moduleparam.h>
#include <linux/init.h>
#include <linux/cred.h>
#include <linux/mm.h>
#include <linux/delay.h>
#include <linux/kernel.h>
#include <linux/syscalls.h>

void dfs_print(struct task_struct *task, int* count)
{

    struct list_head *head;
    struct task_struct *child;
    (*count)++;
    int execution_time = task->utime + task->stime;
    // kuid_t kuid = task->cred->uid;
    // uid_t uid = __kuid_val(kuid);
    long vm = 0;
    if (task->mm)
    {
        // User defined processes have memory | Kernel defined processes do not.
        vm = task->mm->total_vm;
    }
    printk(KERN_INFO "PID: %d | Prioritet: %d | Nice: %d | Korisnik :%d | Kol. Memorija %d | Vreme izvrsenja %ld | Ime procesa: %s", task->pid, task->prio, task_nice(task), task->cred->uid, vm, execution_time, task->comm);
    list_for_each_entry(child, &task->children, sibling){
        dfs_print(child, count);
    }
}
SYSCALL_DEFINE1(show_tree, int, pid)
{

    printk(KERN_INFO "Syscall Pocinje");
    struct task_struct *task;

    if (pid <= 0)
    {
        printk(KERN_INFO "Current pid: %d\n", current->pid);
        pid = current->pid;
        printk(KERN_INFO "PID: %d\n", pid);
    }
    rcu_read_lock();
    printk(KERN_INFO "PID: %d\n", pid);
    task = pid_task(find_vpid(pid), PIDTYPE_PID);
    if (task == NULL)
    {
        rcu_read_unlock();
        printk(KERN_ERR "PID Ne postoji\n");
        return -1;
    }
    int process_count = 0;
    dfs_print(task, &process_count);
    printk(KERN_INFO "Ukupan broj procesa u stablu je %d\n", process_count);
    rcu_read_unlock();
    return 0;
}
