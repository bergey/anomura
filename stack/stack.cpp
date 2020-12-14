#include <stddef.h>
#include <iostream>
using namespace std;

// This very simple stack has a fixed size (does not grow)

template<class A> class Stack {
    A **head;
    size_t top;
    size_t max_size;

public:
    class Overflow { };

    Stack(size_t requested_size = 10) {
        max_size = requested_size;
        top = 0;
        head = new A*[max_size];
    }

    A* pop() {
        if (top == 0) {
            return 0;
        } else {
            top -= 1;
            return head[top];
        }
    }

    void push(A* a) {
        if (top == max_size) {
            throw Overflow(); 
        } else {
            head[top] = a;
            top += 1;
        }
    }
};



int main() {
    Stack<int> s = Stack<int>();
    int* a = new int;
    *a = 1;
    s.push(a);
    cout << s.pop();
}
