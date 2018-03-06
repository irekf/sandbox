#include <stddef.h>
#include <stdio.h>
#include <string.h>

#include <pthread.h>
#include <setjmp.h>
#include <signal.h>
#include <unistd.h>

#define NOT_USED(x) ( (void)(x) )

sigjmp_buf mark;

static void handleSignal(int signum, siginfo_t *info, void *ucontext) {

    NOT_USED(signum);
    NOT_USED(info);
    NOT_USED(ucontext);

    siglongjmp(mark, 1);
}

static void *runThread(void *arg) {

    printf("in runThread, arg=%p\n", arg);

    if (sigsetjmp(mark, 1) == 0) {
        for (int i = 0; i < 1000000; i++) {
            printf("stop me, already printed %d messages\n", i);
        }
    } else {
        printf("we've been sent a sign! let's stop printing useless stuff\n");
    }

    return NULL;
}

void doDangerousLowLevelWork() {

    struct sigaction sa;
    memset(&sa, 0, sizeof(sa));
    sa.sa_sigaction = handleSignal;
    sa.sa_flags = SA_SIGINFO;

    if (sigaction(SIGUSR1, &sa, NULL)) {
        perror("couldn't establish handler, leaving...\n");
        return;
    }

    pthread_t pid;
    if (pthread_create(&pid, NULL, runThread, NULL) == 0) {
        usleep(5000);
        pthread_kill(pid, SIGUSR1);
        pthread_join(pid, NULL);
    } else {
        perror("thread not created, leaving...\n");
        return;
    }

    printf("okay, we're done with C, returning to Rust land\n");
}
