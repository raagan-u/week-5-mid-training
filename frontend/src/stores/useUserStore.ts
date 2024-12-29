import { create } from "zustand";
import { persist, createJSONStorage } from "zustand/middleware";

interface PollOption {
  option_id: number; // Represents i64 in Rust
  text: string;
  votes: number; // Represents i32 in Rust
}

interface Poll {
  poll_id: number; // Represents i64 in Rust
  title: string;
  creator: string;
  description: string;
  created_at: string; // ISO 8601 string for DateTime<Utc>
  expiration_date?: string | null; // Optional ISO 8601 string for Option<DateTime<Utc>>
  status: "Active" | "expired" | "closed"; // Enum-like string for the status field
  options: PollOption[]; // Array of PollOption
}

export interface UserStore {
  name: string;
  ownedPolls: Poll[];
  setOwnedPolls: (polls: Poll[]) => void;
  clearOwnedPolls: (name: string) => void;
  setUser: (name: string) => void;
  ClearUser: (name: string) => void;
}

const useUserStore = create<UserStore>()(
  persist(
    (set) => ({
      name: "",
      ownedPolls: [],
      setOwnedPolls: (polls) => set((state) => ({
        ownedPolls: polls.filter((poll) => poll.creator == state.name),
      })),
      clearOwnedPolls: () => set(() => ({
        ownedPolls: []
      })),
      setUser: (name: string) => set(() => ({ name })),
      ClearUser: () => set(({name: "", ownedPolls: []})),
    }),
    {
      name: "user-storage", 
      storage: createJSONStorage(() => sessionStorage),
    }
  )
);

export default useUserStore;
