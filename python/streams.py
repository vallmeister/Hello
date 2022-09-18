fileObject = open("woerterbuch.txt", "r")
words = {}
# read dictionary from file
for line in fileObject:
    line = line.strip()
    word = line.split(" ")
    words[word[0]] = word[1]
fileObject.close()

# interface for user
while True:
    query = input("Geben Sie ein Wort ein oder 0 zum Beenden: ")
    if query == "0":
        break
    elif query in words:
        print("Das deutsche Wort lautet: ", words[query])
    else:
        print("Das Wort ist unbekannt.")

# write data
fobj = open("ausgabe.txt", "w")
fobj2 = open("ausgabe2.txt", "w")

for english in words:
    fobj.write("{} {}\n".format(english, words[english]))
    s = english + " " + words[english] + "\n"
    fobj2.write(s)
fobj.close
fobj2.close
