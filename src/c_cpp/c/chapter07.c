#include <stdio.h>
#include <stdlib.h>

// exercise 1
void one() {
    
}

void two() {}

void three() {}

void four() {}

void five() {}

void six() {}

void seven() {}

void eight() {}

void nine() {}

void ten() {}

int main() {
    int exercise;
    printf("Choose the exercise to do (1-10).\n");
    scanf("%d", &exercise);

    if (exercise == 1) {
        one();
    } else if (exercise == 2) {
        two();
    } else if (exercise == 3) {
        three();
    } else if (exercise == 4) {
        four();    
    } else if (exercise == 5) {
        five();
    } else if (exercise == 6) {
        six();
    } else if (exercise == 7) {
        seven();
    } else if (exercise == 8) {
        eight();
    } else if (exercise == 9) {
        nine();
    } else if (exercise == 10) {
        ten();
    }
}
