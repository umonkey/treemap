import { TokenResponse } from "@react-oauth/google";
import { getGoogleUser, setGoogleUser } from "./storage";
import { GOOGLE_TOKEN_STORAGE_KEY } from "./config";

const SAMPLE_TOKEN = {
    access_token: "foo",
    expires_in: 3600,
    token_type: "Bearer",
    scope: "profile email",
} as TokenResponse;

test("should return an empty user", () => {
  const user = getGoogleUser();
  expect(user).toBe(null);
});

test("should return a previously saved user", () => {
  localStorage.setItem(GOOGLE_TOKEN_STORAGE_KEY, JSON.stringify(SAMPLE_TOKEN));
  const user = getGoogleUser();
  expect(user).toStrictEqual(SAMPLE_TOKEN);
});

test("should save the token", () => {
  localStorage.clear();
  expect(localStorage.getItem(GOOGLE_TOKEN_STORAGE_KEY)).toBe(null);

  setGoogleUser(SAMPLE_TOKEN);

  expect(localStorage.getItem(GOOGLE_TOKEN_STORAGE_KEY)).toStrictEqual('{"access_token":"foo","expires_in":3600,"token_type":"Bearer","scope":"profile email"}');
});
