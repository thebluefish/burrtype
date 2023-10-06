/** A unit struct has no shape nor fields */
export type UnitStruct = null

/** An enum's variants correlate with struct variants */
export type Enum =
  /** A struct variant is defined by braces and fields with named */
  | { struct: { /** An inline comment */ foo: Foo, bar: string }}
  | { tiny_tuple: string }
  /** A tuple variant is defined by parenthesis and only types */
  | { tuple: [/** Comments give meaning to these nameless types */ Things, Things] }
  /** A unit variant has no shape nor fields */
  | "unit"
  /** Bigger structs can expand to a better format */
  | { big_struct: {
      /** It doesn't matter where types are, we can reference them */
      THREE: DeepTupleStruct,
      FOUR?: RenamedStruct,
      six: TupleStruct,
    }}
;

export type Bar = Foo

/** The simplest enum of all unit types */
export type Things =
  | "one"
  | "two"
;

/** A named struct is defined by braces and fields with named */
export interface Foo {
  /** comments work at all levels
Even below when this field is substituted in using #[serde(flatten)] */
  one: number,
  two: string,
}

/** We can assign a module at the type level */
/** Why do we care about such things */
export type DeepTupleStruct = number

/** A tuple struct is defined by parenthesis and only types */
export type TupleStruct = [number, Foo]

export interface RenamedStruct {
  /** Type alias allows us to treat one type like another
Here we treat a newtype like its known inner type */
  FOO: number,
  /** Rust reserved keywords should resolve properly for other languages */
  ty: number,
  /** We need to support optional fields, too */
  opt?: Foo,
  /** comments work at all levels
Even below when this field is substituted in using #[serde(flatten)] */
  one: number,
  two: string,
}
