/* jshint evil:true */
var app = angular.module("projectEuler", ["ui.codemirror"]);

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

