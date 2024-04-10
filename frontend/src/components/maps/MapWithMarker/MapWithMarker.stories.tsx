import type { Meta, StoryObj } from '@storybook/react';
import { MapWithMarker } from "./MapWithMarker";

import { ILatLng } from "@/types";

const CENTER = {
  lat: 40.181389,
  lon: 44.514444,
} as ILatLng;

const meta = {
  title: 'Maps/MapWithMarker',
  component: MapWithMarker,
  parameters: {
    layout: 'fullscreen', // centered
  },
} satisfies Meta<typeof MapWithMarker>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Desktop: Story = {
  args: {
    center: CENTER,
  },
  parameters: {
    viewport: {
      defaultViewport: "responsive",
    },
  },
};

export const Phone: Story = {
  args: {
    center: CENTER,
  },
  parameters: {
    viewport: {
      defaultViewport: "mobile1",
    },
  },
};
