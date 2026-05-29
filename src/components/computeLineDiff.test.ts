// src/components/computeLineDiff.test.ts
import { describe, test, expect } from "vitest";
import { computeLineDiff } from "./Workbench";

describe("computeLineDiff", () => {
  test("identical input yields all 'same' rows on both sides", () => {
    const { left, right } = computeLineDiff("a\nb\nc", "a\nb\nc");
    const same = [
      { text: "a", type: "same" },
      { text: "b", type: "same" },
      { text: "c", type: "same" },
    ];
    expect(left).toEqual(same);
    expect(right).toEqual(same);
  });

  test("empty rewrite marks every original line as removed", () => {
    const { left, right } = computeLineDiff("a\nb", "");
    expect(left).toEqual([
      { text: "a", type: "removed" },
      { text: "b", type: "removed" },
    ]);
    // "".split("\n") === [""], i.e. one empty added line.
    expect(right).toEqual([{ text: "", type: "added" }]);
  });

  test("empty original marks every rewrite line as added", () => {
    const { left, right } = computeLineDiff("", "x\ny");
    expect(left).toEqual([{ text: "", type: "removed" }]);
    expect(right).toEqual([
      { text: "x", type: "added" },
      { text: "y", type: "added" },
    ]);
  });

  test("a moved line is reported as one removal and one addition around the LCS", () => {
    // "a b c" -> "b c a": the LCS is "b c", so 'a' leaves the top (removed,
    // left) and reappears at the bottom (added, right).
    const { left, right } = computeLineDiff("a\nb\nc", "b\nc\na");
    expect(left).toEqual([
      { text: "a", type: "removed" },
      { text: "b", type: "same" },
      { text: "c", type: "same" },
    ]);
    expect(right).toEqual([
      { text: "b", type: "same" },
      { text: "c", type: "same" },
      { text: "a", type: "added" },
    ]);
  });
});
