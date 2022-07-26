use once_cell::sync::Lazy;
use preset_env_base::version::Version;
use swc_common::collections::AHashMap;

use crate::util::{DataMap, FeatureMap};

macro_rules! iterators {
    (ARRAY, $($s:expr),*) => {
        &[$($s,)* "es.array.iterator", "web.dom-collections.iterator"]
    };
    (ARRAY_WITH_TAG, $($s:expr),*) => {
        iterators!(ARRAY, $($s,)* "es.object.to-string")
    };
    (COMMON, $($s:expr),*) => {
        iterators!(ARRAY, $($s,)* "es.string.iterator")
    };
    (COMMON_WITH_TAG, $($s:expr),*) => {
        iterators!(COMMON, $($s,)* "es.object.to-string")
    };
    ($($s:expr),*) => {
        &[$($s,)* "esnext.iterator.constructor", "es.object.to-string"]
    };
}

macro_rules! promise {
    (WITH_ITERATOR, $($s:expr),*) => {
        iterators!(COMMON, $($s,)* "es.promise", "es.object.to-string")
    };
    ($($s:expr),*) => {
        &[$($s,)* "es.promise", "es.object.to-string"]
    };
}

macro_rules! async_iterators {
    // combined with iterator!(COMMON)
    (WITH_COMMON_ITERATOR_AND_METHOD, $($s:expr),*) => {
        async_iterators!(
            WITH_ITERATOR_AND_METHOD,
            $($s,)*
            "es.string.iterator",
            "es.array.iterator",
            "web.dom-collections.iterator"
        )
    };
    (WITH_ITERATOR_AND_METHOD, $($s:expr),*) => {
        async_iterators!(
            WITH_ITERATOR,
            $($s,)*
            "esnext.async-iterator.every",
            "esnext.async-iterator.filter",
            "esnext.async-iterator.find",
            "esnext.async-iterator.flat-map",
            "esnext.async-iterator.for-each",
            "esnext.async-iterator.map",
            "esnext.async-iterator.reduce",
            "esnext.async-iterator.some"
        )
    };
    // combined with iterator!()
    (WITH_ITERATOR, $($s:expr),*) => {
        async_iterators!($($s,)* "esnext.iterator.constructor", "es.object.to-string")
    };
    ($($s:expr),*) => {
        promise!($($s,)* "esnext.async-iterator.constructor")
    };
}

pub static COMMON_ITERATORS: &[&str] = iterators!(COMMON,);

static ARRAY_NATURE_ITERATORS_WITH_TAG: &[&str] = iterators!(ARRAY_WITH_TAG,);

static TYPED_ARRAY_STATIC_METHODS: FeatureMap = data_map!(Map {
    from: &["es.typed-array.from"],
    fromAsync: TYPED_FROM_ASYNC,
    of: &["es.typed-array.of"],
});

pub static PROMISE_DEPENDENCIES: &[&str] = &["es.promise", "es.object.to-string"];

macro_rules! map_dep {
    ($($s:expr),*) => {
        iterators!(COMMON_WITH_TAG, $($s,)*
            "es.map",
            "esnext.map.delete-all",
            "esnext.map.emplace",
            "esnext.map.every",
            "esnext.map.filter",
            "esnext.map.find",
            "esnext.map.find-key",
            "esnext.map.includes",
            "esnext.map.key-of",
            "esnext.map.map-keys",
            "esnext.map.map-values",
            "esnext.map.merge",
            "esnext.map.reduce",
            "esnext.map.some",
            "esnext.map.update"
        )
    };
}

macro_rules! set_dep {
    ($($s:expr),*) => {
        iterators!(COMMON_WITH_TAG, $($s,)*
            "es.set",
            "esnext.set.add-all",
            "esnext.set.delete-all",
            "esnext.set.difference",
            "esnext.set.every",
            "esnext.set.filter",
            "esnext.set.find",
            "esnext.set.intersection",
            "esnext.set.is-disjoint-from",
            "esnext.set.is-subset-of",
            "esnext.set.is-superset-of",
            "esnext.set.join",
            "esnext.set.map",
            "esnext.set.reduce",
            "esnext.set.some",
            "esnext.set.symmetric-difference",
            "esnext.set.union"
        )
    };
}

