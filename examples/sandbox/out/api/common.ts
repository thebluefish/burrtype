/**  The #[burr] attribute allows us to auto-include this type
 Later, we may support configuring target modules and the like via this attribute */
export interface Foo {
  one: number,
  two: string,
}

/**  A tuple struct is defined by parenthesis and only types */
export type TupleStruct = [number, Foo]

/**  strike the earth! */
export type DeepTupleStruct = [/**  Why do we care about such things */ number]

/**  A named struct is defined by braces and fields with named */
export interface NamedStruct {
  /**  Builtin types are supported and usually converted to primitives */
  foo: number,
  /**  Types can be referenced from anywhere, so long as they're Reflect
 Type overrides can bypass the requirement for Reflect, but are per-language features */
  bar: Foo,
}
