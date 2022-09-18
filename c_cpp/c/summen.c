#include <stdio.h>
#include <stdlib.h>

int main()
{
    int anzahl;
    printf("Gib die Anzahl der Summanden ein.\n");
    scanf("%d", &anzahl);
    fflush(stdin);

    int zahl; 
    int nsum = 0;
    int psum = 0;
    int sum = 0;
    
    for(int i = 1; i <= anzahl; i++)
    {
        printf("%d. Zahl:\n", i);
        int summand;
        scanf("%d", &summand);
        if (summand < 0)
        {
            nsum += summand;
        }
        else 
        {
            psum += summand;
        }
    }
    printf("Summe aller positiven Zahlen: %d\n", psum);
    printf("Summe aller negativen Zahlen: %d\n", nsum);
    sum = psum + nsum;
    printf("Gesamtsumme: %d\n", sum);
}