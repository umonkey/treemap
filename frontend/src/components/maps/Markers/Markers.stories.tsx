import type { Meta, StoryObj } from '@storybook/react';

import { MapBase } from "@/components/maps/MapBase";
import { Markers } from "./Markers";
import { ITreeInfo } from "@/types";

const POSITION = {
  lat: 40.180379,
  lon: 44.513434,
} as ITreeInfo;

const MARKERS = [ 
  {
    id: "1",
    lat: 40.181389,
    lon: 44.514444,
    name: 'Old birch',
  },
  {
    id: "2",
    lat: 40.180379,
    lon: 44.513434,
    name: 'Great Oak',
  },
  {
    id: "3",
    lat: 40.182399,
    lon: 44.515454,
    name: 'Small Willow',
  },
] as ITreeInfo[];

const meta = {
  title: 'Maps/Markers',
  component: Markers,
  parameters: {
    layout: 'fullscreen', // centered
  },
  args: { },
  render: (args) => (
    <MapBase center={POSITION}>
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
