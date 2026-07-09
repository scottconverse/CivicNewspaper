const fs = require('node:fs/promises');
require('module').Module._initPaths();
const { chromium } = require('playwright');
async function cap(page,out,name){
  await page.waitForTimeout(1000);
  await page.screenshot({path:`${out}/${name}.png`, fullPage:true});
  const text=await page.locator('body').innerText().catch(async()=>await page.textContent('body')) || '';
  await fs.writeFile(`${out}/${name}.txt`, text, 'utf8');
  const info=await page.evaluate(()=>({buttons:[...document.querySelectorAll('button')].map((b,i)=>({i,text:b.innerText,aria:b.getAttribute('aria-label'),disabled:b.disabled})), inputs:[...document.querySelectorAll('input,textarea,select')].map((el,i)=>({i,tag:el.tagName,type:el.getAttribute('type'),value:el.value,placeholder:el.getAttribute('placeholder'),label:el.getAttribute('aria-label'),name:el.getAttribute('name')}))}));
  await fs.writeFile(`${out}/${name}.json`, JSON.stringify(info,null,2), 'utf8');
  return text;
}
async function clickText(page, re){
  const buttons = await page.locator('button').all();
  for (const b of buttons) {
    const txt = (await b.innerText().catch(()=>'')) || '';
    const disabled = await b.isDisabled().catch(()=>true);
    if (!disabled && re.test(txt)) { await b.click(); return txt; }
  }
  throw new Error('No enabled button matching '+re);
}
(async()=>{
  const out=process.argv[2];
  const browser=await chromium.connectOverCDP('http://127.0.0.1:9224');
  const page=browser.contexts()[0].pages()[0];
  let log=[];
  let text=await cap(page,out,'cdp-07-before-continue-without-ai');
  if (/Continue without AI/i.test(text)) { log.push('clicked:'+await clickText(page,/Continue without AI/i)); }
  for (let i=0;i<10;i++) {
    text=await cap(page,out,`cdp-08-onboard-finish-loop-${i}`);
    if (/Dashboard|Daily Scan|Story Queue|Workbench/i.test(text) && !/Workspace Setup/i.test(text)) break;
    let clicked=false;
    for (const re of [/Finish setup|Complete setup|Open dashboard|Start using|Next|Continue setup|Continue/i]) {
      try { log.push('clicked:'+await clickText(page,re)); clicked=true; break; } catch {}
    }
    if (!clicked) break;
  }
  await cap(page,out,'cdp-09-after-onboarding');
  await fs.writeFile(`${out}/onboarding-finish-actions.json`, JSON.stringify(log,null,2),'utf8');
  await browser.close();
})().catch(e=>{console.error(e);process.exit(1)});
