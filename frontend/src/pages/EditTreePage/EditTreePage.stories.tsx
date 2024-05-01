import type { Meta, StoryObj } from '@storybook/react';
import { fn } from '@storybook/test';

import { EditTreePage } from "./EditTreePage";

const meta = {
  title: 'Pages/EditTreePage',
  component: EditTreePage,
  parameters: {
    layout: 'fullscreen', // centered
  },
  args: {
    onSuccess: fn(),
    onCancel: fn(),
  },
} satisfies Meta<typeof EditTreePage>;

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