macro_rules! weak_map_dep {
    ($($s:expr),*) => {
        iterators!(COMMON_WITH_TAG, $($s,)*
            "es.weak-map",
            "esnext.weak-map.delete-all",
            "esnext.weak-map.emplace"
        )
    };
}

macro_rules! weak_set_dep {
    ($($s:expr),*) => {
        iterators!(COMMON_WITH_TAG, $($s,)*
            "es.weak-set",
            "esnext.weak-set.add-all",
            "esnext.weak-set.delete-all"
        )
    };
}

static ASYNC_ITERATOR: &[&str] = async_iterators!();
static ITERATOR: &[&str] = iterators!();

static TYPED_FROM_ASYNC: &[&str] = promise!(WITH_ITERATOR, "esnext.typed-array.from-async");

static PROMISE_DEPENDENCIES_WITH_ITERATORS: &[&str] = promise!(WITH_ITERATOR,);

static SYMBOL_DEPENDENCIES: &[&str] =
    &["es.symbol", "es.symbol.description", "es.object.to-string"];

static MAP_DEPENDENCIES: &[&str] = map_dep!();

static SET_DEPENDENCIES: &[&str] = set_dep!();

static WEAK_MAP_DEPENDENCIES: &[&str] = weak_map_dep!();

static WEAK_SET_DEPENDENCIES: &[&str] = weak_set_dep!();

static URL_SEARCH_PARAMS_DEPENDENCIES: &[&str] =
    iterators!(COMMON_WITH_TAG, "web.url-search-params");

pub static REGEXP_DEPENDENCIES: &[&str] = &["es.regexp.constructor"];

macro_rules! typed_array {
    ($e: expr) => {
        &[
            $e,
            "es.typed-array.copy-within",
            "es.typed-array.every",
            "es.typed-array.fill",
            "es.typed-array.filter",
            "es.typed-array.find",
            "es.typed-array.find-index",
            "es.typed-array.for-each",
            "es.typed-array.includes",
            "es.typed-array.index-of",
            "es.typed-array.iterator",
            "es.typed-array.join",
            "es.typed-array.last-index-of",
            "es.typed-array.map",
            "es.typed-array.reduce",
            "es.typed-array.reduce-right",
            "es.typed-array.reverse",
            "es.typed-array.set",
            "es.typed-array.slice",
            "es.typed-array.some",
            "es.typed-array.sort",
            "es.typed-array.subarray",
            "es.typed-array.to-locale-string",
            "es.typed-array.to-string",
            "es.object.to-string",
            "es.array.iterator",
            "es.array-buffer.slice",
            "esnext.typed-array.filter-reject",
            "esnext.typed-array.find-last",
            "esnext.typed-array.find-last-index",
            "esnext.typed-array.group-by",
            "esnext.typed-array.to-reversed",
            "esnext.typed-array.to-sorted",
            "esnext.typed-array.to-spliced",
            "esnext.typed-array.unique-by",
            "esnext.typed-array.with",
        ]
    };
}

static FLOAT32_ARRAY: &[&str] = typed_array!("es.typed-array.float32-array");
static FLOAT64_ARRAY: &[&str] = typed_array!("es.typed-array.float64-array");
static INT8_ARRAY: &[&str] = typed_array!("es.typed-array.int8-array");
static INT16_ARRAY: &[&str] = typed_array!("es.typed-array.int16-array");
static INT32_ARRAY: &[&str] = typed_array!("es.typed-array.int32-array");
static UINT8_ARRAY: &[&str] = typed_array!("es.typed-array.uint8-array");
static UINT8_CLAMPED_ARRAY: &[&str] = typed_array!("es.typed-array.uint8-clamped-array");
static UINT16_ARRAY: &[&str] = typed_array!("es.typed-array.uint16-array");
static UINT32_ARRAY: &[&str] = typed_array!("es.typed-array.uint32-array");

static OBSERVEABLE: &[&str] = iterators!(
    COMMON_WITH_TAG,
    "esnext.observable",
    "esnext.symbol.observable"
);

macro_rules! dom_exception_dep {
    ($($s:expr),*) => {
        &[
            $($s,)*
            "web.dom-exception.constructor",
            "web.dom-exception.stack",
            "web.dom-exception.to-string-tag",
            "es.error.to-string",
        ]
    };
}

