int isPrime(int x) {
  for (int i = 2; i*i <= x; i++) {
    if (x % i == 0) {
      return 0;
    }
  }
  return 1;
}

int main() {
  int count = 0;
  for (int i = 0; i < 10000; i++) {
    if (isPrime(i)) {
      count++;
    }
  }
  __asm__ volatile ("mov %0, %%eax\n"
                    "int $0x80\n"
                    :
                    : "m"(count)
                    : "eax");
}
