import type { Meta, StoryObj } from "@storybook/react";

import { MapBase } from "@/components/maps/MapBase";
import { Markers } from "./Markers";
import { ITreeInfo } from "@/types";
import { DEFAULT_MAP_CENTER, DEFAULT_MAP_ZOOM } from "@/utils/config";

const MARKERS = [
  {
    id: "1",
    lat: 40.181389,
    lon: 44.514444,
    name: "Old birch",
    height: 17,
    circumference: 2.3,
    diameter: 15,
    state: "healthy",
    updated_at: 1712916557,
  },
  {
    id: "2",
    lat: 40.180379,
    lon: 44.513434,
    name: "Great Oak",
    height: 17,
    circumference: 2.3,
    diameter: 15,
    state: "sick",
    updated_at: 1712916557,
  },
  {
    id: "3",
    lat: 40.182399,
    lon: 44.515454,
    name: "Small Willow",
    height: 17,
    circumference: 2.3,
    diameter: 15,
    state: "dead",
    updated_at: 1712916557,
  },
] as ITreeInfo[];

const meta = {
  title: "Maps/Markers",
  component: Markers,
  parameters: {
    layout: "fullscreen", // centered
  },
  args: { },
  render: (args) => (
    <MapBase center={DEFAULT_MAP_CENTER} zoom={DEFAULT_MAP_ZOOM}>
      <Markers {...args} />
    </MapBase>
  ),
} satisfies Meta<typeof Markers>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Desktop: Story = {
  args: {
    markers: MARKERS,
  },
  parameters: {
    viewport: {
      defaultViewport: "responsive",
    },
  },
};

export const Phone: Story = {
  args: {
    markers: MARKERS,
  },
  parameters: {
    viewport: {
      defaultViewport: "mobile1",
    },
  },
};
