#!/usr/bin/env python3

# Write code to check a String is palindrome or not
# Solved

def isPalindrome(text):
    text = text.lower()
    textLength = len(text)
    textLengthToCheck = int(textLength / 2)

    for i in range(0, textLengthToCheck):
        if text[i] != text[textLength - i - 1]:
            return False

    return True

# Should return true
print(isPalindrome("racecar"))
print(isPalindrome("tattarrattat"))
print(isPalindrome("MalayalAm"))

# Should return false
print(isPalindrome("face"))
print(isPalindrome("boy"))
print(isPalindrome("mountain"))
print(isPalindrome("rick james"))
