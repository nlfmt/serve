type ConditionalClass = [string, unknown]
/** Combines classes into a single string */
export function classes(...classNames: (string | ConditionalClass | undefined | null)[]) {
  return classNames.map(c => (
    Array.isArray(c)) ? c[1] ? c[0] : "" : c
  ).filter(Boolean).join(" ")
}