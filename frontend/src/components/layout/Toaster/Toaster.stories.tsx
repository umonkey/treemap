import type { Meta, StoryObj } from "@storybook/react";
import { toast } from "react-hot-toast";

import { Toaster } from "./Toaster";

const meta = {
  title: "Layout/Toaster",
  component: Toaster,
  parameters: {
    layout: "padded", // centered, fullscreen
  },
  render: () => (
    <>
      <Toaster />
      <button onClick={() => {
        toast("Hello, world.");
      }}>Show a toast</button>
    </>
  ),
} satisfies Meta<typeof Toaster>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
};
