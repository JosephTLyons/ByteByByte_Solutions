#!/usr/bin/env python3

# Given a string s consists of upper/lower-case alphabets and empty space characters ' ', return the
# length of last word in the string.
#
# If the last word does not exist, return 0.
#
# Please make sure you try to solve this problem without using library functions. Make sure you only
# traverse the string once.

def lengthOfLastWord(text):
    lengthOfWord = 0;

    for i in text:
        if i == ' ':
            lengthOfWord = 0
        else:
            lengthOfWord += 1

    return lengthOfWord

print(lengthOfLastWord("Hello World")) # 5
print(lengthOfLastWord("Hello ")) # 0
print(lengthOfLastWord("Hello every single person on the planet!")) # 7
