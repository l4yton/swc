//// [override2.ts]
import { _ as _class_call_check } from "@swc/helpers/_/_class_call_check";
import { _ as _inherits } from "@swc/helpers/_/_inherits";
import { _ as _create_super } from "@swc/helpers/_/_create_super";
var AB = function AB() {
    "use strict";
    _class_call_check(this, AB);
};
var AD1 = /*#__PURE__*/ function(AB) {
    "use strict";
    _inherits(AD1, AB);
    var _super = _create_super(AD1);
    function AD1() {
        _class_call_check(this, AD1);
        return _super.apply(this, arguments);
    }
    return AD1;
}(AB);
var AD2 = /*#__PURE__*/ function(AB) {
    "use strict";
    _inherits(AD2, AB);
    var _super = _create_super(AD2);
    function AD2() {
        _class_call_check(this, AD2);
        return _super.apply(this, arguments);
    }
    return AD2;
}(AB);
var AD3 = /*#__PURE__*/ function(AB) {
    "use strict";
    _inherits(AD3, AB);
    var _super = _create_super(AD3);
    function AD3() {
        _class_call_check(this, AD3);
        return _super.apply(this, arguments);
    }
    var _proto = AD3.prototype;
    _proto.foo = function foo(v) {} // need override?
    ;
    _proto.baz = function baz() {};
    return AD3;
}(AB);
var D4 = /*#__PURE__*/ function(AB) {
    "use strict";
    _inherits(D4, AB);
    var _super = _create_super(D4);
    function D4() {
        _class_call_check(this, D4);
        return _super.apply(this, arguments);
    }
    var _proto = D4.prototype;
    _proto.foo = function foo(v) {};
    _proto.bar = function bar(v) {};
    _proto.baz = function baz() {};
    return D4;
}(AB);
