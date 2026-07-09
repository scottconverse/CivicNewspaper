const fs = require('node:fs/promises');
require('module').Module._initPaths();
const { chromium } = require('playwright');
async function cap(page,out,name){
  await page.waitForTimeout(1000);
  await page.screenshot({path:`${out}/${name}.png`, fullPage:true});
  const text=await page.locator('body').innerText().catch(async()=>await page.textContent('body')) || '';
  await fs.writeFile(`${out}/${name}.txt`, text, 'utf8');
  const info=await page.evaluate(()=>({buttons:[...document.querySelectorAll('button')].map((b,i)=>({i,text:b.innerText,aria:b.getAttribute('aria-label'),id:b.id,disabled:b.disabled})), inputs:[...document.querySelectorAll('input,textarea,select')].map((el,i)=>({i,id:el.id,tag:el.tagName,type:el.getAttribute('type'),value:el.value,placeholder:el.getAttribute('placeholder'),label:el.getAttribute('aria-label'),name:el.getAttribute('name')}))}));
  await fs.writeFile(`${out}/${name}.json`, JSON.stringify(info,null,2),'utf8');
  return text;
}
(async()=>{
  const out=process.argv[2];
  const browser=await chromium.connectOverCDP('http://127.0.0.1:9224');
  const page=browser.contexts()[0].pages()[0];
  await cap(page,out,'cdp-19-before-import-colorado-modal');
  const importBtn=page.locator('#btn-import-discovered');
  if(await importBtn.count()) { await importBtn.click({force:true}).catch(()=>{}); await page.waitForTimeout(1200); }
  const closeBtn=page.locator('#modal-discovery button').filter({hasText:'Close'}).first();
  if(await closeBtn.count()) { await closeBtn.click({force:true}).catch(()=>{}); await page.waitForTimeout(800); }
  await page.getByText('Sources', {exact:true}).first().click().catch(()=>{});
  await cap(page,out,'cdp-20-sources-after-colorado-import');
  await page.locator('#btn-trigger-discovery').click({force:true});
  await page.waitForTimeout(800);
  const city=page.locator('#modal-discovery input[placeholder^="City Name"]').first();
  const state=page.locator('#modal-discovery input[placeholder^="State"]').first();
  await city.fill('Longmont');
  await state.fill('CO');
  await cap(page,out,'cdp-21-discover-co-filled');
  await page.locator('#modal-discovery button').filter({hasText:/Auto-Find Feeds/}).click({force:true});
  for(let i=0;i<10;i++){
    await page.waitForTimeout(3000);
    const text=await cap(page,out,`cdp-22-discover-co-wait-${i}`);
    if(!/Searching|Discovering|Loading|Checking/i.test(text) && /Selected:|Import Checked Sources|Municipal Website/i.test(text) && i>1) break;
  }
  await cap(page,out,'cdp-23-discover-co-results');
  await browser.close();
})().catch(e=>{console.error(e);process.exit(1)});
