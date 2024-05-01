import type { Meta, StoryObj } from '@storybook/react';

import { TreePreviewButtons } from "./TreePreviewButtons";

const meta = {
  title: 'Elements/TreePreviewButtons',
  component: TreePreviewButtons,
  parameters: {
    layout: 'padded', // centered, fullscreen
  },
} satisfies Meta<typeof TreePreviewButtons>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    id: "123456",
  },
  parameters: {
    viewport: {
      defaultViewport: "responsive",
    },
  },
};
