import type { Meta, StoryObj } from '@storybook/react';

import { Gallery } from "./Gallery";
import { IGalleryImage } from "@/types";

const IMAGES = [
  {
    small: "https://place-hold.it/300x400",
    large: "https://place-hold.it/600x800",
  },
  {
    small: "https://place-hold.it/400x300",
    large: "https://place-hold.it/800x600",
  },
  {
    small: "https://place-hold.it/300x400",
    large: "https://place-hold.it/600x800",
  },
  {
    small: "https://place-hold.it/400x300",
    large: "https://place-hold.it/800x600",
  },
  {
    small: "https://place-hold.it/300x400",
    large: "https://place-hold.it/600x800",
  },
  {
    small: "https://place-hold.it/400x300",
    large: "https://place-hold.it/800x600",
  },
] as IGalleryImage[];

const meta = {
  title: 'Blocks/Gallery',
  component: Gallery,
  parameters: {
    layout: 'padded', // centered, fullscreen
  },
} satisfies Meta<typeof Gallery>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Desktop: Story = {
  args: {
    images: IMAGES,
  },
  parameters: {
    viewport: {
      defaultViewport: "responsive",
    },
  },
};

export const Phone: Story = {
  args: {
    images: IMAGES,
  },
  parameters: {
    viewport: {
      defaultViewport: "mobile1",
    },
  },
};
