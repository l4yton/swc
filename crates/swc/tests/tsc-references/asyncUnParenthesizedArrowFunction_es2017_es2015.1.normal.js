function asyncGeneratorStep(gen, resolve, reject, _next, _throw, key, arg) {
    try {
        var info = gen[key](arg);
        var value = info.value;
    } catch (error) {
        reject(error);
        return;
    }
    if (info.done) {
        resolve(value);
    } else {
        Promise.resolve(value).then(_next, _throw);
    }
}
function _asyncToGenerator(fn) {
    return function() {
        var self = this, args = arguments;
        return new Promise(function(resolve, reject) {
            var gen = fn.apply(self, args);
            function _next(value) {
                asyncGeneratorStep(gen, resolve, reject, _next, _throw, "next", value);
            }
            function _throw(err) {
                asyncGeneratorStep(gen, resolve, reject, _next, _throw, "throw", err);
            }
            _next(undefined);
        });
    };
}
const x = function() {
    var _ref = _asyncToGenerator(function*(i) {
        return yield someOtherFunction(i);
    });
    return function x(i) {
        return _ref.apply(this, arguments);
    };
}();
const x1 = function() {
    var _ref = _asyncToGenerator(function*(i) {
        return yield someOtherFunction(i);
    });
    return function x1(i) {
        return _ref.apply(this, arguments);
    };
}();
