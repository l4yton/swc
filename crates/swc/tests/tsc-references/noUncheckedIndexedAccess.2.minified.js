//// [noUncheckedIndexedAccess.ts]
var NumericEnum1, NumericEnum2, StringEnum1, NumericEnum11, NumericEnum21, StringEnum11;
(NumericEnum1 = NumericEnum11 || (NumericEnum11 = {}))[NumericEnum1.A = 0] = "A", NumericEnum1[NumericEnum1.B = 1] = "B", NumericEnum1[NumericEnum1.C = 2] = "C", (NumericEnum2 = NumericEnum21 || (NumericEnum21 = {}))[NumericEnum2.A = 0] = "A", NumericEnum2[NumericEnum2.B = 1] = "B", NumericEnum2[NumericEnum2.C = 2] = "C", (StringEnum1 = StringEnum11 || (StringEnum11 = {})).A = "Alpha", StringEnum1.B = "Beta", strMap.foo, strMap.bar, strMap[0], strMap[0], strMap[0], strMap[0], strMap.foo, strMap[NumericEnum11.A], strMap[NumericEnum21.A], strMap[StringEnum11.A], strMap[StringEnum11.A], strMap[NumericEnum11.A], strMap[NumericEnum21.A], strMap[null], strMap.foo, strMap.bar, // Writes don't allow 'undefined'; all should be errors
strMap.baz = void 0, strMap.qua = void 0, strMap[0] = void 0, strMap[null] = void 0, numMap[0], numMap[0], numMap[0], numMap[NumericEnum11.A], numMap[NumericEnum21.A], obj1.x, obj1.y, obj1.y, obj1.z, strMapUnion.foo, symbolMap[s], symbolMap[s] = void 0, nonEmptyStringArray[0], nonEmptyStringArray[1];
