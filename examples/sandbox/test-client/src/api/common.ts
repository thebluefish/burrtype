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
