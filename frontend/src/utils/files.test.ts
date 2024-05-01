import { getFileURL } from "./files";

describe("utils/files", () => {
  beforeEach(() => {
    import.meta.env.STORYBOOK = "";
    import.meta.env.FILE_URL_PATTERN = "";
    import.meta.env.VITE_API_ROOT = "";
  });

  test("image links for Storybook", () => {
    import.meta.env.STORYBOOK = "true";
    expect(getFileURL("123")).toBe("https://placehold.co/600x400?text=123");
  });

  test("image links with a pattern", () => {
    import.meta.env.FILE_URL_PATTERN = "https://example.com/{id}";
    expect(getFileURL("123")).toBe("https://example.com/123");
  });

  test("image links with an API root", () => {
    import.meta.env.VITE_API_ROOT = "https://example.net";
    expect(getFileURL("123")).toBe("https://example.net/v1/files/123");
  });
});
