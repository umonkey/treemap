import type { Meta, StoryObj } from '@storybook/react';
import { fn } from '@storybook/test';

import { MapBase } from "@/components/maps/MapBase";
import { DraggableMarker } from "./DraggableMarker";

const POSITION = {
  lat: 56.26,
  lon: 28.48,
};

const meta = {
  title: 'Maps/DraggableMarker',
  component: DraggableMarker,
  parameters: {
    layout: 'fullscreen', // centered
  },
  args: {
    onChange: fn(),
  },
  render: (args) => (
    <MapBase center={POSITION}>
      <DraggableMarker {...args} />
    </MapBase>
  ),
} satisfies Meta<typeof DraggableMarker>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Desktop: Story = {
  args: {
    position: POSITION,
  },
  parameters: {
    viewport: {
      defaultViewport: "responsive",
    },
  },
};

export const Phone: Story = {
  args: {
    position: POSITION,
  },
  parameters: {
    viewport: {
      defaultViewport: "mobile1",
    },
  },
};
