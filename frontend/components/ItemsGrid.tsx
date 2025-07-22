import React from 'react';
import ItemCard from './ItemCard';
import useStore from '../store/useStore';

export default function ItemsGrid() {
  const items = useStore((s) => s.items);

  return (
    <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
      {items.map((item) => (
        <ItemCard key={item.itemId} item={item} />
      ))}
    </div>
  );
} 