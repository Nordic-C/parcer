int m(int x) {
    return x;
}

int main(void) {
    int x = 0;
    return m(x += 1);
}

int test() {
    if (1)
        return 1;
}