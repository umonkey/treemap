import type { Meta, StoryObj } from '@storybook/react';
import { fn } from '@storybook/test';

import { MapBase } from "@/components/maps/MapBase";
import { MapEventHandler } from "./MapEventHandler";

const POSITION = {
  lat: 56.26,
  lon: 28.48,
};

const meta = {
  title: 'Maps/MapEventHandler',
  component: MapEventHandler,
  parameters: {
    layout: 'fullscreen', // centered
  },
  args: {
    onClick: fn(),
    onBoundsChange: fn(),
  },
  render: (args) => (
    <MapBase center={POSITION}>
      <MapEventHandler {...args} />
    </MapBase>
  ),
} satisfies Meta<typeof MapEventHandler>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Desktop: Story = {
  args: { },
  parameters: {
    viewport: {
      defaultViewport: "responsive",
    },
  },
};

export const Phone: Story = {
  args: { },
  parameters: {
    viewport: {
      defaultViewport: "mobile1",
    },
  },
};
