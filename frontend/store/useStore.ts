import { create } from 'zustand';

export interface Item {
  itemId: string;
  name: string;
  images: { thumbnail: string };
  price: { current: number; original: number };
  rarity: string;
  category: string;
  tags: string[];
  metrics: { tiktokMentions: number; viewsLast24h: number; trendingScore: number };
}

interface State {
  items: Item[];
  addItem: (item: Item) => void;
}

const useStore = create<State>((set) => ({
  items: [],
  addItem: (item) => set((state) => ({ items: [item, ...state.items] })),
}));

export default useStore;
