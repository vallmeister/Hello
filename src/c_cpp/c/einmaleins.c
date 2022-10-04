#include <stdio.h>

int main()
{
    printf("Einmaleins. Gib Start und Ende des Bereichs an.\n");
    int start, ende;
    scanf("%d %d", &start, &ende);
    printf("Einmalseins von %d bis %d.\n", start, ende);

    for(int i = start; i <= ende; i++)
    {
        for(int j = start; j <= ende; j++)
        {
            int produkt = i * j;
            printf("%d mal %d ist %d.\n", i, j, produkt);
        }
    }
}