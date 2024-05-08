import type { Meta, StoryObj } from "@storybook/react";
import { fn } from "@storybook/test";

import { TreeCountSelector } from "./TreeCountSelector";

const meta = {
  title: "Blocks/TreeCountSelector",
  component: TreeCountSelector,
  parameters: {
    layout: "padded",
  },
  args: {
    onChange: fn(),
  },
} satisfies Meta<typeof TreeCountSelector>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
};
