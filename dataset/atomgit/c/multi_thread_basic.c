#include <pthread.h>
#include <stdlib.h>
#include <stdio.h>

// Global shared variable representing account balance
int account_balance = 1000000;  

// Mutex for protecting the account balance
pthread_mutex_t mutex = PTHREAD_MUTEX_INITIALIZER;

void* withdraw(void* v) {
    int amount = 500000;  
    for (int i = 0; i < amount; i++) {
        pthread_mutex_lock(&mutex);
        account_balance--;
        pthread_mutex_unlock(&mutex);
    }
    return NULL;
}

int main() {
    pthread_t t1, t2;
    
    pthread_create(&t1, NULL, withdraw, NULL);
    pthread_create(&t2, NULL, withdraw, NULL);

    pthread_join(t1, NULL);
    pthread_join(t2, NULL);

    printf("Final account balance: %d\n", account_balance); 

    // Destroy the mutex
    pthread_mutex_destroy(&mutex);
    return 0;
}
