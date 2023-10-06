/** A unit struct has no shape nor fields */
export type UnitStruct = null

export type Bar = Foo

/** An enum's variants correlate with struct variants */
export type Enum =
  /** A struct variant is defined by braces and fields with named */
  | { struct: { /** An inline comment */ foo: Foo, bar: string }}
  /** Bigger structs can expand to a better format */
  | { big_struct: {
      /** It doesn't matter where types are, we can reference them */
      THREE: DeepTupleStruct,
      FOUR?: RenamedStruct,
      six: TupleStruct,
    }}
;

/** We can assign a module at the type level */
/** Why do we care about such things */
export type DeepTupleStruct = number

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

/** A tuple struct is defined by parenthesis and only types */
export type TupleStruct = [number, Foo]

/** A named struct is defined by braces and fields with named */
export interface Foo {
  /** comments work at all levels
Even below when this field is substituted in using #[serde(flatten)] */
  one: number,
  two: string,
}
