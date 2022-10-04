#include <stdio.h>
#include <stdlib.h>

// exercises from chapter 03
int main()
{
    // exercise 02
    printf("Please give two integers.\n");
    int first, second;
    scanf("%d %d", &first, &second);
    printf("First: %d\nSecond: %d\n", first, second);

    int sum = first + second;
    printf("%-12s%d\n", "Sum:", sum);
    int diff = first - second;
    printf("%-12s%d\n", "Difference:", diff);
    int product = first * second;
    printf("%-12s%d\n", "Product:", product);
    int quotient = first / second;
    int remainder = first % second;
    printf("%-12s%d%s%d\n", "Quotient:", quotient, " R ", remainder);
    fflush;

    // exercise 03
    printf("Please give the number of a year.\n");
    int year;
    scanf("%d", &year);
    if (year % 400 == 0)
    {
        printf("%d is a leap year.\n", year);
    } 
    else if (year % 100 == 0)
    {
        printf("%d is not a leap year.\n", year);
    } 
    else if (year % 4 == 0)
    {
        printf("%d is a leap year.\n", year);
    }
    else
    {
        printf("%d is not a leap year.\n", year);
    }
    fflush;

    // exercise 04
    printf("Please give a date with format DD.MM.YYYY.\n");
    int month, day;
    int number = 0;
    scanf("%d.%d.%d", &day, &month, &year);

    for(int i = 1; i < month; i++) 
    {
        if (i <= 7 && i % 2 == 1 || i > 7 && i % 2 == 0)
        {
            number += 31;
        }
        else if (i == 2)
        {
            number += 28;
        }
        else if (i <= 7 && i % 2 == 0 || i > 7 && i % 2 == 1)
        {
            number += 30;
        }
    }
    number += day;

    if (year % 400 == 0 && month > 2)
    {
        number += 1;
    } 
    else if (year % 4 == 0 && year % 100 != 0)
    {
        number += 1;
    }
    printf("%d.%d.%d is the %d. day of the year.\n", day, month, year, number);
    fflush;

    // exercise 05
    int upper;
    int lower;
    printf("Give a lower and upper bound for the numbers.\n");
    scanf("%d %d", &lower, &upper);

    for (int i = lower; i <= upper; i++)
    {
        if (!(i % 7))
        {
            printf("%d \n", i);
        }
        
    }
    

    // exercise 06
    printf("Give height of the LEGO stair.\n");
    int height;
    scanf("%d", &height);
    int bricks = height * (height + 1) / 2;
    printf("You need %d bricks to build that stair.\n", bricks);
    fflush;

    // exercise 07
    printf("Give the number of inputs.\n");
    scanf("%d", &number);
    
    int min = 0;
    int max = 0;
    int input;

    printf("Enter your numbers now one by one.\n");
    for (int i = 0; i < number; i++)
    {
        scanf("%d", &input);
        if (input > max)
        {
            max = input;
        }
        if (input < min)
        {
            min = input;
        }
        
    }
    printf("Minimum: %d\nMaximum: %d \n", min, max);

    // exercise 08
    
    
    // exercise 09

    // exercise 10
    printf("%2s%2s%3d%3d%3d%3d%3d%3d%3d%3d%3d%3d\n", "","", 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    for(int i = 1; i <= 10; i++)
    {
        printf("%2d%2s", i, " |");
        for(int j = 1; j <= 10; j++)
        {
            int product = i * j;
            printf("%3d", product);
        }
        printf("\n");
    }
}