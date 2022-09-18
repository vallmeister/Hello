#include <stdio.h>
#include <stdlib.h>

// exercise 1
void one() {
    float start, target, interest;
    int years = 0;
    printf("Please input your start capital, interest rate and target capital.\n");
    scanf("%f %f %f", &start, &interest, &target);

    while (start < target)
    {
        years++;
        start *= (1.0 + interest);
    }
    printf("You need %d years to achieve your desired capital.\n", years);
}

// exercise 2
void two() {
    int minuend, subtrahend, remainder;

    printf("Give the numbers to calculate ggT for.\n");
    scanf("%d %d", &minuend, &subtrahend);
    if (minuend < subtrahend)
    {
        int temp = minuend;
        minuend = subtrahend;
        subtrahend = temp;
    }
    
    while (subtrahend != minuend)
    {
        remainder = minuend - subtrahend;
        if (remainder > subtrahend)
        {
            minuend = remainder;
        } else {
            minuend = subtrahend;
            subtrahend = remainder;
        }
    }
    printf("The ggT is: %d\n", subtrahend);
}

// exercise 3
void three() {

}

// exercise 4
void four() {

}

// exercise 5
void five() {
    
}

// exercise 6
void six() {

}

// exercise 7
void seven() {

}

// exercise 8
void eight() {

}

// exercise 9
void nine() {

}

// exercise 10
void ten() {

}

int main(){
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
