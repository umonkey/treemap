import type { Meta, StoryObj } from '@storybook/react';
import { MapBase } from "./MapBase";

const meta = {
  title: 'Maps/MapBase',
  component: MapBase,
  parameters: {
    layout: 'fullscreen', // centered
  },
} satisfies Meta<typeof MapBase>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Desktop: Story = {
  args: {
    center: {
      lat: 40.181389,
      lon: 44.514444,
    },
  },
  parameters: {
    viewport: {
      defaultViewport: "responsive",
    },
  },
};

export const Phone: Story = {
  args: {
    center: {
      lat: 40.181389,
      lon: 44.514444,
    },
  },
  parameters: {
    viewport: {
      defaultViewport: "mobile1",
    },
  },
};
