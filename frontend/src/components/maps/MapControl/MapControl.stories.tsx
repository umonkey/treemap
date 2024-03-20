import type { Meta, StoryObj } from '@storybook/react';
import { fn } from '@storybook/test';
import { MapControl } from "./MapControl";

const meta = {
  title: 'Maps/MapControl',
  component: MapControl,
  parameters: {
    layout: 'fullscreen', // centered
  },
  args: {
    onBoundsChange: fn(),
  },
} satisfies Meta<typeof MapControl>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Desktop: Story = {
  args: {
    center: {
      lat: 40.181389,
      lon: 44.514444,
    },
    markers: [
      {
        id: 1,
        lat: 40.181389,
        lon: 44.514444,
      },
      {
        id: 2,
        lat: 40.180379,
        lon: 44.513434,
      },
      {
        id: 3,
        lat: 40.182399,
        lon: 44.515454,
      },
    ],
  },
};

export const Phone: Story = {
  args: {
    center: {
      lat: 40.181389,
      lon: 44.514444,
    },
    markers: [
      {
        id: 1,
        lat: 40.181389,
        lon: 44.514444,
      },
    ],
  },
};
