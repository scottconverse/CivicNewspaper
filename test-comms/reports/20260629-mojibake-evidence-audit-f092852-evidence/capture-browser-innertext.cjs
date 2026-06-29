const { chromium } = require('playwright');
const fs = require('fs');
(async () => {
  const browser = await chromium.launch({ headless: true, executablePath: 'C:/Program Files/Google/Chrome/Application/chrome.exe' });
  const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
  let result = { url: 'https://merry-frost-9arx.here.now', reachable: false, status: null, error: null };
  try {
    const response = await page.goto('https://merry-frost-9arx.here.now', { waitUntil: 'networkidle', timeout: 45000 });
    result.status = response ? response.status() : null;
    result.reachable = !!response && response.ok();
    const text = await page.evaluate(() => document.body ? document.body.innerText : '');
    fs.writeFileSync(process.argv[2], text, { encoding: 'utf8' });
    await page.screenshot({ path: process.argv[3], fullPage: true });
  } catch (err) {
    result.error = err && err.message ? err.message : String(err);
  } finally {
    await browser.close();
    fs.writeFileSync(process.argv[4], JSON.stringify(result, null, 2), { encoding: 'utf8' });
  }
})();