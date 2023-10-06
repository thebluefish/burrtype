import { DeepTupleStruct, Things } from './core'
import { Foo } from '../common'

export type Enum =
  | { Struct: { foo: Foo, bar: string }}
  | { TinyTuple: string }
  | { Tuple: [Things, Things] }
  | "Unit"
  | { BigStruct: {
      one: Foo,
      three: DeepTupleStruct,
      four?: NamedStruct,
      five: TupleStruct,
    }}
;

export interface NamedStruct {
  foo: number,
  ty: number,
  opt?: Foo,
}

export type TupleStruct = [number, Foo]

export type UnitStruct = null
