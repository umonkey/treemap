import type { Meta, StoryObj } from '@storybook/react';
import { fn } from '@storybook/test';

import { ImagePicker } from "./ImagePicker";

const meta = {
  title: 'Elements/ImagePicker',
  component: ImagePicker,
  parameters: {
    layout: 'padded', // centered, fullscreen
  },
  args: {
    onChange: fn(),
  },
} satisfies Meta<typeof ImagePicker>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Desktop: Story = {
  args: {
    disabled: false,
  },
  parameters: {
    viewport: {
      defaultViewport: "responsive",
    },
  },
};

export const Phone: Story = {
  args: {
    disabled: false,
  },
  parameters: {
    viewport: {
      defaultViewport: "mobile1",
    },
  },
};