static URL_DEP: &[&str] = iterators!(COMMON_WITH_TAG, "web.url", "web.url-search-params");

static DOM_EXCEPTION: &[&str] = dom_exception_dep!();
static ATOB: &[&str] = dom_exception_dep!("web.atob");
static BTOA: &[&str] = dom_exception_dep!("web.btoa");
static STRUCTURE_CLONE: &[&str] = dom_exception_dep!(
    "web.structured-clone",
    "es.array.iterator",
    "es.object.keys",
    "es.object.to-string",
    "es.map",
    "es.set"
);

static ERROR_DEP: &[&str] = &["es.error.cause", "es.error.to-string"];
static AGGREGATE_ERROR_DEP: &[&str] = iterators!(
    COMMON_WITH_TAG,
    "es.aggregate-error",
    "es.error.cause",
    "es.error.to-string",
    "es.aggregate-error.cause"
);

pub static BUILTINS: DataMap<&[&str]> = data_map!(Map {
    AsyncIterator: ASYNC_ITERATOR,
    AggregateError: AGGREGATE_ERROR_DEP,
    ArrayBuffer: [
        "es.array-buffer.constructor",
        "es.array-buffer.slice",
        "es.object.to-string",
    ],
    DataView: [
        "es.data-view",
        "es.array-buffer.slice",
        "es.object.to-string"
    ],
    Date: ["es.date.to-string"],
    DOMException: DOM_EXCEPTION,
    Error: ERROR_DEP,
    EvalError: ERROR_DEP,
    Iterator: ITERATOR,
    Float32Array: FLOAT32_ARRAY,
    Float64Array: FLOAT64_ARRAY,
    Int8Array: INT8_ARRAY,
    Int16Array: INT16_ARRAY,
    Int32Array: INT32_ARRAY,
    Uint8Array: UINT8_ARRAY,
    Uint8ClampedArray: UINT8_CLAMPED_ARRAY,
    Uint16Array: UINT16_ARRAY,
    Uint32Array: UINT32_ARRAY,
    Map: MAP_DEPENDENCIES,
    Number: ["es.number.constructor"],
    Observable: OBSERVEABLE,
    Promise: PROMISE_DEPENDENCIES,
    RangeError: ERROR_DEP,
    ReferenceError: ERROR_DEP,
    Reflect: ["es.reflect.to-string-tag", "es.object.to-string"],
    RegExp: [
        "es.regexp.constructor",
        "es.regexp.exec",
        "es.regexp.to-string"
    ],
    Set: SET_DEPENDENCIES,
    Symbol: SYMBOL_DEPENDENCIES,
    TypeError: ERROR_DEP,
    URIError: ERROR_DEP,
    URL: URL_DEP,
    URLSearchParams: URL_SEARCH_PARAMS_DEPENDENCIES,
    WeakMap: WEAK_MAP_DEPENDENCIES,
    WeakSet: WEAK_SET_DEPENDENCIES,

    atob: ATOB,
    btoa: BTOA,
    clearImmediate: ["web.immediate"],
    compositeKey: ["esnext.composite-key"],
    compositeSymbol: ["esnext.composite-symbol"],
    escape: ["es.escape"],
    fetch: PROMISE_DEPENDENCIES,
    globalThis: ["es.global-this"],
    parseFloat: ["es.parse-float"],
    parseInt: ["es.parse-int"],
    queueMicrotask: ["web.queue-microtask"],
    setTimeout: ["web.timers"],
    setInterval: ["web.timers"],
    setImmediate: ["web.immediate"],
    structuredClone: STRUCTURE_CLONE,
    unescape: ["es.unescape"],
});

static INDEXED_PAIRS: &[&str] = async_iterators!(
    WITH_ITERATOR,
    "esnext.async-iterator.as-indexed-pairs",
    "esnext.iterator.as-indexed-pairs"
);

static DROP: &[&str] = async_iterators!(
    WITH_ITERATOR,
    "esnext.async-iterator.drop",
    "esnext.iterator.drop"
);

static EVERY: &[&str] = iterators!(
    "es.array.every",
    "esnext.async-iterator.every",
    "esnext.iterator.every"
);

static FILTER: &[&str] = iterators!(
    "es.array.filter",
    "esnext.async-iterator.filter",
    "esnext.iterator.filter"
);

