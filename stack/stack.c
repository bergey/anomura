#include <stddef.h>

// This very simple stack has a fixed size (does not grow)
// and can only store int

class Stack {
    int **head;
    size_t top;
    size_t max_size;

public:
    Stack(size_t max_size = 10);
    int* pop();
    void push(int* a);
};

Stack::Stack(size_t requested_size) {
    max_size = requested_size;
    top = 0;
    head = new int*[max_size];
}

Stack::pop() {
    if top == 0 {
            return 0;
        } else {
        top -= 1;
        return head[top]
    }
}
