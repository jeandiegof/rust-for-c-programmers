#include <stdio.h>
#include <string.h>
#include <stdint.h>
#include <stdlib.h>

#define MAX_NAME_LENGTH 50

typedef struct {
  char name[MAX_NAME_LENGTH];
  uint8_t age;
} Person;

Person* person_new(char *name, uint8_t age) {
  Person* person = malloc(sizeof(Person));

  if (person != NULL) {
    strncpy(person->name, name, MAX_NAME_LENGTH);
    person->age = age;
  }

  return person;
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

  person_free(p);
  printf("%s | %d\n", p->name, p->age);

  return 0;
}
