export type Result<T, E> =
  | { value: T; error?: undefined; ok: true }
  | { value?: undefined; error: E; ok: false }

export const Result = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  Ok<T, E = any>(value: T): Result<T, E> {
    return {
      ok: true,
      value,
    }
  },
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  Err<E, T = any>(error: E): Result<T, E> {
    return {
        ok: false,
        error
    }
  }
}
