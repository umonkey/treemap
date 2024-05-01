import type { Meta, StoryObj } from '@storybook/react';
import { MapControl } from "./MapControl";
import { DEFAULT_MAP_CENTER, DEFAULT_MAP_ZOOM } from "@/utils/config";

const meta = {
  title: 'Maps/MapControl',
  component: MapControl,
  parameters: {
    layout: 'fullscreen', // centered
  },
} satisfies Meta<typeof MapControl>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    center: DEFAULT_MAP_CENTER,
    zoom: DEFAULT_MAP_ZOOM,
  },
};
