import { createContext } from "react";

interface T {
    index: number;
    setIndex: React.Dispatch<React.SetStateAction<number>>;
}

const NavigateTab = createContext<T>({} as T);

export default NavigateTab;
