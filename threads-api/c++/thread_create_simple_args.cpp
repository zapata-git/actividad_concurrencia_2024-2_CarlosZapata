#include <iostream>
#include <thread>
#include <future>

void myThread(std::promise<long long int> promise, long long int value) {
    std::cout << value << std::endl;
    promise.set_value(value + 1); // Devolvemos el resultado al hilo principal
}

int main() {
    long long int rvalue;

    // Creamos un std::promise para pasar el resultado al hilo principal
    std::promise<long long int> promise;
    std::future<long long int> future = promise.get_future();

    // Creamos un hilo y le pasamos el valor inicial
    std::thread t(myThread, std::move(promise), 100);

    // Esperamos el resultado del hilo
    rvalue = future.get();
    t.join(); // Esperamos a que el hilo termine

    std::cout << "returned " << rvalue << std::endl;

    return 0;
}

