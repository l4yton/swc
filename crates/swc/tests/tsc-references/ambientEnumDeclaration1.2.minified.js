//// [ambientEnumDeclaration1.ts]
// In ambient enum declarations, all values specified in enum member declarations must be classified as constant enum expressions.
var E, E1;
(E1 = E || (E = {}))[E1.a = 10] = "a", E1[E1.b = 11] = "b", E1[E1.c = 11] = "c", E1[E1.d = 12] = "d", E1[E1.e = 655360] = "e";
