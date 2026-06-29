import { describe, it, expect } from "vitest";
import { toUserMessage } from "./ipc";

// QA-R2-mn1 / QA-R2-mn2: typed `UPPER_SNAKE:` prefixes the backend emits must be
// translated into plain-language guidance with the machine token stripped, not
// leaked as "Something went wrong: NO_EVIDENCE: ...".
describe("toUserMessage typed-prefix translation", () => {
  it("translates NO_EVIDENCE: into neutral source/window guidance", () => {
    const msg = toUserMessage(
      "NO_EVIDENCE: No recent evidence was found after checking sources."
    );
    expect(msg).not.toContain("NO_EVIDENCE");
    expect(msg).not.toContain("Something went wrong");
    expect(msg.toLowerCase()).toContain("checking sources");
    expect(msg.toLowerCase()).toContain("widen the scan window");
  });

  it("translates MODEL_NOT_INSTALLED: into an AI Model remedy", () => {
    const msg = toUserMessage(
      "MODEL_NOT_INSTALLED: The selected AI model 'qwen3:8b' is not installed. Open Setup and download a model before drafting."
    );
    expect(msg).not.toContain("MODEL_NOT_INSTALLED");
    expect(msg).not.toContain("Something went wrong");
    expect(msg.toLowerCase()).toContain("ai model");
  });

  it("strips an unknown typed prefix and surfaces the human-readable remainder", () => {
    const msg = toUserMessage("SOME_OTHER_CODE: The widget could not be frobnicated.");
    expect(msg).not.toContain("SOME_OTHER_CODE");
    expect(msg).toBe("The widget could not be frobnicated.");
  });

  it("handles a typed prefix carried inside an Error object", () => {
    const msg = toUserMessage(new Error("NO_EVIDENCE: nothing here yet."));
    expect(msg).not.toContain("NO_EVIDENCE");
    expect(msg.toLowerCase()).toContain("checking sources");
  });

  it("leaves ordinary lowercase messages untouched by the prefix branch", () => {
    // A normal sentence shouldn't be mistaken for a typed prefix.
    const msg = toUserMessage("could not reach ollama: connection refused");
    expect(msg.toLowerCase()).toContain("ollama");
  });
});
