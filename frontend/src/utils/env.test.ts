import { getApiRoot } from "./env";

test("should return an empty default API root", () => {
  expect(getApiRoot()).toBe("");
});

test("should return the API root from the environment", () => {
  import.meta.env.VITE_API_ROOT = "http://127.0.0.2:1234";
  expect(getApiRoot()).toBe("http://127.0.0.2:1234");
});

test("should return an empty API root under Storybook", () => {
  import.meta.env.VITE_API_ROOT = "http://127.0.0.2:1234";
  import.meta.env.STORYBOOK = "true";
  expect(getApiRoot()).toBe("");
});
