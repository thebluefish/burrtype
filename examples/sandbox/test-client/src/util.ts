import { deepEqual } from 'fast-equals'

// Helper function to run tests
// Asserts if the two results aren't equal
export function assert_eq<T>(first: T, second: T) {
    console.assert(deepEqual(first, second), ":\n", first, "\n!=\n", second)
}