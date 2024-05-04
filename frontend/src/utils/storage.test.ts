import { getUserInfo, setUserInfo } from "./storage";

describe("utils/storage", () => {
  beforeEach(() => {
    localStorage.clear();
  });

  test("get empty user info", () => {
    const value = getUserInfo();
    expect(value).toBeNull();
  });

  test("change user info", () => {
    const info = {
      name: "Alice",
      picture: "none",
      token: "foobar",
    };

    setUserInfo(info);

    const value = getUserInfo();
    expect(value).toEqual(info);
  });
});
