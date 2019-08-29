#!/usr/bin/env python3

# How to check if two String are anagram?
# Solved

# This solution is BigO(n * m), where n and m are the lengths of the input strings

def stringsAreAnagrams(stringOne, stringTwo):
    if len(stringOne) != len(stringTwo):
        return False

    stringOne = stringOne.lower()
    stringTwo = stringTwo.lower()

    for i in stringOne:
        matchingLetterFound = False

        for j in stringTwo:
            if i == j:
                matchingLetterFound = True
                break

        if not matchingLetterFound:
            return False

    return True

# Should return true
print(stringsAreAnagrams("dog", "god"))
print(stringsAreAnagrams("listen", "silent"))
print(stringsAreAnagrams("elvis", "lives"))

# Should return false
print(stringsAreAnagrams("dog", "car"))
print(stringsAreAnagrams("paper", "rock"))
print(stringsAreAnagrams("laugh", "silly"))
