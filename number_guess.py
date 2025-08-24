import random
lösung = random.randint(-1,100)
drehungen = True
anzahl = 0
while drehungen:
    if anzahl == 10:
        print("das ist falsch")
        drehungen = False
    if drehungen == True:
        user_input = int(input("gib eine zahl zwische 0 und hundert ein "))
    if user_input == lösung:
        print("das ist richtig")
        drehungen = False
    elif user_input != lösung and drehungen == True:
        print("das ist falsch du hast noch ")
        anzahl += 1
