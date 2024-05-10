import type { Meta, StoryObj } from '@storybook/react';

import { PreviewToolbar } from "./PreviewToolbar";

const meta = {
  title: 'Blocks/PreviewToolbar',
  component: PreviewToolbar,
  parameters: {
    layout: "fullscreen",
    viewport: {
      defaultViewport: "mobile1",
    },
  },
} satisfies Meta<typeof PreviewToolbar>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    id: "134793003121381380",
  },
};
