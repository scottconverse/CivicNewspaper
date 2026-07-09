const fs = require('node:fs/promises');
require('module').Module._initPaths();
const { chromium } = require('playwright');
async function cap(page,out,name){
  await page.waitForTimeout(1000);
  await page.screenshot({path:`${out}/${name}.png`, fullPage:true});
  const text=await page.locator('body').innerText().catch(async()=>await page.textContent('body')) || '';
  await fs.writeFile(`${out}/${name}.txt`, text, 'utf8');
  const info=await page.evaluate(()=>({buttons:[...document.querySelectorAll('button')].map((b,i)=>({i,text:b.innerText,aria:b.getAttribute('aria-label'),disabled:b.disabled})), inputs:[...document.querySelectorAll('input,textarea,select')].map((el,i)=>({i,tag:el.tagName,type:el.getAttribute('type'),value:el.value,placeholder:el.getAttribute('placeholder'),label:el.getAttribute('aria-label'),name:el.getAttribute('name')}))}));
  await fs.writeFile(`${out}/${name}.json`, JSON.stringify(info,null,2),'utf8');
  return {text, info};
}
async function clickByText(page,re){
  const els=await page.locator('button').all();
  for(const el of els){ const txt=await el.innerText().catch(()=>''); const disabled=await el.isDisabled().catch(()=>true); if(!disabled && re.test(txt)){ await el.click(); return txt; } }
  throw new Error('button not found '+re);
}
async function fillVisible(page, city, state){
  const inputs=await page.locator('input').all();
  for(const input of inputs){
    const ph=await input.getAttribute('placeholder').catch(()=>null);
    const label=await input.getAttribute('aria-label').catch(()=>null);
    const val=await input.inputValue().catch(()=>'');
    const hint=(ph||'')+' '+(label||'');
    if(/city|Longmont|Brighton/i.test(hint) || val==='Longmont') { await input.fill(city); }
    if(/state|CO|Colorado/i.test(hint) || val==='CO' || val==='Colorado') { await input.fill(state); }
  }
}
(async()=>{
  const out=process.argv[2];
  const browser=await chromium.connectOverCDP('http://127.0.0.1:9224');
  const page=browser.contexts()[0].pages()[0];
  await page.getByText('Sources', {exact:true}).first().click();
  await cap(page,out,'cdp-15-sources-before-discovery');
  async function run(label,state){
    try { await clickByText(page,/Discover for my city|Discover/i); } catch {}
    await page.waitForTimeout(500);
    await fillVisible(page,'Longmont',state);
    await cap(page,out,`cdp-16-discover-${label}-filled`);
    await clickByText(page,/Discover|Find sources|Search/i);
    for(let i=0;i<10;i++){
      await page.waitForTimeout(3000);
      const {text}=await cap(page,out,`cdp-17-discover-${label}-wait-${i}`);
      if(!/Discovering|Searching|Loading|Checking/i.test(text) && i>1) break;
    }
    await cap(page,out,`cdp-18-discover-${label}-results`);
  }
  await run('colorado','Colorado');
  await page.getByText('Sources', {exact:true}).first().click();
  await run('co','CO');
  await browser.close();
})().catch(e=>{console.error(e);process.exit(1)});
