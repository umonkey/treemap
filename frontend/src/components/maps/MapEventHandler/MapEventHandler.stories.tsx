import type { Meta, StoryObj } from '@storybook/react';
import { fn } from '@storybook/test';

import { MapBase } from "@/components/maps/MapBase";
import { MapEventHandler } from "./MapEventHandler";
import { DEFAULT_MAP_CENTER, DEFAULT_MAP_ZOOM } from '@/utils/config';

const meta = {
  title: 'Maps/MapEventHandler',
  component: MapEventHandler,
  parameters: {
    layout: 'fullscreen', // centered
  },
  args: {
    onClick: fn(),
    onBoundsChange: fn(),
    onViewChange: fn(),
  },
  render: (args) => (
    <MapBase center={DEFAULT_MAP_CENTER} zoom={DEFAULT_MAP_ZOOM}>
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
