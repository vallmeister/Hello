#!/usr/bin/env python

print("Hello World!")

# Zeilenkommentar
print("Zahlen raten")

geheimnis = 1988
versuch = -1
zaehler = 0
""" Blockkommentar
Test """
while geheimnis != versuch:
    versuch = int(input("Raten Sie: "))

    if versuch < geheimnis:
        print("Zu klein!")
    elif versuch > geheimnis:
        print("Zu gross!")
    
    zaehler += 1

print("Super, Sie haben es in", zaehler, "Versuchen geschafft!")