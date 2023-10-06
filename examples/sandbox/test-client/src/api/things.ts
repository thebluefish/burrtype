import { NamedStruct, TupleStruct } from './types'
import { Foo, Stuff, Things } from './common'
import { DeepTupleStruct } from './deep'

export type Bar = Foo

/** An enum's variants correlate with struct variants */
export type Enum =
  /** A struct variant is defined by braces and fields with named */
  | { Struct: { /** An inline comment */ foo: Foo, bar: string }}
  | { TinyTuple: string }
  /** A tuple variant is defined by parenthesis and only types */
  | { Tuple: [/** Comments give meaning to these nameless types */ Things, Things] }
  /** A unit variant has no shape nor fields */
  | "Unit"
  /** Bigger structs can expand to a better format */
  | { BigStruct: {
      one: Foo,
      /** It doesn't matter where types are, we can reference them */
      three: DeepTupleStruct,
      four?: NamedStruct,
      five: TupleStruct,
    }}
;

/** A unit struct has no shape nor fields */
export type UnitStruct = null

export interface RenamedStruct {
  FOO: Stuff,
  optional?: Foo,
  /** comments work at all levels
Even below when this field is substituted in using #[serde(flatten)] */
  one: number,
  two: string,
}

/** An enum's variants correlate with struct variants */
export type InternallyTaggedEnum =
  | { type: "Struct", foo: Foo, bar: string }
  | { type: "Unit" }
  | {
      type: "BigStruct",
      /** comments work at all levels
Even below when this field is substituted in using #[serde(flatten)] */
      one: number,
      two: string,
      /** It doesn't matter where types are, we can reference them */
      THREE: DeepTupleStruct,
      FOUR?: RenamedStruct,
      six: TupleStruct,
    }
;

/** An enum's variants correlate with struct variants */
export type AdjacentlyTaggedEnum =
  | { t: "Struct", c: { foo: Foo, bar: string } }
  | { t: "TinyTuple", c: string }
  | {
      t: "Tuple",
      c: [Stuff, Stuff],
    }
  | { t: "Unit" }
  | {
      t: "BigStruct",
      c: {
        THREE: DeepTupleStruct,
        FOUR?: RenamedStruct,
        six: TupleStruct,
      }
    }
;

/** An enum's variants correlate with struct variants */
export type UntaggedEnum =
  | { foo: Foo, bar: string }
  /** Unit variant will be a string, but the newtype below will also capture a string
In untagged enum representations, serde will attempt them top-to-bottom
So we place more specific cases before general ones */
  | "unit"
  | string
  | [Stuff, Stuff]
  /** Bigger structs can expand to a better format */
  | {
      THREE: DeepTupleStruct,
      FOUR?: RenamedStruct,
      six: TupleStruct,
    }
;
