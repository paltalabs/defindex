import type { Meta, StoryObj } from "@storybook/react";

import { Code } from "../code";

const meta: Meta<typeof Code> = {
  component: Code,
};
export default meta;

type Story = StoryObj<typeof Code>;

export const Default: Story = {
  args: {
    children: "Code children",
  },
};
