import { createListCollection, Field, Input, Portal, Select } from "@chakra-ui/react";
import { basePadding } from "./Common";
import './CustomInputFields.css';
interface ReusableSelectProps {
  collection: ReturnType<typeof createListCollection<{ label: string; value: string }>>;
  label: string;
  placeholder: string;
  value?: string[];
  onSelect?: (value: string[]) => void;
  multiple?: boolean;
}

export function CustomSelect({ collection, label, placeholder, value, onSelect, multiple }: ReusableSelectProps) {
  if (!collection.items || collection.items.length === 0) return null;
  return (
    <Select.Root
      multiple={multiple}
      collection={collection}
      disabled={!collection.items || collection.items.length === 0}
      value={value}
      onValueChange={(e) => onSelect?.(e.value)}
    >
      <Select.HiddenSelect />
      <Select.Label truncate className="custom-input-label">{label}</Select.Label>
      <Select.Control>
        <Select.Trigger px={basePadding} className="custom-select">
          <Select.ValueText placeholder={placeholder} />
        </Select.Trigger>
        <Select.IndicatorGroup p={basePadding} >
          <Select.Indicator />
        </Select.IndicatorGroup>
      </Select.Control>
      <Portal>
        <Select.Positioner>
          <Select.Content className="custom-select-content">
            {collection.items.map((item, index) => (
              <Select.Item key={index} item={item} className="custom-select-item">
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
  invalid?: boolean;
  errorMessage?: string;
}
export function FormField({
  label,
  placeholder,
  type = 'text',
  min = undefined,
  max = undefined,
  value,
  onChange,
  invalid = false,
  errorMessage = undefined,
}: ReusableFromProps) {
  return (
    <Field.Root invalid={value !== '' && invalid} justifyItems={'start'}>
      <Field.Label truncate className="custom-input-label">{label}</Field.Label>
      <Input
        type={type}
        min={min}
        max={max}
        placeholder={placeholder}
        px={basePadding}
        value={value}
        onChange={(e) => onChange?.(e)}
        className="custom-input-field"
      />
      <Field.ErrorText>{errorMessage}</Field.ErrorText>
    </Field.Root>
  );
}

