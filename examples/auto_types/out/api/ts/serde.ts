import { DeepTupleStruct } from './inner/core'
import { TupleStruct } from './inner/index'
import { Stuff } from './core/serde'
import { Foo } from './common'

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
export type InternallyTaggedEnum =
  | { type: "Struct", foo: Foo, bar: string }
  | { type: "Unit" }
  | {
      type: "BigStruct",
      one: number,
      two: string,
      /** It doesn't matter where types are, we can reference them */
      THREE: DeepTupleStruct,
      FOUR?: RenamedStruct,
      six: TupleStruct,
    }
;

export interface RenamedStruct {
  FOO: Stuff,
  optional?: Foo,
  one: number,
  two: string,
}

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
