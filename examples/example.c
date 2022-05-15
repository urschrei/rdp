// compile with e.g. `clang -lrdp -L target/release -o rdp_example  examples/example.c` from project root
// run with `LD_LIBRARY_PATH=target/release ./rdp_example` from project root
#include <stdio.h>
#include <stdint.h>

typedef struct {
  void *data;
  size_t len;
} externalarray;

extern externalarray simplify_rdp_ffi(externalarray, double);
extern void drop_float_array(externalarray);

int main(int argc, const char *argv[]) {
  double input[5][2] = {{0.0, 0.0}, {5.0, 4.0}, {11.0, 5.5}, {17.3, 3.2}, {27.8, 0.1}};
   // cast to void pointer and length
  size_t len = sizeof(input);
  void (*vp) = input;
  // vp = &input;
  externalarray ea = {
    .len = len,
    .data = vp
  };
  externalarray adj = simplify_rdp_ffi(ea, 1.0);
  // cast back to array
  double adj_arr = *(double *)adj.data;

  // printf("%f\n", adj_arr);
  // print all values in a loop
  for (int i = 0; i < adj.len; i++)
      printf("%f\n", ((double*)adj.data)[i]);
  drop_float_array(adj);
  getchar();
  return 0;
}
