import type { Meta, StoryObj } from "@storybook/react";
import { fn } from "@storybook/test";

import { PositionSelector } from "./PositionSelector";

const meta = {
  title: "Maps/PositionSelector",
  component: PositionSelector,
  parameters: {
    layout: "padded",
  },
  args: {
    onChange: fn(),
  },
} satisfies Meta<typeof PositionSelector>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
};
