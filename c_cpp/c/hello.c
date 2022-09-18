#include <stdio.h>

int main()
{
    printf("Hello, World! This is a native C program compiled on the command line.\n");

    int wert = 1;
    int zeile = 2;
    printf("Das ist die %d. Zeile mit dem Wert %d!\n", zeile, wert);

    // kleines Einmaleins
    for (int i = 1; i <= 10; i++)
    {
        for (int j = 1; j <= 10; j++)
        {
            int produkt = i * j;
            printf("%d mal %d ist %d\n", i, j, produkt);
        }
        
    }
    
    return 0;
}