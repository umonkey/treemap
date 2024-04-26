import type { Meta, StoryObj } from "@storybook/react";

import { Header } from "./Header";

const meta = {
  title: "Layout/Header",
  component: Header,
  parameters: {
    layout: "padded", // centered, fullscreen
  },
  args: {
    searchQuery: "Thuja cordata",
  },
} satisfies Meta<typeof Header>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Desktop: Story = {
  args: { },
  parameters: {
    viewport: {
      defaultViewport: "responsive",
    },
  },
};

export const Phone: Story = {
  args: { },
  parameters: {
    viewport: {
      defaultViewport: "mobile1",
    },
  },
};
