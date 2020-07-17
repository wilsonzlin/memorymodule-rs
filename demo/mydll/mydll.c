#define DllExport __declspec(dllexport)

DllExport int add(int a, int b) {
  return a + b;
}
