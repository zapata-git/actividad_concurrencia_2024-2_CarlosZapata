#include <iostream>
#include <thread>
#include <memory>
#include <cassert>

struct MyArg {
    int a;
    int b;
};

struct MyRet {
    int x;
    int y;
};

void myThread(const MyArg& args, std::shared_ptr<MyRet> rvals) {
    std::cout << "args " << args.a << " " << args.b << std::endl;
    rvals->x = 1;
    rvals->y = 2;
}

int main() {
    MyArg args = {10, 20};
    auto rvals = std::make_shared<MyRet>();

    std::thread t(myThread, std::ref(args), rvals);

    t.join(); // Esperamos a que el hilo termine

    std::cout << "returned " << rvals->x << " " << rvals->y << std::endl;

    return 0;
}

