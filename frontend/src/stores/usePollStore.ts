import { create } from "zustand";
import { persist, createJSONStorage } from "zustand/middleware";

interface PollOption {
    option_id: number;
    text: string;
    votes: number;
  }
  
  interface Poll {
    poll_id: number; 
    title: string;
    creator: string;
    description: string;
    created_at: string;
    expiration_date?: string | null;
    status: "Active" | "expired" | "closed";
    options: PollOption[];
    users_voted: number[];
  }
  

export interface PollStore {
    polls: Poll[],
    initPolls: (polls: []) => void
    addPoll: (poll: Poll) => void
}

const usePollStore = create<PollStore>()(
  persist(
    (set) => ({
      polls: [],
      initPolls: (polls: []) => set(() => ({ polls })),
      addPoll: (poll: Poll) =>
        set((state) => ({
          polls: [...state.polls, poll],
        })),
    }),
    {
      name: "polls-storage",
      storage: createJSONStorage(() => sessionStorage),
    }
  )
);

export default usePollStore;
