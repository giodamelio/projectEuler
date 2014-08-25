/* jshint evil:true */
var app = angular.module("projectEuler", ["ui.codemirror"]);

// Override console.log to print to output pane
(function () {
    var old = console.log;
    var logger = document.getElementById("log");
    console.log = function() {
        // Turn arguments into an array
        var args = Array.prototype.slice.call(arguments);

        // Log it to the console
        old.apply(this, args);

        // Log it to the output pane
        var message = args.map(function(arg) {
            if (typeof arg === "object") {
                return JSON.stringify(arg, null, "    ") + "\n";
            } else {
                return arg + "\n";
            }
        });
        message.unshift("<pre>");
        message.push("</pre>");
        logger.innerHTML += message.join("");
    
        // Scroll the output pane
        logger.scrollTop = logger.scrollHeight;
    };
})();

// Get our probems
app.factory("Problems", ["$http", function($http) {
    return {
        getCompletedProblems: function(callback) {
            $http.get("problems/completed.txt")
                .success(function(data) {
                    callback(data.trim().split("\n"));
                });
        },
        getProblem: function(number, callback) {
            $http.get("problems/" + number + ".js")
                .success(function(data) {
                    callback(data);
                });
        }
    };
}]);

// Format the problem names nicely
app.filter("formatName", function() {
    return function(input) {
        return "Problem #" + input;
    };
});

// Script controller
app.controller("ScriptController", ["$scope", "Problems", function($scope, Problems) {
    // Our codemirror options
    $scope.editorOptions = {
        theme: "monokai",
        mode: "javascript",
        lineWrapping : true,
        lineNumbers: true,
        indentUnit: 4
    };

    // Run the current code
    $scope.run = function() {
        eval($scope.code);
    };
    
    // Get completed problems
    $scope.problems = [];
    Problems.getCompletedProblems(function(problems) {
        $scope.problems = problems;

        // Select to the location.hash or problem #1 by default
        if (location.hash === "") {
            $scope.setSelectedProblem(1);
        } else {
            $scope.setSelectedProblem(location.hash.slice(1));
        }
    });

    // Set selected problem
    $scope.setSelectedProblem = function(number) {
        // Set the select box
        $scope.problemNumber = $scope.problems[$scope.problems.indexOf(number.toString())];

        // Set the editor contents
        Problems.getProblem(number, function(script) {
            $scope.code = script;
        });

        // Clear the log
        document.getElementById("log").innerHTML = "";

        // Set the hash
        location.hash = number;
    };

    // Change the problem when the hash changes
    window.onhashchange = function() {
        $scope.setSelectedProblem(location.hash.slice(1));
    };

    // Change the problem when the select is changes
    $scope.changeProblem = function() {
        $scope.setSelectedProblem($scope.problemNumber);
    };

    $scope.refresh = function() {
        $scope.setSelectedProblem($scope.problemNumber);
    };
}]);

