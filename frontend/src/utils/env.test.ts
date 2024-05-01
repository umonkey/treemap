import { getApiRoot, getGoogleClientId, getFileUrlPattern } from "./env";
import { DEFAULT_GOOGLE_CLIENT_ID } from "@/utils/config";

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

test("should return the default Google client ID", () => {
  expect(getGoogleClientId()).toBe(DEFAULT_GOOGLE_CLIENT_ID);
});

test("should return a custom Google client ID", () => {
  import.meta.env.VITE_GOOGLE_CLIENT_ID = "foobar";
  expect(getGoogleClientId()).toBe("foobar");
});

test("should return an empty file url pattern", () => {
  import.meta.env.FILE_URL_PATTERN = "";
  expect(getFileUrlPattern()).toBe(null);
});

test("should return a non-empty file url pattern", () => {
  import.meta.env.FILE_URL_PATTERN = "https://example.com/{id}";
  expect(getFileUrlPattern()).toBe("https://example.com/{id}");
});
