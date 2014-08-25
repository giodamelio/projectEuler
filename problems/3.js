/*
    Problem #3

    The prime factors of 13195 are 5, 7, 13 and 29.

    What is the largest prime factor of the number 600851475143 ?
*/

// Function to get all prime factors
var getPrimeFactors = function(n) {
    var root = Math.sqrt(n);
    // Get unnamed argument from recursive calls
    var result = arguments[1] || [];
    var x = 2; 

    if(n % x){ 
        x = 3;
        while((n % x) && ((x = x + 2) < root)){}
    }
    x = (x <= root) ? x: n;
    result.push(x);
    return (x === n) ? result: getPrimeFactors(n/x, result);
};

console.log(Math.max.apply(null, getPrimeFactors(600851475143)));
