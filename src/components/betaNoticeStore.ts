// src/components/betaNoticeStore.ts
// Tiny persistence shim for the first-run beta notice (issue #12). Kept separate
// from the component so the localStorage access is easy to mock in tests and the
// component stays presentational. Dismissal is a one-time, best-effort flag.

const storageName = "civicnews.betaNoticeDismissed";
const dismissedValue = "1";

export function betaNoticeIsDismissed(): boolean {
  try {
    return window.localStorage.getItem(storageName) === dismissedValue;
  } catch {
    // localStorage may be unavailable (locked-down env); fail open so the
    // notice still shows rather than being silently suppressed.
    return false;
  }
}

export function betaNoticeMarkDismissed(): void {
  try {
    window.localStorage.setItem(storageName, dismissedValue);
  } catch {
    // Persistence is best-effort; dismissal still applies for this session.
  }
}
