int mul(int x, int y) { return x * y; }

int main() {
  int i = 1;
  int result = 1;
  while (i < 10) {
    result = mul(i, result);
    i = i + 1;
  }
  return 0;
}
