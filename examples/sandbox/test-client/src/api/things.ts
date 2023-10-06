import { Foo } from './common'
import { DeepTupleStruct } from './deep'
import { RenamedStruct, TupleStruct } from './types'

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
