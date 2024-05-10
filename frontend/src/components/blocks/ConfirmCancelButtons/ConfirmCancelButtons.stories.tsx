import type { Meta, StoryObj } from "@storybook/react";

import { ConfirmCancelButtons } from "./ConfirmCancelButtons";
import { fn } from "@storybook/test";

const meta = {
  title: "Blocks/ConfirmCancelButtons",
  component: ConfirmCancelButtons,
  parameters: {
    layout: "padded",
  },
  args: {
    onCancel: fn(),
    onConfirm: fn(),
  },
} satisfies Meta<typeof ConfirmCancelButtons>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Enabled: Story = {
  args: {
    canConfirm: true,
  },
};

export const Disabled: Story = {
  args: {
    canConfirm: false,
  },
};
