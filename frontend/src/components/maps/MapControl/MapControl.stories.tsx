import type { Meta, StoryObj } from '@storybook/react';
import { fn } from '@storybook/test';
import { MapControl } from "./MapControl";
import { DEFAULT_MAP_CENTER, DEFAULT_MAP_ZOOM } from "@/utils/config";

const meta = {
  title: 'Maps/MapControl',
  component: MapControl,
  parameters: {
    layout: 'fullscreen', // centered
  },
  args: {
    onAddTree: fn(),
    onPick: fn(),
  },
} satisfies Meta<typeof MapControl>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Desktop: Story = {
  args: {
    center: DEFAULT_MAP_CENTER,
    zoom: DEFAULT_MAP_ZOOM,
    picker: false,
  },
  parameters: {
    viewport: {
      defaultViewport: "responsive",
    },
  },
};

export const Phone: Story = {
  args: {
    center: DEFAULT_MAP_CENTER,
    zoom: DEFAULT_MAP_ZOOM,
    picker: false,
  },
  parameters: {
    viewport: {
      defaultViewport: "mobile1",
    },
  },
};
