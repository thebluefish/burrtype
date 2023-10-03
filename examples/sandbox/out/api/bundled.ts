/** An enum's variants correlate with struct variants */
export type Enum =
  /** A struct variant is defined by braces and fields with named */
  | { Struct: { /** An inline comment */ foo: Foo, bar: string }}
  /** A tuple variant is defined by parenthesis and only types */
  | { Tuple: [/** Give some meaning to these nameless types */ number, number] }
  /** A unit variant has no shape nor fields */
  | "Unit"
  /** Bigger structs can expand to a better format */
  | { BigStruct: {
      /** It doesn't matter where types are, we can reference them */
      one: DeepTupleStruct,
      two?: NamedStruct,
      three: TupleStruct,
      four: Foo,
    } }
;

export type Bar = Foo

/** A unit struct has no shape nor fields */
export type UnitStruct = null

export interface NamedStruct {
  /** Type alias allows us to treat one type like another
This is useful if a type doesn't derive Burr */
  foo: number,
  /** We need to support optional fields, too */
  bar?: Foo,
}

/** A tuple struct is defined by parenthesis and only types */
export type TupleStruct = [number, Foo]

/** A named struct is defined by braces and fields with named */
export interface Foo {
  /** comments work at all levels */
  one: number,
  two: string,
}

/** strike the earth! */
/** Why do we care about such things */
export type DeepTupleStruct = number
