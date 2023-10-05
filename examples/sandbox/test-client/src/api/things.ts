import { TupleStruct, RenamedStruct } from './types'
import { Things, Foo } from './common'
import { DeepTupleStruct } from './deep'

export type Bar = Foo

/** An enum's variants correlate with struct variants */
export type Enum =
  /** A struct variant is defined by braces and fields with named */
  | { Struct: { /** An inline comment */ foo: Foo, bar: string }}
  | { TinyTuple: string }
  /** A tuple variant is defined by parenthesis and only types */
  | { Tuple: [/** Give some meaning to these nameless types */ Things, Things] }
  /** A unit variant has no shape nor fields */
  | "Unit"
  /** Bigger structs can expand to a better format */
  | { BigStruct: {
      /** It doesn't matter where types are, we can reference them */
      one: DeepTupleStruct,
      two?: RenamedStruct,
      three: TupleStruct,
      four: Foo,
    } }
;

/** A unit struct has no shape nor fields */
export type UnitStruct = null
