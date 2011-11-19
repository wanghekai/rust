// -*- rust -*-

/*
Module: tri

ADT for the ternary Kleene logic K3

This allows reasoning with three logic values (true, false, unknown)

Implementation: Truth values are represented using a single u8 and
all operations are done using bitshifting which is fast
on current cpus.
*/

export t, true, false, unknown;
export not, and, or, xor, implies, eq, ne, is_true, is_false;
export from_str, to_str, all_values, to_bit;

/*
Type: t

The type of ternary logic values
*/
type t = u8;

const b0: u8  = 1u8;
const b1: u8  = 2u8;
const b01: u8 = 3u8;

/*
Constant: unknown

Logic value for unknown (maybe true xor maybe false)
*/
const unknown: t = 0u8;

/*
Constant: true

Logic value for truth
*/
const true: t = 1u8;

/*
Constant: false

Logic value for falsehood
*/
const false: t = 2u8;

/* Function: not

Negation/Inverse
*/
pure fn not(v: t) -> t { ((v << 1u8) | (v >> 1u8)) & b01 }

/* Function: and

Conjunction
*/
pure fn and(a: t, b: t) -> t { ((a | b) & b1) | ((a & b) & b0) }

/* Function: or

Disjunction
*/
pure fn or(a: t, b: t) -> t { ((a & b) & b1) | ((a | b) & b0) }

/*
Function: xor

Exclusive or
*/
pure fn xor(a: t, b: t) -> t {
    let anb = a & b;
    let aob = a & not(b);
    ret ((anb & b1) | (anb << 1u8) | (aob >> 1u8) | (aob & b0)) & b01;
}

/*
Function: implies

Classic implication, i.e. from `a` follows `b`
*/
pure fn implies(a: t, b: t) -> t {
    ret ((a & b1) >> 1u8) | (b & b0) | ((a << 1u8) & b & b1);
}

/*
Predicate: eq

Returns:

true if truth values `a` and `b` are indistinguishable in the logic
*/
pure fn eq(a: t, b: t) -> bool {  a == b }

/*
Predicate: ne

Returns:

true if truth values `a` and `b` are distinguishable in the logic
*/
pure fn ne(a: t, b: t) -> bool { a != b }

/*
Predicate: is_true

Returns:

true if `v` represents truth in the logic
*/
pure fn is_true(v: t) -> bool { v == tri::true }

/*
Predicate: is_false

Returns:

true if `v` represents false in the logic
*/
pure fn is_false(v: t) -> bool { v == tri::false }

/*
Predicate: is_unknown

Returns:

true if `v` represents the unknown state in the logic
*/
pure fn is_unknown(v: t) -> bool { v == unknown }

/*
Function: from_str

Parse logic value from `s`
*/
pure fn from_str(s: str) -> t {
    alt s {
      "unknown" { unknown }
      "true" { tri::true }
      "false" { tri::false }
    }
}

/*
Function: to_str

Convert `v` into a string
*/
pure fn to_str(v: t) -> str {
    // FIXME replace with consts as soon as that works
    alt v {
      0u8 { "unknown" }
      1u8 { "true" }
      2u8 { "false" }
    }
}

/*
Function: all_values

Iterates over all truth values by passing them to `blk`
in an unspecified order
*/
fn all_values(blk: block(v: t)) {
    blk(tri::false);
    blk(unknown);
    blk(tri::true);
}

/*
Function: to_bit

Returns:

An u8 whose first bit is set if `if_true(v)` holds
*/
fn to_bit(v: t) -> u8 { v & b0 }

// Local Variables:
// mode: rust;
// fill-column: 78;
// indent-tabs-mode: nil
// c-basic-offset: 4
// buffer-file-coding-system: utf-8-unix
// End:
