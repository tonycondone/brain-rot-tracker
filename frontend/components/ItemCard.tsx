import React from 'react';
import { motion } from 'framer-motion';

export default function ItemCard({ item }) {
  return (
    <motion.div
      className="bg-white rounded-lg shadow-md p-4 hover:scale-105 transform transition"
      whileHover={{ scale: 1.05 }}
    >
      <div className="relative">
        <img src={item.images.thumbnail} alt={item.name} className="w-full h-40 object-cover rounded"/>
        <div className="absolute top-2 left-2 bg-black text-white text-xs px-2 py-1 rounded">
          {item.rarity}
        </div>
        {item.metrics.trendingScore > 80 && (
          <div className="absolute top-2 right-2">🔥</div>
        )}
      </div>
      <h3 className="mt-2 font-semibold">{item.name}</h3>
      <p className="text-sm text-gray-500">{item.category}</p>
      <div className="mt-2">
        <span className="font-bold">{item.price.current} R$</span>
        {item.price.original !== item.price.current && (
          <span className="line-through text-gray-500 ml-2">{item.price.original} R$</span>
        )}
      </div>
      <div className="mt-2 flex flex-wrap gap-1">
        {item.tags.map((tag) => (
          <span key={tag} className="text-xs bg-blue-100 px-2 py-1 rounded">
            #{tag}
          </span>
        ))}
      </div>
      <div className="mt-2 text-xs text-gray-600">
        🎵 {item.metrics.tiktokMentions} • 👀 {item.metrics.viewsLast24h}
      </div>
      <button
        className="mt-3 bg-blue-500 text-white px-4 py-2 rounded"
        onClick={() =>
          window.open(`https://www.roblox.com/catalog/${item.itemId}`, '_blank')
        }
      >
        View on Roblox
      </button>
    </motion.div>
  );
} 