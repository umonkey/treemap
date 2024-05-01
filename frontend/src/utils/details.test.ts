import { SAMPLE_TREES } from "@/sample-data";

import { formatTreeDimensions } from "./details";

describe("utils/details", () => {
  test("formatDimensions (full)", () => {
    const tree = {
      ...SAMPLE_TREES[0],
      height: 1.234,
      circumference: 2.345,
      diameter: 3.456,
    };

    const text = formatTreeDimensions(tree);
    expect(text).toBe("H= 1.2 m, C= 2.35 m, D= 3.5 m");
  });

  test("formatDimensions (partial)", () => {
    const tree = {
      ...SAMPLE_TREES[0],
      height: null,
      circumference: 2.345,
      diameter: null,
    };

    const text = formatTreeDimensions(tree);
    expect(text).toBe("C= 2.35 m");
  });
});
