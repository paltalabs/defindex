import { createListCollection, Field, Input, Portal, Select } from "@chakra-ui/react";
import { basePadding } from "./Common";

interface ReusableSelectProps {
  collection: ReturnType<typeof createListCollection<{ label: string; value: string }>>;
  label: string;
  placeholder: string;
  value?: string[];
  onSelect?: (value: string[]) => void;
}

export function CustomSelect({ collection, label, placeholder, value, onSelect }: ReusableSelectProps) {
  return (
    <Select.Root
      multiple
      collection={collection}
      disabled={collection.items.length === 0}
      value={value}
      onValueChange={(e) => onSelect?.(e.value)}
    >
      <Select.HiddenSelect />
      <Select.Label>{label}</Select.Label>
      <Select.Control>
        <Select.Trigger px={basePadding}>
          <Select.ValueText placeholder={placeholder} />
        </Select.Trigger>
        <Select.IndicatorGroup p={basePadding}>
          <Select.Indicator />
        </Select.IndicatorGroup>
      </Select.Control>
      <Portal>
        <Select.Positioner>
          <Select.Content>
            {collection.items.map((item, index) => (
              <Select.Item key={index} item={item}>
                {item.label}
                <Select.ItemIndicator />
              </Select.Item>
            ))}
          </Select.Content>
        </Select.Positioner>
      </Portal>
    </Select.Root>
  )
}

interface ReusableFromProps {
  label: string;
  placeholder: string;
  type?: string;
  min?: number;
  max?: number;
  value?: string | number;
  onChange?: (e: React.ChangeEvent<HTMLInputElement>) => void;
}
export function FormField({
  label,
  placeholder,
  type = 'text',
  min = undefined,
  max = undefined,
  value,
  onChange }: ReusableFromProps) {
  return (
    <Field.Root>
      <Field.Label>{label}</Field.Label>
      <Input
        type={type}
        min={min}
        max={max}
        placeholder={placeholder}
        px={basePadding}
        value={value}
        onChange={(e) => onChange?.(e)}
      />
    </Field.Root>
  );
}

