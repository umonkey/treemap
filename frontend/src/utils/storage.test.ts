import { getLoginInfo, setLoginInfo } from "./storage";

describe("utils/storage", () => {
  beforeEach(() => {
    localStorage.clear();
  });

  test("get empty user info", () => {
    const value = getLoginInfo();
    expect(value).toBeNull();
  });

  test("change user info", () => {
    const info = {
      name: "Alice",
      picture: "none",
      token: "foobar",
    };

    setLoginInfo(info);

    const value = getLoginInfo();
    expect(value).toEqual(info);
  });
});
