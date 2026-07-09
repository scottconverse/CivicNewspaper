const fs = require('node:fs/promises');
require('module').Module._initPaths();
const { chromium } = require('playwright');
async function cap(page,out,name){
  await page.screenshot({path:`${out}/${name}.png`, fullPage:true});
  const text=await page.locator('body').innerText().catch(async()=>await page.textContent('body')) || '';
  await fs.writeFile(`${out}/${name}.txt`, text, 'utf8');
  const info=await page.evaluate(()=>({buttons:[...document.querySelectorAll('button')].map((b,i)=>({i,text:b.innerText,aria:b.getAttribute('aria-label'),id:b.id,disabled:b.disabled}))}));
  await fs.writeFile(`${out}/${name}.json`, JSON.stringify(info,null,2),'utf8');
  return text;
}
async function clickRun(page){
  const buttons=await page.locator('button').all();
  for(const b of buttons){ const txt=await b.innerText().catch(()=>''); const disabled=await b.isDisabled().catch(()=>true); if(!disabled && /Run Daily Scan|Refresh/i.test(txt)){ await b.click({force:true}); return txt; } }
  throw new Error('No enabled scan button');
}
(async()=>{
  const out=process.argv[2];
  const browser=await chromium.connectOverCDP('http://127.0.0.1:9224');
  const page=browser.contexts()[0].pages()[0];
  await page.locator('#nav-tab-daily').click({force:true});
  await page.waitForTimeout(1000);
  await cap(page,out,'cdp-27-daily-before-second-run');
  const clicked=await clickRun(page);
  let states=[];
  for(let i=0;i<30;i++){
    await page.waitForTimeout(5000);
    const text=await cap(page,out,`cdp-28-daily-second-run-${i}`);
    states.push({i, clicked, running:/Running|Checking|Scanning|Analyzing|Loading/i.test(text), top:text.split('\n').slice(0,35)});
    if(!/Running|Checking|Scanning|Analyzing|Loading/i.test(text) && i>2) break;
  }
  await fs.writeFile(`${out}/daily-second-run-wait-states.json`, JSON.stringify(states,null,2),'utf8');
  await cap(page,out,'cdp-29-daily-after-second-run');
  await page.locator('#nav-tab-queue').click({force:true});
  await cap(page,out,'cdp-30-story-queue-after-second-run');
  await browser.close();
})().catch(e=>{console.error(e);process.exit(1)});
