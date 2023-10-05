/** A unit struct has no shape nor fields */
export type UnitStruct = null

export type Bar = Foo

/** An enum's variants correlate with struct variants */
export type Enum =
  /** A struct variant is defined by braces and fields with named */
  | { Struct: { /** An inline comment */ foo: Foo, bar: string }}
  | { TinyTuple: string }
  /** A tuple variant is defined by parenthesis and only types */
  | { Tuple: [/** Give some meaning to these nameless types */ Things, NumberedThings] }
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

/** A tuple struct is defined by parenthesis and only types */
export type TupleStruct = [number, Foo]

export interface NamedStruct {
  /** Type alias allows us to treat one type like another
Here we treat a newtype like its known inner type */
  foo: number,
  /** Rust reserved keywords should resolve properly for other languages */
  type: number,
  /** We need to support optional fields, too */
  opt?: Foo,
}

/** We can assign a module at the type level */
/** Why do we care about such things */
export type DeepTupleStruct = number

/** A named struct is defined by braces and fields with named */
export interface Foo {
  /** comments work at all levels */
  one: number,
  two: string,
}

/** The simplest enum of all unit types */
export type Things =
  | "ThingOne"
  | "ThingTwo"
;

/** Discriminant enum variants */
export type NumberedThings =
  | 1
  | 2
;
