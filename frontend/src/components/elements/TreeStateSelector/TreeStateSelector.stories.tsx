import type { Meta, StoryObj } from '@storybook/react';
import { fn } from '@storybook/test';

import { TreeStateSelector } from "./TreeStateSelector";

const meta = {
  title: 'Elements/TreeStateSelector',
  component: TreeStateSelector,
  parameters: {
    layout: 'padded', // centered, fullscreen
  },
  args: {
    onChange: fn(),
  },
} satisfies Meta<typeof TreeStateSelector>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Desktop: Story = {
  args: {
    state: "healthy",
  },
  parameters: {
    viewport: {
      defaultViewport: "responsive",
    },
  },
};

export const Phone: Story = {
  args: {
    state: "healthy",
  },
  parameters: {
    viewport: {
      defaultViewport: "mobile1",
    },
  },
};
