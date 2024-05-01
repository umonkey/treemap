import { formatState } from "./utils";

test("should format state", () => {
  expect(formatState("healthy")).toBe("Looks good.");
  expect(formatState("sick")).toBe("Looks sick.");
  expect(formatState("dead")).toBe("Looks dead.");
  expect(formatState("deformed")).toBe("Looks deformed.");
  expect(formatState("gone")).toBe("Was removed completely.");
});
