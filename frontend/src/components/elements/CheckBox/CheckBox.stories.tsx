import type { Meta, StoryObj } from "@storybook/react";
import { fn } from "@storybook/test";

import { CheckBox } from "./CheckBox";

const meta = {
  title: "Elements/CheckBox",
  component: CheckBox,
  parameters: {
    layout: "padded", // centered, fullscreen
  },
  args: {
    onChange: fn(),
  },
} satisfies Meta<typeof CheckBox>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    value: true,
    label: "Check me",
  },
};
