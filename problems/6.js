/*
    Problem #6

    The sum of the squares of the first ten natural numbers is,

            1 ^ 2 + 2 ^ 2 + ... + 10 ^ 2 = 385

    The square of the sum of the first ten natural numbers is,

            (1 + 2 + ... + 10) ^ 2 = 552 = 3025

    Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.

    Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
*/

// Create an array of numbers, similar to pythons range()
var range = function(start, stop, step){
    if (typeof stop === "undefined"){
        // one param defined
        stop = start;
        start = 0;
    }
    if (typeof step === "undefined"){
        step = 1;
    }
    if ((step > 0 && start >= stop) || (step < 0 && start <= stop)){
        return [];
    }
    var result = [];
    for (var i = start; step > 0 ? i < stop: i > stop; i += step){
        result.push(i);
    }
    return result;
};

// Get the sum of squares
var sumOfSquares =
    // Create an array of numbers
    range(1, 101)
    // Square them
    .map(function(num) {
        return Math.pow(num, 2);
    })
    // Sum the list
    .reduce(function(prev, current) {
        return prev + current;
    });

// Get the square of sum
var squareOfSum = Math.pow(
    // Create an array of numbers
    range(1, 101)
    // Sum them'
    .reduce(function(prev, current) {
        return prev + current;
    }), 2);

console.log(squareOfSum - sumOfSquares);
