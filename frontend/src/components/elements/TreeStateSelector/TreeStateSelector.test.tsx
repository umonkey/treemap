import { render, fireEvent, within } from "@testing-library/react";
import "@testing-library/jest-dom";
import { vi } from "vitest";

import { TreeStateSelector } from "./TreeStateSelector";

describe("TreeStateSelector", () => {
  test("change to sick", () => {
    const onChange = vi.fn();

    const { getByRole } = render(
      <TreeStateSelector state="healthy" onChange={onChange} />
    );

    fireEvent.mouseDown(getByRole("combobox"));

    const listbox = within(getByRole("listbox"));
    fireEvent.click(listbox.getByText("Gone"));

    expect(onChange).toHaveBeenCalledWith("gone");
  });

  test("no change", () => {
    const onChange = vi.fn();

    const { getByRole } = render(
      <TreeStateSelector state="healthy" onChange={onChange} />
    );

    fireEvent.mouseDown(getByRole("combobox"));

    const listbox = within(getByRole("listbox"));
    fireEvent.click(listbox.getByText("Healthy"));

    expect(onChange).not.toHaveBeenCalled();
  });
});
