const fs = require('node:fs/promises');
require('module').Module._initPaths();
const { chromium } = require('playwright');
(async()=>{
  const out=process.argv[2];
  const browser=await chromium.connectOverCDP('http://127.0.0.1:9224');
  const page=browser.contexts()[0].pages()[0];
  await page.getByRole('button', {name:/Finish Onboarding/i}).click();
  await page.waitForTimeout(3000);
  await page.screenshot({path:`${out}/cdp-10-dashboard-after-finish.png`, fullPage:true});
  const text=await page.locator('body').innerText().catch(async()=>await page.textContent('body')) || '';
  await fs.writeFile(`${out}/cdp-10-dashboard-after-finish.txt`, text, 'utf8');
  const info=await page.evaluate(()=>({buttons:[...document.querySelectorAll('button')].map((b,i)=>({i,text:b.innerText,aria:b.getAttribute('aria-label'),disabled:b.disabled})), inputs:[...document.querySelectorAll('input,textarea,select')].map((el,i)=>({i,tag:el.tagName,type:el.getAttribute('type'),value:el.value,placeholder:el.getAttribute('placeholder'),label:el.getAttribute('aria-label'),name:el.getAttribute('name')}))}));
  await fs.writeFile(`${out}/cdp-10-dashboard-after-finish.json`, JSON.stringify(info,null,2),'utf8');
  await browser.close();
})().catch(e=>{console.error(e);process.exit(1)});
