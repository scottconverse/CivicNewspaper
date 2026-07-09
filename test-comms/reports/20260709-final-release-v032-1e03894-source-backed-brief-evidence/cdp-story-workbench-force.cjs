const fs = require('node:fs/promises');
require('module').Module._initPaths();
const { chromium } = require('playwright');
async function cap(page,out,name){
  await page.waitForTimeout(1000);
  await page.screenshot({path:`${out}/${name}.png`, fullPage:true});
  const text=await page.locator('body').innerText().catch(async()=>await page.textContent('body')) || '';
  await fs.writeFile(`${out}/${name}.txt`, text, 'utf8');
  const info=await page.evaluate(()=>({buttons:[...document.querySelectorAll('button')].map((b,i)=>({i,text:b.innerText,aria:b.getAttribute('aria-label'),id:b.id,disabled:b.disabled})), selects:[...document.querySelectorAll('select')].map((el,i)=>({i,value:el.value, options:[...el.options].map(o=>({text:o.text,value:o.value,selected:o.selected}))})), inputs:[...document.querySelectorAll('input,textarea')].map((el,i)=>({i,id:el.id,tag:el.tagName,type:el.getAttribute('type'),value:el.value,placeholder:el.getAttribute('placeholder'),label:el.getAttribute('aria-label'),name:el.getAttribute('name')}))}));
  await fs.writeFile(`${out}/${name}.json`, JSON.stringify(info,null,2),'utf8');
  return text;
}
(async()=>{
  const out=process.argv[2];
  const browser=await chromium.connectOverCDP('http://127.0.0.1:9224');
  const page=browser.contexts()[0].pages()[0];
  await page.keyboard.press('Escape').catch(()=>{});
  await page.waitForTimeout(500);
  const close=page.locator('#modal-discovery button').filter({hasText:'Close'}).first();
  if(await close.count()) await close.click({force:true}).catch(()=>{});
  await page.waitForTimeout(800);
  await page.locator('#nav-tab-queue').click({force:true});
  await cap(page,out,'cdp-24-story-queue');
  const openButtons=await page.locator('button').filter({hasText:/Open in Workbench/i}).all();
  if(openButtons.length){ await openButtons[0].click({force:true}); await page.waitForTimeout(1500); }
  await cap(page,out,'cdp-25-after-open-workbench-attempt');
  await page.locator('#nav-tab-workbench').click({force:true});
  await cap(page,out,'cdp-26-workbench');
  await browser.close();
})().catch(e=>{console.error(e);process.exit(1)});
