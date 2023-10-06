import { DeepTupleStruct } from './deep'
import { Things, Foo } from './common'
import { TupleStruct, RenamedStruct } from './types'

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
