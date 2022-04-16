#include <iostream>
#include <boost/stacktrace.hpp>

void bar(){
    std::cout << boost::stacktrace::stacktrace();
}
int foo(){
    bar();
    return 1;
}

int main(){
    foo();
    return 0;
}
