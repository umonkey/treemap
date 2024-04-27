import type { Meta, StoryObj } from "@storybook/react";
import { fn } from "@storybook/test";

import { SearchBar } from "./SearchBar";

const meta = {
  title: "Elements/SearchBar",
  component: SearchBar,
  parameters: {
    layout: "padded", // centered, fullscreen
  },
  args: {
    searchQuery: "Thuja cordata",
    onChange: fn(),
  },
} satisfies Meta<typeof SearchBar>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Desktop: Story = {
  parameters: {
    viewport: {
      defaultViewport: "responsive",
    },
  },
};

export const Phone: Story = {
  parameters: {
    viewport: {
      defaultViewport: "mobile1",
    },
  },
};
