import { Foo } from './common'

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
