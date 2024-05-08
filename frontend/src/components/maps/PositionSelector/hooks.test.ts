import { rowBuilder } from "./hooks";

describe("PositionSelector hooks", () => {
  describe("rowBuilder", () => {
    test("Build a row of 2 points", () => {
      const a = { lat: 1, lon: 2 };
      const b = { lat: 3, lon: 4 };

      const row = rowBuilder(a, b, 2);
      expect(row).toEqual([a, b]);
    });

    test("Build a row of 3 points", () => {
      const a = { lat: 1, lon: 1 };
      const b = { lat: 2, lon: 2 };

      const row = rowBuilder(a, b, 3);

      expect(row).toEqual([
        {
          lat: 1,
          lon: 1,
        },
        {
          lat: 1.5,
          lon: 1.5,
        },
        {
          lat: 2,
          lon: 2,
        },
      ]);
    });
  });
});
