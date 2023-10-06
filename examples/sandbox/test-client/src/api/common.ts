/** A named struct is defined by braces and fields with named */
export interface Foo {
  /** comments work at all levels
Even below when this field is substituted in using #[serde(flatten)] */
  one: number,
  two: string,
}

export type Stuff =
  | "red"
  | "two"
;

/** The simplest enum of all unit types */
export type Things =
  | "One"
  | "Two"
;
