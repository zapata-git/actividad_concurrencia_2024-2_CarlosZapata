#include <cassert>
#include <iostream>
#include <pthread.h>

struct MyArg {
    int a;
    int b;
};

void* myThread(void* arg) {
    MyArg* args = static_cast<MyArg*>(arg);
    std::cout << args->a << " " << args->b << std::endl;
    return nullptr;
}

int main() {
    pthread_t p;
    MyArg args = { 10, 20 };

    int rc = pthread_create(&p, nullptr, myThread, &args);
    assert(rc == 0);
    pthread_join(p, nullptr);

    std::cout << "done" << std::endl;
    return 0;
}

