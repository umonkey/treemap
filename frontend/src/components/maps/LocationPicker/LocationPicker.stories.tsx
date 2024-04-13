import type { Meta, StoryObj } from '@storybook/react';
import { fn } from '@storybook/test';

import { MapBase } from "@/components/maps/MapBase";
import { LocationPicker } from "./LocationPicker";
import { DEFAULT_MAP_CENTER, DEFAULT_MAP_ZOOM } from "@/utils/config";

const meta = {
  title: 'Maps/LocationPicker',
  component: LocationPicker,
  parameters: {
    layout: 'fullscreen', // centered
  },
  args: {
    onChange: fn(),
  },
  render: (args) => (
    <MapBase center={DEFAULT_MAP_CENTER} zoom={DEFAULT_MAP_ZOOM}>
      <LocationPicker {...args} />
    </MapBase>
  ),
} satisfies Meta<typeof LocationPicker>;

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
