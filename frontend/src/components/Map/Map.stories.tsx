import type { Meta, StoryObj } from '@storybook/react';
import { fn } from '@storybook/test';
import { Map } from "./Map";

const meta = {
  title: 'Maps/Map',
  component: Map,
  parameters: {
    layout: 'fullscreen', // centered
  },
  args: { onClick: fn() },
} satisfies Meta<typeof Map>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Desktop: Story = {
  args: {
    label: 'Map',
  },
};

export const Phone: Story = {
  args: {
    label: 'Map',
  },
};
