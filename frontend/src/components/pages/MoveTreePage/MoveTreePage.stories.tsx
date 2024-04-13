import type { Meta, StoryObj } from '@storybook/react';
import { fn } from '@storybook/test';

import { MoveTreePage } from "./MoveTreePage";

const meta = {
  title: 'Pages/MoveTreePage',
  component: MoveTreePage,
  parameters: {
    layout: 'fullscreen', // centered
  },
  args: {
    onSuccess: fn(),
    onCancel: fn(),
  },
} satisfies Meta<typeof MoveTreePage>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Desktop: Story = {
  args: {
    id: "134793003121381380",
    token: "foo.bar.baz",
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
    token: "foo.bar.baz",
  },
  parameters: {
    viewport: {
      defaultViewport: "mobile1",
    },
  },
};