static FIND: &[&str] = iterators!(
    "es.array.find",
    "esnext.async-iterator.find",
    "esnext.iterator.find"
);

static FLAT_MAP: &[&str] = iterators!(
    "es.array.flat-map",
    "es.array.unscopables.flat-map",
    "esnext.async-iterator.flat-map",
    "esnext.iterator.flat-map"
);

static FOR_EACH: &[&str] = iterators!(
    "es.array.for-each",
    "esnext.async-iterator.for-each",
    "esnext.iterator.for-each",
    "web.dom-collections.for-each"
);

static REDUCE: &[&str] = iterators!(
    "es.array.reduce",
    "esnext.async-iterator.reduce",
    "esnext.iterator.reduce"
);

static SOME: &[&str] = iterators!(
    "es.array.some",
    "esnext.async-iterator.some",
    "esnext.iterator.some"
);

static TAKE: &[&str] = async_iterators!(
    WITH_ITERATOR,
    "esnext.async-iterator.take",
    "esnext.iterator.take"
);

static TO_ARRAY: &[&str] = async_iterators!(
    WITH_ITERATOR,
    "esnext.async-iterator.to-array",
    "esnext.iterator.to-array"
);

static TO_ASYNC: &[&str] = async_iterators!(WITH_ITERATOR_AND_METHOD, "esnext.iterator.to-async");

static PROMISE_FINALLY: &[&str] = promise!("es.promise.finally");

pub static INSTANCE_PROPERTIES: DataMap<&[&str]> = data_map!(Map {
    asIndexedPairs: INDEXED_PAIRS,
    // TODO: check type of variable
    at: ["es.string.at-alternative", "es.array.at"],
    anchor: ["es.string.anchor"],
    big: ["es.string.big"],
    bind: ["es.function.bind"],
    blink: ["es.string.blink"],
    bold: ["es.string.bold"],
    codePointAt: ["es.string.code-point-at"],
    codePoints: ["esnext.string.code-points"],
    concat: ["es.array.concat"],
    copyWithin: ["es.array.copy-within"],
    description: ["es.symbol", "es.symbol.description"],
    dotAll: [ "es.regexp.dot-all"],
    drop: DROP,
    emplace: ["esnext.map.emplace", "esnext.weak-map.emplace"],
    endsWith: ["es.string.ends-with"],
    entries: ARRAY_NATURE_ITERATORS_WITH_TAG,
    every: EVERY,
    exec: ["es.regexp.exec"],
    fill: ["es.array.fill"],
    filter: FILTER,
    filterReject: "esnext.array.filter-reject",
    finally: PROMISE_FINALLY,
    find: FIND,
    findIndex: ["es.array.find-index"],
    findLast: "esnext.array.find-last",
    fixed: ["es.string.fixed"],
    flags: ["es.regexp.flags"],
    flat: ["es.array.flat", "es.array.unscopables.flat"],
    flatMap: FLAT_MAP,
    fontcolor: ["es.string.fontcolor"],
    fontsize: ["es.string.fontsize"],
    forEach: FOR_EACH,
    getYear: "es.date.get-year",
    groupBy: "esnext.array.group-by",
    groupByToMap: ["esnext.array.group-by-to-map", "es.map", "es.object.to-string"],
    includes: ["es.array.includes", "es.string.includes"],
    indexOf: ["es.array.index-of"],
    italics: ["es.string.italics"],
    join: ["es.array.join"],
    keys: ARRAY_NATURE_ITERATORS_WITH_TAG,
    lastIndex: ["esnext.array.last-index"],
    lastIndexOf: ["es.array.last-index-of"],
    lastItem: ["esnext.array.last-item"],
    link: ["es.string.link"],
    match: ["es.string.match", "es.regexp.exec"],
    matchAll: ["es.string.match-all", "es.regexp.exec"],
    map: ["es.array.map"],
    name: ["es.function.name"],
    padEnd: ["es.string.pad-end"],
    padStart: ["es.string.pad-start"],
    reduce: REDUCE,
    reduceRight: ["es.array.reduce-right"],
    repeat: ["es.string.repeat"],
    replace: ["es.string.replace", "es.regexp.exec"],
    replaceAll: ["es.string.replace-all", "es.string.replace","es.regexp.exec"],
    reverse: ["es.array.reverse"],
    search: ["es.string.search", "es.regexp.exec"],
    slice: ["es.array.slice"],
    small: ["es.string.small"],
    some: SOME,
    sort: ["es.array.sort"],
    splice: ["es.array.splice"],
    split: ["es.string.split", "es.regexp.exec"],
    startsWith: ["es.string.starts-with"],
    sticky:["es.regexp.sticky"],
    strike: ["es.string.strike"],
    sub: ["es.string.sub"],
    substr: ["es.string.substr"],
    sup: ["es.string.sup"],
    take: TAKE,
    test: ["es.regexp.test", "es.regexp.exec"],
    toArray: TO_ARRAY,
    toAsync: TO_ASYNC,
    toExponential: "es.number.to-exponential",
    toFixed: ["es.number.to-fixed"],
    toGMTString: "es.date.to-gmt-string",
    toISOString: ["es.date.to-iso-string"],
    toJSON: ["es.date.to-json", "web.url.to-json"],
    toPrecision: ["es.number.to-precision"],
    toString: [
        "es.object.to-string",
        "es.regexp.to-string",
        "es.date.to-string"
    ],
    toSorted: ["esnext.array.to-sorted", "es.array.sort"],
    toSpliced: "esnext.array.to-spliced",
    toString: ["es.object.to-string", "es.error.to-string", "es.date.to-string", "es.regexp.to-string"],
    trim: ["es.string.trim"],
    trimEnd: ["es.string.trim-end"],
    trimLeft: ["es.string.trim-start"],
    trimRight: ["es.string.trim-end"],
    trimStart: ["es.string.trim-start"],
    uniqueBy: ["esnext.array.unique-by", "es.map"],
    unThis: "esnext.function.un-this",
    values: ARRAY_NATURE_ITERATORS_WITH_TAG,
    with: "esnext.array.with",
    __defineGetter__: ["es.object.define-getter"],
    __defineSetter__: ["es.object.define-setter"],
    __lookupGetter__: ["es.object.lookup-getter"],
    __lookupSetter__: ["es.object.lookup-setter"],
});

