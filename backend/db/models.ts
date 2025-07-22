import { Schema, model } from 'mongoose';

const PriceHistorySchema = new Schema({
  price: Number,
  timestamp: Date,
});

const ItemSchema = new Schema({
  itemId: { type: String, unique: true },
  name: String,
  description: String,
  category: String,
  brainRotType: String,
  price: {
    current: Number,
    original: Number,
    lowest: Number,
    highest: Number,
    history: [PriceHistorySchema],
  },
  rarity: String,
  availability: {
    inStock: Boolean,
    quantity: Number,
    isLimited: Boolean,
    endDate: Date,
  },
  metrics: {
    tiktokMentions: Number,
    viewsLast24h: Number,
    salesVelocity: Number,
    trendingScore: Number,
  },
  images: {
    thumbnail: String,
    fullSize: String,
    inGame: [String],
  },
  creator: {
    name: String,
    userId: String,
    verified: Boolean,
  },
  tags: [String],
  scrapedFrom: [String],
  lastUpdated: Date,
  createdAt: { type: Date, default: Date.now },
});

export const Item = model('Item', ItemSchema);
