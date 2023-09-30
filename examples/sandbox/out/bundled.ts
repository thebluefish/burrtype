export type Bar = [Foo]

/**  The #[burr] attribute allows us to auto-include this type
 Later, we may support configuring target modules and the like via this attribute */
export interface Foo {
  one: number,
  two: string,
}
