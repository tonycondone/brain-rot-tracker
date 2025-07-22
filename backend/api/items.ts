import { NextApiRequest, NextApiResponse } from 'next';
import { Item } from '../db/models';
import mongoose from 'mongoose';

mongoose.connect(process.env.MONGO_URI!);

export default async function handler(req: NextApiRequest, res: NextApiResponse) {
  const items = await Item.find().sort({ 'metrics.trendingScore': -1 }).limit(50).exec();
  res.status(200).json(items);
}