static ASYNC_ITER_FROM: &[&str] = async_iterators!(
    WITH_COMMON_ITERATOR_AND_METHOD,
    "esnext.async-iterator.from"
);
static FROM_ASYNC: &[&str] = promise!(WITH_ITERATOR, "esnext.array.from-async");
static ALL_SETTLED: &[&str] = promise!(WITH_ITERATOR, "es.promise.all-settled");
static PROMISE_ANY: &[&str] = promise!(WITH_ITERATOR, "es.promise.any", "es.aggregate-error");
static PROMISE_TRY: &[&str] = promise!(WITH_ITERATOR, "esnext.promise.try");

static MAP_FROM: &[&str] = map_dep!("esnext.map.from");
static MAP_GROUP_BY: &[&str] = map_dep!("esnext.map.group-by");
static MAP_KEY_BY: &[&str] = map_dep!("esnext.map.key-by");
static MAP_OF: &[&str] = map_dep!("esnext.map.of");

static SET_FROM: &[&str] = set_dep!("esnext.set.from");
static SET_OF: &[&str] = set_dep!("esnext.set.of");

static WEAK_MAP_FROM: &[&str] = set_dep!("esnext.weak-map.from");
static WEAK_MAP_OF: &[&str] = set_dep!("esnext.weak-map.of");

static WEAK_SET_FROM: &[&str] = set_dep!("esnext.weak-set.from");
static WEAK_SET_OF: &[&str] = set_dep!("esnext.weak-set.of");

static SYMBOL_ITERATOR: &[&str] = iterators!(COMMON_WITH_TAG, "es.symbol.iterator");

