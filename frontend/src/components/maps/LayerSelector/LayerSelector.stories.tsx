import type { Meta, StoryObj } from '@storybook/react';
import { MapContainer } from "react-leaflet";

import { LayerSelector } from "./LayerSelector";
import "leaflet/dist/leaflet.css";

const POSITION = [56.26, 28.48];

const meta = {
  title: 'Maps/LayerSelector',
  component: LayerSelector,
  parameters: {
    layout: 'fullscreen', // centered
  },
  args: { },
  render: () => (
    <MapContainer center={[POSITION[0], POSITION[1]]} zoom={13} maxZoom={18} scrollWheelZoom={true} className="map" zoomControl={false}>
      <LayerSelector />
    </MapContainer>
  ),
} satisfies Meta<typeof LayerSelector>;

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
