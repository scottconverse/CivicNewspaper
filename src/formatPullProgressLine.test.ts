// src/formatPullProgressLine.test.ts
import { describe, test, expect } from "vitest";
import { formatPullProgressLine } from "./useApp";

// TEST-002 / ENG-004: after consolidating the three pull commands down to the
// single structured, cancellable `pull_ollama_model`, the surviving
// `ollama-pull-progress` event carries an object payload ({model,status,
// completed,total}) — not the JSON string the removed `pull_model` emitted.
// These pin that shape so the listener can't silently regress to string parsing.
describe("formatPullProgressLine (surviving ollama-pull-progress payload shape)", () => {
  test("renders status and percent from a structured progress payload", () => {
    expect(
      formatPullProgressLine({ model: "gemma2:9b", status: "downloading", completed: 50, total: 100 })
    ).toBe("downloading (50%)");
  });

  test("renders status alone when no byte counts are present", () => {
    expect(formatPullProgressLine({ model: "gemma2:9b", status: "pulling manifest" })).toBe(
      "pulling manifest"
    );
  });

  test("falls back to a default label when status is missing", () => {
    expect(formatPullProgressLine({})).toBe("Downloading...");
  });

  test("omits the percent when total is zero rather than emitting NaN/Infinity", () => {
    expect(formatPullProgressLine({ status: "verifying", completed: 0, total: 0 })).toBe("verifying");
  });

  test("rounds the percent to a whole number", () => {
    expect(formatPullProgressLine({ status: "downloading", completed: 1, total: 3 })).toBe(
      "downloading (33%)"
    );
  });
});
