export type Result<T, E> =
  | { value: T; error?: undefined; ok: true }
  | { value?: undefined; error: E; ok: false }

export const Result = {
  Ok<T, E>(value: T): Result<T, E> {
    return {
      ok: true,
      value,
    }
  },
  Err<T,E>(error: E): Result<T, E> {
    return {
        ok: false,
        error
    }
  }
}
