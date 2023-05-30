#include <stdio.h>
#include <string.h>
#include <stdlib.h>

char * allocate_string(char * str) {
  char * dest = malloc(strlen(str) * sizeof(char));
  strncpy(dest, str, strlen(str));

  return dest;
}

void print_str(char const * str) {
  printf("%s\n", str);

  char * mut_str = (char *)str;
  strcpy(mut_str, "bye world!");
}

int main(void) {
  char const * str = allocate_string("hello world!");

  print_str(str);
  printf("%s\n", str);

  return 0;
}
