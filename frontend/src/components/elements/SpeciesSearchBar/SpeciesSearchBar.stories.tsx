import type { Meta, StoryObj } from "@storybook/react";
import { fn } from "@storybook/test";

import { SpeciesSearchBar } from "./SpeciesSearchBar";

const meta = {
  title: "Elements/SpeciesSearchBar",
  component: SpeciesSearchBar,
  parameters: {
    layout: "padded", // centered, fullscreen
  },
  args: {
    onSearch: fn(),
  },
} satisfies Meta<typeof SpeciesSearchBar>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    query: "",
  },
};
