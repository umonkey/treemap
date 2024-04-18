import type { Meta, StoryObj } from '@storybook/react';

import { AddCommentButton } from "./AddCommentButton";

const meta = {
  title: 'Elements/AddCommentButton',
  component: AddCommentButton,
  parameters: {
    layout: 'padded', // centered, fullscreen
  },
} satisfies Meta<typeof AddCommentButton>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Desktop: Story = {
  args: {
    id: "123",
  },
  parameters: {
    viewport: {
      defaultViewport: "responsive",
    },
  },
};

export const Phone: Story = {
  args: {
    id: "123",
  },
  parameters: {
    viewport: {
      defaultViewport: "mobile1",
    },
  },
};