pub static STATIC_PROPERTIES: DataMap<DataMap<&[&str]>> = data_map!(Map {
    AsyncIterator: Map {
        from: ASYNC_ITER_FROM,
    },
    Array: Map {
        from: ["es.array.from", "es.string.iterator"],
        fromAsync: FROM_ASYNC,
        isArray: ["es.array.is-array"],
        isTemplateObject: "esnext.array.is-template-object",
        of: ["es.array.of"],
    },

    ArrayBuffer: Map {
        isView: ["es.array-buffer.is-view"],
    },

    BigInt: Map {
        range: ["esnext.bigint.range", "es.object.to-string"],
    },

    Date: Map { now: "es.date.now" },

    Function: Map {
        isCallable: "esnext.function.is-callable",
        isConstructor: "esnext.function.is-constructor",
    },

    Object: Map {
        assign: "es.object.assign",
        create: "es.object.create",
        defineProperty: "es.object.define-property",
        defineProperties: "es.object.define-properties",
        entries: "es.object.entries",
        freeze: "es.object.freeze",
        fromEntries: ["es.object.from-entries", "es.array.iterator"],
        getOwnPropertyDescriptor: "es.object.get-own-property-descriptor",
        getOwnPropertyDescriptors: "es.object.get-own-property-descriptors",
        getOwnPropertyNames: "es.object.get-own-property-names",
        getOwnPropertySymbols: "es.symbol",
        getPrototypeOf: "es.object.get-prototype-of",
        hasOwn: "es.object.has-own",
        is: "es.object.is",
        isExtensible: "es.object.is-extensible",
        isFrozen: "es.object.is-frozen",
        isSealed: "es.object.is-sealed",
        keys: "es.object.keys",
        preventExtensions: "es.object.prevent-extensions",
        seal: "es.object.seal",
        setPrototypeOf: "es.object.set-prototype-of",
        values: "es.object.values",
    },

    Math: Map {
        DEG_PER_RAD: "esnext.math.deg-per-rad",
        RAD_PER_DEG: "esnext.math.rad-per-deg",
        acosh: "es.math.acosh",
        asinh: "es.math.asinh",
        atanh: "es.math.atanh",
        cbrt: "es.math.cbrt",
        clamp: "esnext.math.clamp",
        clz32: "es.math.clz32",
        cosh: "es.math.cosh",
        degrees: "esnext.math.degrees",
        expm1: "es.math.expm1",
        fround: "es.math.fround",
        fscale: "esnext.math.fscale",
        hypot: "es.math.hypot",
        iaddh: "esnext.math.iaddh",
        imul: "es.math.imul",
        imulh: "esnext.math.imulh",
        isubh: "esnext.math.isubh",
        log1p: "es.math.log1p",
        log10: "es.math.log10",
        log2: "es.math.log2",
        radians: "esnext.math.radians",
        scale: "esnext.math.scale",
        seededPRNG: "esnext.math.seeded-prng",
        sign: "es.math.sign",
        signbit: "esnext.math.signbit",
        sinh: "es.math.sinh",
        tanh: "es.math.tanh",
        trunc: "es.math.trunc",
        umulh: "esnext.math.umulh",
    },

    String: Map {
        cooked: "esnext.string.cooked",
        fromCodePoint: "es.string.from-code-point",
        raw: "es.string.raw",
    },

    Number: Map {
        EPSILON: "es.number.epsilon",
        MIN_SAFE_INTEGER: "es.number.min-safe-integer",
        MAX_SAFE_INTEGER: "es.number.max-safe-integer",
        fromString: "esnext.number.from-string",
        isFinite: "es.number.is-finite",
        isInteger: "es.number.is-integer",
        isSafeInteger: "es.number.is-safe-integer",
        isNaN: "es.number.is-nan",
        parseFloat: "es.number.parse-float",
        parseInt: "es.number.parse-int",
        range: [
            "esnext.number.range",
            "es.object.to-string",
        ],
    },

    Map: Map {
        from: MAP_FROM,
        groupBy: MAP_GROUP_BY,
        keyBy: MAP_KEY_BY,
        of: MAP_OF,
    },

    Set: Map {
        from: SET_FROM,
        of: SET_OF,
    },

    WeakMap: Map {
        from: WEAK_MAP_FROM,
        of: WEAK_MAP_OF,
    },

    WeakSet: Map {
        from: WEAK_SET_FROM,
        of: WEAK_SET_OF,
    },

    Promise: Map {
        all: PROMISE_DEPENDENCIES_WITH_ITERATORS,
        allSettled: ALL_SETTLED,
        any: PROMISE_ANY,
        race: PROMISE_DEPENDENCIES_WITH_ITERATORS,
        try: PROMISE_TRY,
    },

    Reflect: Map {
        apply: "es.reflect.apply",
        construct: "es.reflect.construct",
        defineMetadata: "esnext.reflect.define-metadata",
        defineProperty: "es.reflect.define-property",
        deleteMetadata: "esnext.reflect.delete-metadata",
        deleteProperty: "es.reflect.delete-property",
        get: "es.reflect.get",
        getMetadata: "esnext.reflect.get-metadata",
        getMetadataKeys: "esnext.reflect.get-metadata-keys",
        getOwnMetadata: "esnext.reflect.get-own-metadata",
        getOwnMetadataKeys: "esnext.reflect.get-own-metadata-keys",
        getOwnPropertyDescriptor: "es.reflect.get-own-property-descriptor",
        getPrototypeOf: "es.reflect.get-prototype-of",
        has: "es.reflect.has",
        hasMetadata: "esnext.reflect.has-metadata",
        hasOwnMetadata: "esnext.reflect.has-own-metadata",
        isExtensible: "es.reflect.is-extensible",
        metadata: "esnext.reflect.metadata",
        ownKeys: "es.reflect.own-keys",
        preventExtensions: "es.reflect.prevent-extensions",
        set: "es.reflect.set",
        setPrototypeOf: "es.reflect.set-prototype-of",
    },

    Symbol: Map {
        asyncDispose: ["es.symbol.async-dispose"],
        asyncIterator: ["es.symbol.async-iterator"],
        dispose: ["esnext.symbol.dispose"],
        hasInstance: ["es.symbol.has-instance", "es.function.has-instance"],
        isConcatSpreadable: ["es.symbol.is-concat-spreadable", "es.array.concat"],
        iterator: SYMBOL_ITERATOR,
        match: ["es.symbol.match", "es.string.match"],
        matcher: ["es.symbol.matcher"],
        match: ["es.symbol.match-all", "es.string.match-all"],
        metadata: "esnext.symbol.metadata",
        observable: ["esnext.symbol.observable"],
        patternMatch: ["esnext.symbol.pattern-match"],
        replace: ["es.symbol.replace", "es.string.replace"],
        search: ["es.symbol.search", "es.string.search"],
        species: ["es.symbol.species", "es.array.species"],
        split: ["es.symbol.split", "es.string.split"],
        toPrimitive: ["es.symbol.to-primitive", "es.date.to-primitive"],
        toStringTag: [
            "es.symbol.to-string-tag",
            "es.object.to-string",
            "es.math.to-string-tag",
            "es.json.to-string-tag",
        ],
        unscopables: ["es.symbol.unscopables"],
    },

    Int8Array: &TYPED_ARRAY_STATIC_METHODS,
    Uint8Array: &TYPED_ARRAY_STATIC_METHODS,
    Uint8ClampedArray: &TYPED_ARRAY_STATIC_METHODS,
    Int16Array: &TYPED_ARRAY_STATIC_METHODS,
    Uint16Array: &TYPED_ARRAY_STATIC_METHODS,
    Int32Array: &TYPED_ARRAY_STATIC_METHODS,
    Uint32Array: &TYPED_ARRAY_STATIC_METHODS,
    Float32Array: &TYPED_ARRAY_STATIC_METHODS,
    Float64Array: &TYPED_ARRAY_STATIC_METHODS,

    WebAssembly: Map {
        CompileError: ERROR_DEP,
        LinkError: ERROR_DEP,
        RuntimeError: ERROR_DEP,
    },
});

//pub static COMMON_INSTANCE_DEPENDENCIES: &[&str] = &[
//    "es.object.to-string",
//    "es.object.define-getter",
//    "es.object.define-setter",
//    "es.object.lookup-getter",
//    "es.object.lookup-setter",
//    "es.regexp.exec",
//];

pub static POSSIBLE_GLOBAL_OBJECTS: &[&str] = &["global", "globalThis", "self", "window"];

pub static MODULES_BY_VERSION: Lazy<AHashMap<&'static str, Version>> = Lazy::new(|| {
    serde_json::from_str::<AHashMap<_, _>>(include_str!(
        "../../data/core-js-compat/modules-by-versions.json"
    ))
    .expect("failed to parse modules-by-versions.json")
    .into_iter()
    .flat_map(|(k, v): (Version, Vec<String>)| {
        v.into_iter()
            .map(|s: String| (&*Box::leak(s.into_boxed_str()), k))
            .collect::<Vec<_>>()
    })
    .collect()
});
