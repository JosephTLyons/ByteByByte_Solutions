#!/usr/bin/env python3

# Write a method which will remove any given character from a String
# Solved

def removeChar(text, char):
    newString = ""

    for i in text:
        if char != i:
            newString += i

    return newString


print(removeChar("This is a test", 'i'))
