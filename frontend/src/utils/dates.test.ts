import { formatDate } from "./dates";

test("should format a date", () => {
  const formatted = formatDate(1713290634);
  expect(formatted).toBe("16.04.2024");
});
