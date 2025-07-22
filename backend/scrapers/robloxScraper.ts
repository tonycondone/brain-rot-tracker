import fetch from 'node-fetch';

export async function fetchRecentItems() {
  const res = await fetch('https://catalog.roblox.com/v1/search/items?sortType=3');
  const json = await res.json();
  return json.data;
}

export function calcBrainRotScore(name: string, desc: string) {
  const keywords = ['skibidi','toilet','ohio','sigma','gyatt','rizz','sus','among','cringe'];
  const text = (name + ' ' + desc).toLowerCase();
  return keywords.reduce((s, k) => s + (text.includes(k) ? 15 : 0), 0);
}
