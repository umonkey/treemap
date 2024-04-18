import type { Meta, StoryObj } from '@storybook/react';

import { TreeComments } from "./TreeComments";

const meta = {
  title: 'Blocks/TreeComments',
  component: TreeComments,
  parameters: {
    layout: 'padded', // centered, fullscreen
  },
} satisfies Meta<typeof TreeComments>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Desktop: Story = {
  args: {
    id: "134793003121381380",
  },
  parameters: {
    viewport: {
      defaultViewport: "responsive",
    },
  },
};

export const Phone: Story = {
  args: {
    id: "134793003121381380",
  },
  parameters: {
    viewport: {
      defaultViewport: "mobile1",
    },
  },
};
