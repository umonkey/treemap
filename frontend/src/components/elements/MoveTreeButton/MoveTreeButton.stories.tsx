import type { Meta, StoryObj } from '@storybook/react';

import { MoveTreeButton } from "./MoveTreeButton";

const TREE_ID = "135724792375545856";

const meta = {
  title: 'Elements/MoveTreeButton',
  component: MoveTreeButton,
  parameters: {
    layout: 'padded', // centered, fullscreen
  },
  args: { },
} satisfies Meta<typeof MoveTreeButton>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Desktop: Story = {
  args: {
    id: TREE_ID,
  },
  parameters: {
    viewport: {
      defaultViewport: "responsive",
    },
  },
};

export const Phone: Story = {
  args: {
    id: TREE_ID,
  },
  parameters: {
    viewport: {
      defaultViewport: "mobile1",
    },
  },
};
