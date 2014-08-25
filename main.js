/* jshint evil:true */
var app = angular.module("projectEuler", ["ui.codemirror"]);

// Override console.log to print to output pane
(function () {
    var old = console.log;
    var logger = document.getElementById("log");
    console.log = function (message) {
        // Log it to the console
        old.apply(this, arguments);

        // Log it to the outpur pane
        if (typeof message === "string" || typeof message === "number") {
            logger.innerHTML += "<pre>" + message + "</pre>";
        } else if (typeof message === "object") {
            logger.innerHTML += "<pre>" + JSON.stringify(message, null, "    ") + "</pre>";
        }

        // Scroll the output pane
        logger.scrollTop = logger.scrollHeight;
    };
})();

// Script controller
app.controller("ScriptController", ["$scope", function($scope) {
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
}]);

