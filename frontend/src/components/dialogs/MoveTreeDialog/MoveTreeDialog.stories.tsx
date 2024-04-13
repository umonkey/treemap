import type { Meta, StoryObj } from '@storybook/react';
import { fn } from '@storybook/test';

import { MoveTreeDialog } from "./MoveTreeDialog";
import { DEFAULT_MAP_CENTER } from "@/utils/config";

const meta = {
  title: 'Dialogs/MoveTreeDialog',
  component: MoveTreeDialog,
  parameters: {
    layout: 'padded', // centered, fullscreen
  },
  args: {
    onContinue: fn(),
    onCancel: fn(),
  },
} satisfies Meta<typeof MoveTreeDialog>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Desktop: Story = {
  args: {
    position: DEFAULT_MAP_CENTER,
    busy: false,
    error: "Error updating location.",
  },
  parameters: {
    viewport: {
      defaultViewport: "responsive",
    },
  },
};

export const Phone: Story = {
  args: {
    position: DEFAULT_MAP_CENTER,
    busy: false,
    error: "Error updating location.",
  },
  parameters: {
    viewport: {
      defaultViewport: "mobile1",
    },
  },
};
