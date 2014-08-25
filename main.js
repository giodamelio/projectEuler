var app = angular.module("projectEuler", ["ui.codemirror"]);

// Script controller
app.controller("ScriptController", ["$scope", function($scope) {
    $scope.editorOptions = {
        theme: "monokai",
        mode: "javascript",
        lineWrapping : true,
        lineNumbers: true,
        indentUnit: 4
    };
}]);

