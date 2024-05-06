import type { Meta, StoryObj } from '@storybook/react';
import { MapContainer } from "react-leaflet";
import { fn } from '@storybook/test';

import { LayerSelector } from "./LayerSelector";
import "leaflet/dist/leaflet.css";

const POSITION = [56.26, 28.48];

const meta = {
  title: 'Maps/LayerSelector',
  component: LayerSelector,
  parameters: {
    layout: 'fullscreen', // centered
  },
  args: {
    onZoomChange: fn(),
  },
  render: () => (
    <MapContainer center={[POSITION[0], POSITION[1]]} zoom={18} maxZoom={25} scrollWheelZoom={true} className="map" zoomControl={false}>
      <LayerSelector />
    </MapContainer>
  ),
} satisfies Meta<typeof LayerSelector>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: { },
};
