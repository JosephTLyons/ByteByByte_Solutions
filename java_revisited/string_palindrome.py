#!/usr/bin/env python3

# Write code to check a String is palindrome or not
# Solved

def checkIfPalindrome(text):
    text = text.lower()
    textLength = len(text)
    textLengthToCheck = int(textLength / 2)

    for i in range(0, textLengthToCheck):
        if text[i] != text[textLength - i - 1]:
            return False

    return True

# Should return true
print(checkIfPalindrome("racecar"))
print(checkIfPalindrome("tattarrattat"))
print(checkIfPalindrome("MalayalAm"))

# Should return false
print(checkIfPalindrome("face"))
print(checkIfPalindrome("boy"))
print(checkIfPalindrome("mountain"))
print(checkIfPalindrome("rick james"))
