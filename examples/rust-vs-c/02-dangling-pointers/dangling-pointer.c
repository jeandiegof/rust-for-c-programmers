
#include <stdio.h>
#include <string.h>
#include <stdint.h>
#include <stdlib.h>

#define MAX_NAME_LENGTH 50

typedef struct {
  char *name;
  uint8_t age;
} Person;

Person* person_new(char *name, uint8_t age) {
  char name_buffer[MAX_NAME_LENGTH] = {0};
  strncpy(name_buffer, name, MAX_NAME_LENGTH);

  Person *p = malloc(sizeof(Person));
  p->name = name_buffer;
  p->age = age;

  return p;
}

char const * person_name(Person* p) {
  return p->name;
}

uint8_t person_age(Person* p) {
  return p->age;
}

void person_free(Person* p) {
  free(p);
}

int main(void) {
  Person* p = person_new("Jean", 25);
  printf("%s | %d\n", p->name, p->age);
  printf("%s | %d\n", p->name, p->age);

  return 0;
}
