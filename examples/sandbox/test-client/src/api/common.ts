export interface NamedStruct {
  /** Type alias allows us to treat one type like another
Here we treat a newtype like its known inner type */
  foo: number,
  /** Here we treat a third-party type by its known serde representation */
  bar: number,
  /** We need to support optional fields, too */
  opt?: Foo,
}

/** A named struct is defined by braces and fields with named */
export interface Foo {
  /** comments work at all levels */
  one: number,
  two: string,
}

/** strike the earth! */
/** Why do we care about such things */
export type DeepTupleStruct = number

/** A tuple struct is defined by parenthesis and only types */
export type TupleStruct = [number, Foo]
