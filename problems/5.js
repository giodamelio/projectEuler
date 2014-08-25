/*
    Problem #5

    2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

    What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
*/

// A function to find the greatest common divisor of two numbers
var greatestCommenDivisor = function(a, b) {
    if (b) {
        return greatestCommenDivisor(b, a % b);
    } else {
        return Math.abs(a);
    }
};

// A function to find the least common multible of two numbers
var leastCommonMultiple = function(a, b) {
    return (a * b) / greatestCommenDivisor(a, b);
};

var max = 20;
var n = 1;
for(var i = 1; i <= max; i++) {
    n = leastCommonMultiple(n, i);
}
console.log(n);
