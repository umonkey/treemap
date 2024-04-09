import type { Meta, StoryObj } from '@storybook/react';

import { MapBase } from "@/components/maps/MapBase";
import { LocationPicker } from "./LocationPicker";
import { ITreeInfo } from "@/types";
import { fn } from '@storybook/test';

const POSITION = {
  lat: 40.180379,
  lon: 44.513434,
} as ITreeInfo;

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
    <MapBase center={POSITION}>
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
      defaultViewport: "desktop",
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
