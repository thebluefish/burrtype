import { DeepTupleStruct, NamedStruct, TupleStruct, Foo } from '../../common'

/**  An enum's variants correlate with struct variants */
export type Enum =
  /**  A struct variant is defined by braces and fields with named */
  | { /**  An inline comment */ "foo": Foo, "bar": string }
  /**  A tuple variant is defined by parenthesis and only types */
  | [/**  Give some meaning to these nameless types */ number, number]
  /**  A unit variant has no shape nor fields */
  | "Unit"
  /**  Bigger structs can expand to a better format */
  | {
      /**  It doesn't matter where types are, we can reference them */
      "one": DeepTupleStruct,
      "two": NamedStruct,
      "three": TupleStruct,
      "four": Foo,
    }
;

/**  A unit struct has no shape nor fields */
export interface UnitStruct {}
