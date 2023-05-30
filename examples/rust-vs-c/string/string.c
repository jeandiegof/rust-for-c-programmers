#include <stdlib.h>
#include <string.h>
#include <stdio.h>

char * allocate_string(char * str) {
  char * dest = malloc(strlen(str) * sizeof(char));
  strncpy(dest, str, strlen(str));

  return dest;
}

int main(void) {
  char * s1 = allocate_string("hello");
  char * s2 = s1;

  printf("%s\n", s1);
}