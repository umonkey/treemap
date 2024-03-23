import { render, screen } from "@testing-library/react";
import "@testing-library/jest-dom";

import { SideBar } from "./SideBar";

describe("SideBar", () => {
  test("displays content", () => {
    render(<SideBar><p>It Works!</p></SideBar>);

    const p = screen.getByText("It Works!");
    expect(p).toBeInTheDocument();
  });
});
