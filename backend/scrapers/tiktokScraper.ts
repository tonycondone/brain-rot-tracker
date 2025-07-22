import { chromium } from 'playwright';
import cheerio from 'cheerio';

export async function scrapeTikTok(hashtag: string) {
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  await page.goto(`https://www.tiktok.com/tag/${hashtag}`);
  await page.waitForSelector('[data-e2e="challenge-item"]');
  const html = await page.content();
  await browser.close();

  const $ = cheerio.load(html);
  const videos: Array<{ description: string; views: number }> = [];

  $('[data-e2e="challenge-item"]').each((_, el) => {
    const desc = $(el).find('[data-e2e="browse-video-desc"]').text();
    const viewsText = $(el).find('[data-e2e="video-views"]').text().replace(/\D/g, '');
    const views = parseInt(viewsText) || 0;
    videos.push({ description: desc, views });
  });

  return videos.filter(v => /roblox/i.test(v.description));
}
