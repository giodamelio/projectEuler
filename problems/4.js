/*
    Problem #4

    A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

    Find the largest palindrome made from the product of two 3-digit numbers.
*/

// A function to detirmin if a number is palindromic
var isPalindromic = function(n) {
    // Convert our number to string
    var str = n.toString();

    // Loop through our string comparing the first and last characters, then the second and second to last and so on
    var a = str.length - 1;
    var b = 0;
    while (a > b) {
        if (str.charAt(b++) !== str.charAt(a--)) {
            return false;
        }
    }
    return true;
};

// Find all the palindromic numbers that are a product of two three digit numbers and keep track of the largest one
var largest = 0;
for (var a = 1; a < 1000; a++) {
    for (var b = 1; b < 1000; b++) {
        var x = a * b;
        if (isPalindromic(x) && x > largest) {
            largest = x;
        }
    }
}
console.log(largest);
