import { formatMeta, formatState } from "./utils";

test("should round numbers", () => {
  const meta = formatMeta({
    id: "1",
    lat: 0,
    lon: 0,
    species: "Quercus",
    notes: "Oak",
    height: 1.234,
    circumference: 2.345,
    diameter: 3.456,
    state: "healthy",
    updated_at: 0,
    thumbnail_id: null,
  });

  expect(meta).toBe("H= 1.2 m, C= 2.35 m, D= 3.5 m");
});

test("should format state", () => {
  expect(formatState("healthy")).toBe("Looks good.");
  expect(formatState("sick")).toBe("Looks sick.");
  expect(formatState("dead")).toBe("Looks dead.");
  expect(formatState("deformed")).toBe("Looks deformed.");
  expect(formatState("gone")).toBe("Was removed completely.");
});
