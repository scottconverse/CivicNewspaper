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
  let text=await cap(page,out,'cdp-04-before-ai-skip');
  log.push('before:'+text.split('\n').slice(0,3).join(' | '));
  if (/Skip for now/i.test(text)) { log.push('clicked:'+await clickText(page,/Skip for now/i)); await page.waitForTimeout(500); }
  for (let i=0;i<8;i++) {
    text=await cap(page,out,`cdp-05-onboard-loop-${i}`);
    if (/Dashboard|Daily Scan|Story Queue|Workbench/i.test(text) && !/Workspace Setup/i.test(text)) break;
    if (/Finish|Start|Open dashboard|Complete setup/i.test(text)) { log.push('clicked:'+await clickText(page,/Finish|Start|Open dashboard|Complete setup/i)); continue; }
    if (/Next|Continue setup|Continue/i.test(text)) { try { log.push('clicked:'+await clickText(page,/Next|Continue setup|Continue/i)); continue; } catch(e) { log.push('no-next:'+e.message); } }
    break;
  }
  text=await cap(page,out,'cdp-06-after-onboarding');
  await fs.writeFile(`${out}/onboarding-actions.json`, JSON.stringify(log,null,2),'utf8');
  await browser.close();
})().catch(e=>{console.error(e);process.exit(1)});
