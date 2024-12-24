import { create } from "zustand";
import { persist, createJSONStorage } from "zustand/middleware";

export interface UserStore {
  name: string;
  setUser: (name: string) => void;
}

const useUserStore = create<UserStore>()(
  persist(
    (set) => ({
      name: "",
      setUser: (name: string) => set(() => ({ name })),
    }),
    {
      name: "user-storage", // Key for session storage
      storage: createJSONStorage(() => sessionStorage), // Use session storage
    }
  )
);

export default useUserStore;
